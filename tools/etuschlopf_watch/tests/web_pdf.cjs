// Manual browser probe: embedded production HTML, synthetic PDF responses.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');
const source = process.argv[2] || path.join(__dirname, '../src/web_watch.rs');
const html = fs.readFileSync(source, 'utf8').match(/const WATCH_HTML: &str = r#"([\s\S]*?)"#;/)?.[1];
assert.ok(html);
function model(session = 'synthetic-session', projects = [['one', 'Synthetic Ω']]) {
  return { mqtt: 'live', current_session_id: session, sessions: [{ observation_session_id: session,
    sources: projects.map(([id, label]) => ({ source: { source_id: id, source_role: 'project',
      project_label: label, status: 'running', stage: 'project_arrangement_solver' },
      run_elapsed_millis: 1000, last_seen_age_millis: 0, last_state_change_age_millis: 0 })) }] };
}
(async () => {
  const browser = await chromium.launch({ headless: true });
  const errors = []; let passed = 0;
  async function fixture(initial = model()) {
    const p = await browser.newPage(); p.on('pageerror', e => errors.push(e.message));
    await p.addInitScript(() => {
      window.nativeFetch = window.fetch.bind(window); window.pdfRequests = [];
      // Deliberately allows late resolution after abort to test the result guard.
      window.fetch = (url, options) => new Promise((resolve, reject) => window.pdfRequests.push({ url, options, resolve, reject }));
      window.EventSource = class { constructor() { this.listeners = {}; window.watchEvents = this; }
        addEventListener(t, f) { this.listeners[t] = f; } };
    });
    await p.route('http://watch.test/**', r => r.fulfill({ contentType: 'text/html', body: html }));
    await p.goto('http://watch.test/'); await send(p, initial); return p;
  }
  async function send(p, m) { await p.evaluate(m => window.watchEvents.listeners.state({ data: JSON.stringify(m) }), m); }
  async function count(p) { return p.evaluate(() => window.pdfRequests.length); }
  async function settle(p, index, options = {}) {
    await p.evaluate(({ index, options }) => {
      const r = window.pdfRequests[index];
      if (options.networkError) r.reject(new Error('synthetic disconnect'));
      else r.resolve(new Response(options.body ?? '%PDF-1.7\nsynthetic PDF\n%%EOF\n', {
        status: options.status || 200, headers: { 'content-type': options.type || 'application/pdf' },
      }));
    }, { index, options });
    await p.evaluate(() => new Promise(r => setTimeout(r, 0)));
  }
  const title = p => p.locator('.title .pdf-link');
  async function ready(p) { await p.waitForFunction(() => document.querySelector('.pdf-link')?.dataset.pdfState === 'ready'); }
  async function failed(p) { await p.waitForFunction(() => document.querySelector('.pdf-link')?.dataset.pdfState === 'error'); }
  async function bytes(p, href) { return p.evaluate(async href => (await window.nativeFetch(href)).text(), href); }
  async function check(name, fn) { await fn(); passed++; console.log('PASS ' + name); }
  try {
    await check('early loading is single flight and the ready PDF remains a native link', async () => {
      const p = await fixture(); assert.equal(await count(p), 1);
      assert.equal(await title(p).getAttribute('aria-busy'), 'true');
      const url = await p.evaluate(() => window.pdfRequests[0].url);
      assert.equal(url, '/pdf-data/synthetic-session/Synthetic%20%CE%A9');
      await settle(p, 0); await ready(p);
      assert.equal(await title(p).getAttribute('role'), null);
      assert.equal(await title(p).getAttribute('target'), '_blank');
      assert.equal(await title(p).getAttribute('rel'), 'noopener');
      const hrefs = await p.locator('.pdf-link').evaluateAll(xs => xs.map(x => x.href));
      assert.equal(new Set(hrefs).size, 1); assert.ok(hrefs[0].startsWith('blob:'));
      assert.match(await bytes(p, hrefs[0]), /synthetic PDF/);
      const popupPromise = p.context().waitForEvent('page');
      await title(p).click({ modifiers: ['Control'] });
      const popup = await popupPromise; await popup.waitForLoadState('domcontentloaded').catch(() => {});
      // Headless Shell may delegate PDFs to its viewer/download handling.
      // Assert native tab creation; blob identity and bytes are checked above.
      assert.notEqual(popup, p); await popup.close(); await p.close();
    });
    await check('failure is visible and snapshots cannot silently retry', async () => {
      const p = await fixture(); await settle(p, 0, { status: 404 }); await failed(p);
      assert.equal(await title(p).getAttribute('role'), 'button');
      assert.match(await title(p).textContent(), /PDF erneut laden/);
      for (let i = 0; i < 4; i++) { const m = model(); m.sessions[0].sources[0].run_elapsed_millis += i; await send(p, m); }
      assert.equal(await count(p), 1); await p.close();
    });
    await check('keyboard retry is single flight and keeps its focused DOM element', async () => {
      const p = await fixture(); await settle(p, 0, { status: 404 }); await failed(p);
      await title(p).focus(); await p.evaluate(() => { window.savedPdfControl = document.activeElement; });
      await p.keyboard.press('Space'); assert.equal(await count(p), 2);
      await p.keyboard.press('Enter'); await p.keyboard.press('Space'); assert.equal(await count(p), 2);
      await settle(p, 1); await ready(p);
      assert.equal(await p.evaluate(() => document.activeElement === window.savedPdfControl), true);
      assert.equal(await title(p).getAttribute('role'), null); await p.close();
    });
    await check('late responses cannot steal focus after the user moved elsewhere', async () => {
      const p = await fixture(); await settle(p, 0, { status: 404 }); await failed(p);
      await title(p).press('Enter'); assert.equal(await count(p), 2);
      await p.evaluate(() => { const b = document.createElement('button'); b.id = 'outside'; document.body.append(b); b.focus(); });
      await settle(p, 1); await ready(p);
      assert.equal(await p.evaluate(() => document.activeElement.id), 'outside'); await p.close();
    });
    await check('a new session cannot reuse an old blob for the same project name', async () => {
      const p = await fixture(model('old-session')); await settle(p, 0, { body: '%PDF-1.7\nOLD\n%%EOF\n' }); await ready(p);
      const old = await title(p).getAttribute('href'); await send(p, model('new-session'));
      assert.equal(await count(p), 2); assert.equal(await title(p).getAttribute('href'), '#');
      await settle(p, 1, { body: '%PDF-1.7\nNEW\n%%EOF\n' }); await ready(p);
      assert.match(await bytes(p, await title(p).getAttribute('href')), /NEW/);
      assert.match(await bytes(p, old), /OLD/); await p.close();
    });
    await check('an aborted old response cannot overwrite the new session', async () => {
      const p = await fixture(model('old-session')); await send(p, model('new-session'));
      assert.equal(await count(p), 2);
      assert.equal(await p.evaluate(() => window.pdfRequests[0].options.signal.aborted), true);
      await settle(p, 0, { body: '%PDF-1.7\nOLD\n%%EOF\n' });
      assert.equal(await title(p).getAttribute('href'), '#');
      await settle(p, 1, { body: '%PDF-1.7\nNEW\n%%EOF\n' }); await ready(p);
      assert.match(await bytes(p, await title(p).getAttribute('href')), /NEW/); await p.close();
    });
    await check('equal labels share the PDF without merging source focus', async () => {
      const p = await fixture(model('duplicate', [['one', 'Same Ω'], ['two', 'Same Ω']]));
      assert.equal(await count(p), 1); const second = p.locator('.projectrow .pdf-link').nth(1);
      await settle(p, 0, { status: 404 }); await failed(p); await second.focus();
      await p.keyboard.press('Space'); assert.equal(await count(p), 2);
      assert.equal(await p.locator('details.projectrow[open]').count(), 0);
      await settle(p, 1); await ready(p);
      assert.equal(await second.evaluate(x => x === document.activeElement), true); await p.close();
    });
    await check('network errors, wrong content type and empty bodies remain retryable', async () => {
      for (const options of [{ networkError: true }, { type: 'text/html' }, { body: '' }]) {
        const p = await fixture(); await settle(p, 0, options); await failed(p);
        await title(p).press('Enter'); assert.equal(await count(p), 2);
        await settle(p, 1); await ready(p); await p.close();
      }
    });
    await check('independent resources keep separate outcomes under duplication and reordering', async () => {
      const m = model('independent', [['one', 'First Ω'], ['two', 'Second Ü']]);
      const p = await fixture(m); assert.equal(await count(p), 2);
      await settle(p, 0); await settle(p, 1, { status: 404 });
      const first = p.locator('[data-source-id="one"] .pdf-link');
      const second = p.locator('[data-source-id="two"] .pdf-link');
      const href = await first.getAttribute('href'); assert.ok(href.startsWith('blob:'));
      assert.equal(await second.getAttribute('role'), 'button');
      m.sessions[0].sources.reverse(); await send(p, m); assert.equal(await count(p), 2);
      assert.equal(await first.getAttribute('href'), href);
      await second.press('Enter'); assert.equal(await count(p), 3);
      await settle(p, 2); await ready(p);
      assert.equal(await first.getAttribute('href'), href);
      assert.equal(await second.getAttribute('role'), null); await p.close();
    });
    assert.deepEqual(errors, []); console.log(`${passed} PDF browser scenarios passed`);
  } finally { await browser.close(); }
})().catch(e => { console.error(e); process.exitCode = 1; });
