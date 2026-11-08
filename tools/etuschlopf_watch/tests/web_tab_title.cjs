// Manual browser regression: Node.js + externally installed Playwright/Chromium.
// Uses the embedded production page and synthetic observation messages only.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');

const source = process.argv[2] || path.join(__dirname, '../src/web_watch.rs');
const html = fs.readFileSync(source, 'utf8').match(/const WATCH_HTML: &str = r#"([\s\S]*?)"#;/)?.[1];
assert.ok(html, 'embedded production HTML must be available');
function session(id, status, outcome) {
  return { observation_session_id: id, sources: [{
    source: { source_id: 'launcher', source_role: 'launcher', status,
      stage: 'launcher_finalizing', outcome_color: outcome },
    run_elapsed_millis: 1000, last_state_change_age_millis: 0, last_seen_age_millis: 0,
  }] };
}
function model(status, outcome) {
  return { mqtt: 'live', current_session_id: 'current', sessions: [session('current', status, outcome)] };
}
function corpusModel(completed, total, status = 'running', outcome) {
  const value = model(status, outcome);
  value.sessions[0].sources.push({
    source: { source_id: 'corpus', source_role: 'corpus', status,
      stage: 'corpus_running_projects', progress: { completed, total, active: 0, failed: 0 } },
    run_elapsed_millis: 1000, last_state_change_age_millis: 0, last_seen_age_millis: 0,
  });
  return value;
}

(async () => {
  const browser = await chromium.launch({ headless: true });
  const errors = [];
  let passed = 0;
  try {
    const page = await browser.newPage();
    page.on('pageerror', error => errors.push(error.message));
    await page.addInitScript(() => {
      window.EventSource = class {
        static CLOSED = 2;
        close() { this.readyState = 2; }
        constructor() { this.listeners = {}; window.watchEvents = this; }
        addEventListener(type, fn) { this.listeners[type] = fn; }
      };
    });
    await page.route('http://watch.test/**', route => route.fulfill({ contentType: 'text/html', body: html }));
    await page.goto('http://watch.test/');
    async function send(value) {
      await page.evaluate(value => window.watchEvents.listeners.state({ data: JSON.stringify(value) }), value);
    }
    async function title(label, count = '') {
      assert.equal(await page.title(), `${label}${count ? ' · ' + count : ''} · EtuSchlopf Watch`);
      assert.equal(await page.locator('#runStatusLabel').textContent(), label);
    }
    async function check(name, run) {
      await run();
      passed++;
      console.log('PASS ' + name);
    }
    await check('waiting is visible before the first observation', async () => {
      await title('WAITING');
    });
    await check('running updates the browser tab and repeated ticks keep a stable title', async () => {
      await send(model('running'));
      await title('RUNNING');
      await page.evaluate(() => { tick(); tick(); });
      await title('RUNNING');
    });
    await check('unchanged observations and the real timer leave the title DOM untouched', async () => {
      await page.evaluate(() => {
        window.titleMutations = [];
        window.titleObserver = new MutationObserver(records => window.titleMutations.push(...records));
        window.titleObserver.observe(document.querySelector('title'), { childList: true, characterData: true, subtree: true });
      });
      try {
        const unchanged = model('running');
        unchanged.sessions[0].sources[0].run_elapsed_millis = 8000;
        await send(unchanged);
        // Exercise the production interval, not just a direct call to tick().
        await page.waitForTimeout(1200);
        await title('RUNNING');
        assert.equal(await page.evaluate(() => window.titleMutations.length), 0,
          'clock updates and repeated snapshots must not rewrite an unchanged tab title');
        await send(model('completed'));
        await title('COMPLETED');
        assert.ok(await page.evaluate(() => window.titleMutations.length > 0),
          'an actual status change must still reach the browser title');
      } finally {
        await page.evaluate(() => window.titleObserver.disconnect());
      }
    });
    await check('completion, failure and cancellation replace the running title', async () => {
      for (const [status, label] of [['completed', 'COMPLETED'], ['failed', 'FAILED'], ['cancelled', 'CANCELLED']]) {
        await send(model('running'));
        await send(model(status));
        await title(label);
      }
    });
    await check('the reported terminal outcome stays visible instead of generic completion', async () => {
      for (const [outcome, label] of [['dark_green', 'DUNKELGRÜN'], ['light_green', 'HELLGRÜN'], ['yellow', 'GELB'], ['red', 'ROT']]) {
        await send(model('completed', outcome));
        await title(label);
      }
    });
    await check('a running observation does not become green from a premature outcome', async () => {
      await send(model('running', 'dark_green'));
      await title('RUNNING');
    });
    await check('only the current session supplies the title and a missing session resets it', async () => {
      const next = model('running');
      next.sessions.unshift(session('old', 'failed'));
      await send(next);
      await title('RUNNING');
      next.current_session_id = 'not-observed';
      await send(next);
      await title('WAITING');
    });
    await check('transport closure preserves the last reported result', async () => {
      await send(model('completed', 'yellow'));
      await page.evaluate(() => { window.watchEvents.onerror(); tick(); });
      await title('GELB');
      assert.equal(await page.locator('#transport').getAttribute('data-state'), 'closed');
    });
    await check('reported corpus counts advance without changing the status and survive completion', async () => {
      await send(corpusModel(0, 7));
      await title('RUNNING', '0/7');
      await send(corpusModel(3, 7));
      await title('RUNNING', '3/7');
      await send(corpusModel(7, 7, 'completed', 'yellow'));
      await title('GELB', '7/7');
      await page.evaluate(() => { window.watchEvents.onerror(); tick(); });
      await title('GELB', '7/7');
    });
    await check('unreported totals and previous sessions cannot supply an invented count', async () => {
      const next = model('running');
      next.sessions.unshift(corpusModel(6, 9).sessions[0]);
      next.sessions[0].observation_session_id = 'old-corpus';
      await send(next);
      await title('RUNNING');
      await send(corpusModel(0, 0));
      await title('RUNNING');
      const noProgress = corpusModel(2, 7);
      delete noProgress.sessions[0].sources[1].source.progress;
      await send(noProgress);
      await title('RUNNING');
      noProgress.current_session_id = 'not-observed';
      await send(noProgress);
      await title('WAITING');
    });
    assert.deepEqual(errors, [], 'production page must not raise script errors');
    console.log(`${passed} browser scenarios passed`);
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
