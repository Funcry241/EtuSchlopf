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
      await title('Warte auf Laufdaten');
    });
    await check('running updates the browser tab and repeated ticks keep a stable title', async () => {
      await send(model('running'));
      await title('Läuft');
      await page.evaluate(() => { tick(); tick(); });
      await title('Läuft');
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
        await title('Läuft');
        assert.equal(await page.evaluate(() => window.titleMutations.length), 0,
          'clock updates and repeated snapshots must not rewrite an unchanged tab title');
        await send(model('completed'));
        await title('Abgeschlossen');
        assert.ok(await page.evaluate(() => window.titleMutations.length > 0),
          'an actual status change must still reach the browser title');
      } finally {
        await page.evaluate(() => window.titleObserver.disconnect());
      }
    });
    await check('completion, failure and cancellation replace the running title', async () => {
      for (const [status, label] of [['completed', 'Abgeschlossen'], ['failed', 'Fehlgeschlagen'], ['cancelled', 'Abgebrochen']]) {
        await send(model('running'));
        await send(model(status));
        await title(label);
      }
    });
    await check('the reported terminal outcome stays visible instead of generic completion', async () => {
      for (const [outcome, label] of [['dark_green', 'Abgeschlossen'], ['light_green', 'Abgeschlossen · Hinweis'], ['yellow', 'Abgeschlossen · Freigabe prüfen'], ['red', 'Abgeschlossen · Fehler gemeldet']]) {
        await send(model('completed', outcome));
        await title(label);
        assert.equal(await page.locator('#runStatus').getAttribute('data-state'), outcome);
      }
    });
    await check('a running observation does not become green from a premature outcome', async () => {
      await send(model('running', 'dark_green'));
      await title('Läuft');
    });
    await check('only the current session supplies the title and a missing session resets it', async () => {
      const next = model('running');
      next.sessions.unshift(session('old', 'failed'));
      await send(next);
      await title('Läuft');
      next.current_session_id = 'not-observed';
      await send(next);
      await title('Warte auf Laufdaten');
    });
    await check('transport closure preserves the last reported result', async () => {
      await send(model('completed', 'yellow'));
      await page.evaluate(() => { window.watchEvents.onerror(); tick(); });
      await title('Abgeschlossen · Freigabe prüfen');
      assert.equal(await page.locator('#transport').getAttribute('data-state'), 'closed');
    });
    await check('reported corpus counts advance without changing the status and survive completion', async () => {
      await send(corpusModel(0, 7));
      await title('Läuft', '0/7');
      await send(corpusModel(3, 7));
      await title('Läuft', '3/7');
      await send(corpusModel(7, 7, 'completed', 'yellow'));
      await title('Abgeschlossen · Freigabe prüfen', '7/7');
      await page.evaluate(() => { window.watchEvents.onerror(); tick(); });
      await title('Abgeschlossen · Freigabe prüfen', '7/7');
    });
    await check('unreported totals and previous sessions cannot supply an invented count', async () => {
      const next = model('running');
      next.sessions.unshift(corpusModel(6, 9).sessions[0]);
      next.sessions[0].observation_session_id = 'old-corpus';
      await send(next);
      await title('Läuft');
      await send(corpusModel(0, 0));
      await title('Läuft');
      const noProgress = corpusModel(2, 7);
      delete noProgress.sessions[0].sources[1].source.progress;
      await send(noProgress);
      await title('Läuft');
      noProgress.current_session_id = 'not-observed';
      await send(noProgress);
      await title('Warte auf Laufdaten');
    });
    await check('the last project waits for the launcher outcome and malformed updates leave the view intact', async () => {
      const value = model('completed');
      value.sessions[0].sources[0].source = {
        source_id: 'p', source_role: 'project', status: 'completed',
        project_label: 'Probe', stage: 'project_arrangement_solver',
      };
      await send(value);
      await title('Warte auf Laufergebnis');
      assert.equal(await page.locator('.phase.done').count(), 2);
      await send({ current_session_id: 'broken', sessions: [{ sources: [null] }] });
      await title('Warte auf Laufergebnis');
      assert.equal(await page.evaluate(() => formatMillis(3661000)), '1:01:01');
    });
    await check('a full project bar explicitly announces finalization', async () => {
      const value = corpusModel(7, 7);
      value.sessions[0].sources[1].source.stage = 'corpus_finalizing_corpus_report';
      await send(value);
      assert.equal(await page.getByRole('progressbar').getAttribute('aria-valuenow'), '7');
      assert.match(await page.locator('.progressnote').textContent(), /Korpusbericht wird erstellt/);
      await page.setViewportSize({ width: 761, height: 640 });
      value.sessions[0].sources.push({
        source: { source_id: 'p', source_role: 'project', status: 'failed',
          stage: 'project_initial_render_transaction' },
      });
      await send(value);
      assert.equal(await page.locator('.projectcolumns').evaluate(el => getComputedStyle(el).display), 'none');
      assert.match(await page.locator('.projectstate').textContent(), /Letzte Phase Initial Render/);
      assert.equal(await page.locator('.projectrow .pdf-link').count(), 0);
    });
    await check('a failure before the first project does not announce preparation', async () => {
      const value = corpusModel(0, 7, 'failed', 'red');
      await send(value);
      assert.match(await page.locator('.projectwaiting').textContent(), /vor dem ersten Projekt fehlgeschlagen/);
    });
    await check('an empty current session uses the waiting view without stale-data age', async () => {
      const value = model('running');
      value.sessions[0].sources = [];
      value.mqtt = 'reconnecting';
      await send(value);
      await title('Warte auf Laufdaten');
      assert.match(await page.locator('#shell .empty').textContent(), /Warte auf Laufdaten/);
      assert.doesNotMatch(await page.locator('#transport').textContent(), /Stand von vor/);
    });
    await check('a project-only overview never borrows a project duration for the whole run', async () => {
      const value = model('running');
      const launcher = value.sessions[0].sources[0];
      value.sessions[0].sources = ['a', 'b'].map(id => ({
        source: { source_id: id, source_role: 'project', status: 'completed',
          stage: 'project_release_report_and_exit' },
        run_elapsed_millis: 12000,
      }));
      await send(value);
      assert.equal(await page.locator('.hero .title').textContent(), 'Projektübersicht');
      assert.equal(await page.locator('.herometric').count(), 0);
      assert.equal(await page.locator('.projectage').count(), 2);
      launcher.source.status = 'completed';
      value.sessions[0].sources.push(launcher);
      await send(value);
      assert.match(await page.locator('.herometric').textContent(), /^Laufdauer/);
    });
    assert.deepEqual(errors, [], 'production page must not raise script errors');
    console.log(`${passed} browser scenarios passed`);
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
