// Manual browser regression: Node.js + externally installed Playwright/Chromium.
// Exercises the embedded production page with synthetic observation messages.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');

const source = process.argv[2] || path.join(__dirname, '../src/web_watch.rs');
const rust = fs.readFileSync(source, 'utf8');
const html = rust.match(/const WATCH_HTML: &str = r#"([\s\S]*?)"#;/)?.[1];
assert.ok(html, 'embedded production HTML must be available');

const project = (id, label = id) => ({
  source: { source_id: id, source_role: 'project', project_label: label,
    status: 'running', stage: 'project_arrangement_solver', activity: null },
  run_elapsed_millis: 1000, last_state_change_age_millis: 1000,
  last_seen_age_millis: 0,
});
function model(projects, session = 'synthetic-session', corpus = true) {
  return { mqtt: 'live', current_session_id: session, sessions: [{
    observation_session_id: session,
    sources: corpus ? [{ source: { source_id: 'corpus', source_role: 'corpus',
      status: 'running', stage: 'corpus_running_projects' } }, ...projects] : projects,
  }] };
}
function advance(m) {
  const next = structuredClone(m);
  for (const o of next.sessions[0].sources) {
    if (o.source.source_role === 'project') o.source.stage = 'project_master_notation_and_salience';
  }
  return next;
}

(async () => {
  const browser = await chromium.launch({ headless: true });
  const errors = [];
  let passed = 0;
  async function fixture(initial) {
    const page = await browser.newPage();
    page.on('pageerror', error => errors.push(error.message));
    await page.addInitScript(() => {
      window.EventSource = class {
        constructor() { this.listeners = {}; window.watchEvents = this; }
        addEventListener(type, fn) { this.listeners[type] = fn; }
      };
    });
    await page.route('http://watch.test/**', route => {
      if (new URL(route.request().url()).pathname === '/') {
        return route.fulfill({ contentType: 'text/html', body: html });
      }
      return route.fulfill({ status: 404, body: '' });
    });
    await page.goto('http://watch.test/');
    await send(page, initial);
    return page;
  }
  async function send(page, value) {
    await page.evaluate(value => window.watchEvents.listeners.state({ data: JSON.stringify(value) }), value);
  }
  async function focused(page, locator, message) {
    assert.equal(await locator.evaluate(el => el === document.activeElement), true, message);
  }
  // Attribute values are deliberately compared as data, not interpolated CSS.
  async function rowIndex(page, id) {
    const ids = await page.locator('details.projectrow').evaluateAll(rows => rows.map(r => r.dataset.sourceId));
    const index = ids.indexOf(id);
    assert.ok(index >= 0, 'synthetic project must be present');
    return page.locator('details.projectrow').nth(index);
  }
  async function check(name, run) {
    await run();
    passed++;
    console.log('PASS ' + name);
  }
  try {
    await check('project summary survives status changes and keyboard navigation continues', async () => {
      const m = model([project('a'), project('b')]);
      const p = await fixture(m);
      const summary = p.locator('details.projectrow > summary').first();
      await summary.focus();
      await send(p, advance(m));
      await focused(p, summary, 'status update must preserve summary focus');
      await p.keyboard.press('Tab');
      await focused(p, p.locator('details.projectrow .pdf-link').first(), 'Tab must continue to its PDF link');
      await p.keyboard.press('Shift+Tab');
      await p.keyboard.press('Space');
      assert.equal(await p.locator('details.projectrow').first().evaluate(el => el.open), true);
      await p.close();
    });
    await check('PDF focus follows source identity through insertion, renaming and reordering', async () => {
      const id = 'source-"[\\] & <β>';
      const m = model([project('other', 'Beta'), project(id, 'Same & <name>')]);
      const p = await fixture(m);
      await (await rowIndex(p, id)).locator('.pdf-link').focus();
      const next = advance(m);
      next.sessions[0].sources.reverse();
      next.sessions[0].sources.push(project('new', 'A first'));
      next.sessions[0].sources.find(o => o.source.source_id === id).source.project_label = 'A renamed';
      await send(p, next);
      await focused(p, (await rowIndex(p, id)).locator('.pdf-link'), 'identity must survive changing list positions and labels');
      await p.close();
    });
    await check('all expanded rows and technical details survive refresh', async () => {
      const m = model([project('a'), project('b')]);
      const p = await fixture(m);
      await p.locator('details > summary').evaluateAll(nodes => nodes.forEach(el => { el.parentElement.open = true; }));
      await p.locator('details.detailsbox > summary').focus();
      await send(p, advance(m));
      assert.equal(await p.locator('details[open]').count(), 3);
      await focused(p, p.locator('details.detailsbox > summary'), 'technical details summary must keep focus');
      await p.close();
    });
    await check('single-project title and details PDF keep separate focus identities', async () => {
      const m = model([project('solo', 'Shared title')], 'single', false);
      const p = await fixture(m);
      await p.locator('.title .pdf-link').focus();
      await send(p, advance(m));
      await focused(p, p.locator('.title .pdf-link'), 'title link must retain focus');
      await p.locator('details.detailsbox > summary').press('Space');
      await p.locator('.card .pdf-link').focus();
      await send(p, m);
      await focused(p, p.locator('.card .pdf-link'), 'details link must not restore focus to the identically named title');
      assert.equal(await p.locator('details.detailsbox').evaluate(el => el.open), true);
      await p.close();
    });
    await check('duplicate project labels do not merge focused sources', async () => {
      const m = model([project('one', 'Duplicate'), project('two', 'Duplicate')]);
      const p = await fixture(m);
      await (await rowIndex(p, 'two')).locator('.pdf-link').focus();
      await send(p, advance(m));
      await focused(p, (await rowIndex(p, 'two')).locator('.pdf-link'), 'equal labels must keep distinct source identities');
      await p.close();
    });
    await check('a new session reusing source IDs inherits neither focus nor expanded details', async () => {
      const m = model([project('a'), project('b')]);
      const p = await fixture(m);
      await p.locator('details > summary').evaluateAll(nodes => nodes.forEach(el => { el.parentElement.open = true; }));
      await p.locator('details.projectrow > summary').first().focus();
      await send(p, model([project('a'), project('b')], 'another-session'));
      assert.equal(await p.locator('details[open]').count(), 0, 'expansion belongs to its original session');
      assert.equal(await p.evaluate(() => document.activeElement.tagName), 'BODY');
      await p.close();
    });
    await check('removed project or layout role does not focus another control', async () => {
      const m = model([project('a'), project('b')]);
      const p = await fixture(m);
      await (await rowIndex(p, 'b')).locator('.pdf-link').focus();
      await send(p, model([project('a')]));
      assert.equal(await p.evaluate(() => document.activeElement.tagName), 'BODY');
      await send(p, model([project('a')], 'single', false));
      await p.locator('.title .pdf-link').focus();
      await send(p, model([project('a'), project('b')], 'single'));
      assert.equal(await p.evaluate(() => document.activeElement.tagName), 'BODY');
      await p.close();
    });
    await check('refresh does not steal outside focus and tick does not replace focused elements', async () => {
      const m = model([project('a'), project('b')]);
      const p = await fixture(m);
      await p.evaluate(() => { const b = document.createElement('button'); b.id = 'outside'; document.body.append(b); b.focus(); });
      await send(p, advance(m));
      await focused(p, p.locator('#outside'), 'outside focus must remain untouched');
      await p.locator('details.projectrow > summary').first().focus();
      await p.evaluate(() => { window.savedFocus = document.activeElement; });
      const tick = advance(m);
      tick.sessions[0].sources[1].run_elapsed_millis += 1000;
      await send(p, tick);
      assert.equal(await p.evaluate(() => window.savedFocus === document.activeElement), true);
      await p.close();
    });
    await check('focus restoration does not scroll the viewport', async () => {
      const m = model(Array.from({ length: 40 }, (_, i) => project(`item-${i}`)));
      const p = await fixture(m);
      await (await rowIndex(p, 'item-39')).locator('.pdf-link').focus();
      await p.evaluate(() => window.scrollTo(0, 0));
      await send(p, advance(m));
      await focused(p, (await rowIndex(p, 'item-39')).locator('.pdf-link'), 'focus must remain on its original project');
      assert.equal(await p.evaluate(() => window.scrollY), 0, 'restoration must not scroll to the focused project');
      await p.close();
    });
    assert.deepEqual(errors, [], 'production page must not raise script errors');
    console.log(`${passed} browser scenarios passed`);
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
