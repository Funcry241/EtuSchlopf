// Real browser EventSource over a local HTTP/SSE fixture; production embedded UI.
const assert = require('node:assert/strict');
const fs = require('node:fs'); const path = require('node:path'); const http = require('node:http');
const { chromium } = require('playwright');
const source = process.argv[2] || path.join(__dirname, '../src/web_watch.rs');
const html = fs.readFileSync(source, 'utf8').match(/const WATCH_HTML: &str = r#"([\s\S]*?)"#;/)?.[1];
assert.ok(html);
const PDF = '%PDF-1.7\nsynthetic network PDF\n%%EOF\n';
function observation(id, role, status) {
  return { source: { source_id: id, source_role: role, project_label: role === 'project' ? id : null,
    status, stage: role === 'launcher' ? 'launcher_running_target' : 'project_arrangement_solver' },
    run_elapsed_millis: 1000, last_seen_age_millis: 0, last_state_change_age_millis: 0 };
}
function model() {
  return { mqtt: 'live', current_session_id: 'synthetic-network', sessions: [{
    observation_session_id: 'synthetic-network', sources: [observation('launcher', 'launcher', 'running'), observation('Synthetic', 'project', 'running')],
  }] };
}
(async () => {
  const browser = await chromium.launch({ headless: true }); const errors = []; let passed = 0;
  async function fixture(initial = model(), options = {}) {
    let state = initial, connections = 0, pdfRequests = 0;
    const streams = []; const held = [];
    const server = http.createServer((req, res) => {
      if (req.url === '/') { res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' }); res.end(html); }
      else if (req.url === '/events') {
        connections++;
        if (options.denyReconnect && connections > 1) { res.writeHead(204); res.end(); return; }
        streams.push(res); res.writeHead(200, { 'content-type': 'text/event-stream', 'cache-control': 'no-store' });
        res.write('retry: 80\n\nevent: state\ndata: ' + JSON.stringify(state) + '\n\n');
      } else if (req.url.startsWith('/pdf-data/')) {
        pdfRequests++;
        if (options.holdPdf) held.push(res);
        else if (options.pdf) { res.writeHead(200, { 'content-type': 'application/pdf' }); res.end(PDF); }
        else { res.writeHead(404); res.end('missing'); }
      } else { res.writeHead(404); res.end(); }
    });
    await new Promise(r => server.listen(0, '127.0.0.1', r));
    const p = await browser.newPage(); p.setDefaultTimeout(5000); p.on('pageerror', e => errors.push(e.message));
    await p.goto('http://127.0.0.1:' + server.address().port + '/');
    await p.waitForFunction(() => events.readyState === EventSource.OPEN);
    let stopped = false;
    async function stop() { if (stopped) return; stopped = true; server.closeAllConnections(); await new Promise(r => server.close(r)); }
    return { p, streams, held, connections: () => connections, pdfRequests: () => pdfRequests,
      send(next) { state = next; for (const r of streams) if (!r.destroyed && !r.writableEnded) r.write('event: state\ndata: ' + JSON.stringify(state) + '\n\n'); },
      stop, async close() { await p.close(); await stop(); } };
  }
  async function transport(p, value) { await p.waitForFunction(v => document.querySelector('#transport').dataset.state === v, value); }
  async function terminal(f, status = 'completed', color = 'yellow', child = 'completed') {
    const m = model(); m.sessions[0].sources[0].source.status = status;
    m.sessions[0].sources[0].source.outcome_color = color;
    m.sessions[0].sources[1].source.status = child; f.send(m);
    await transport(f.p, 'closed');
    assert.equal(await f.p.evaluate(() => events.readyState), 2);
  }
  async function check(name, fn) { await fn(); passed++; console.log('PASS ' + name); }
  try {
    await check('mixed project failure reconnects through a real second event stream', async () => {
      const m = model(); m.sessions[0].sources.push(observation('Synthetic Failed', 'project', 'failed'));
      const f = await fixture(m);
      try {
        await f.p.waitForFunction(() => document.querySelector('#runStatusLabel').textContent === 'FAILED');
        f.streams[0].end(); await transport(f.p, 'reconnecting');
        assert.equal(await f.p.evaluate(() => events.readyState), 0);
        await transport(f.p, 'live'); assert.ok(f.connections() >= 2);
        assert.equal(await f.p.locator('.projectrow.running').count(), 1);
        assert.equal(await f.p.locator('#runStatusLabel').textContent(), 'FAILED');
      } finally { await f.close(); }
    });
    await check('an empty observation stays waiting and reconnects', async () => {
      const m = model(); m.sessions[0].sources = []; const f = await fixture(m);
      try { assert.equal(await f.p.locator('#runStatusLabel').textContent(), 'WAITING');
        f.streams[0].end(); await transport(f.p, 'reconnecting'); await transport(f.p, 'live');
        assert.equal(await f.p.locator('#runStatusLabel').textContent(), 'WAITING');
      } finally { await f.close(); }
    });
    await check('MQTT loss is observed while the browser stream stays open', async () => {
      const f = await fixture();
      try { const m = model(); m.mqtt = 'reconnecting'; f.send(m); await transport(f.p, 'reconnecting');
        assert.equal(await f.p.evaluate(() => events.readyState), 1);
        f.send(model()); await transport(f.p, 'live');
      } finally { await f.close(); }
    });
    await check('project completion waits for the actual launcher terminal outcome', async () => {
      for (const [status, color, label] of [['completed', 'yellow', 'GELB'], ['failed', 'red', 'ROT'], ['cancelled', 'yellow', 'GELB']]) {
        for (const child of ['completed', 'running']) {
        const m = model(); m.sessions[0].sources[1].source.status = child; const f = await fixture(m);
        try { await transport(f.p, 'live'); assert.equal(await f.p.evaluate(() => events.readyState), 1);
          await terminal(f, status, color, child);
          const clocks = await f.p.locator('.clock[data-age-kind="run"]').allTextContents();
          await f.p.waitForTimeout(child === 'running' ? 1100 : 200);
          assert.deepEqual(await f.p.locator('.clock[data-age-kind="run"]').allTextContents(), clocks);
          assert.equal(f.connections(), 1); assert.equal(await f.p.locator('#runStatusLabel').textContent(), label);
        } finally { await f.close(); }
        }
      }
    });
    await check('a cached PDF remains readable after terminal state and actual server shutdown', async () => {
      const f = await fixture(model(), { pdf: true });
      try {
        await f.p.waitForFunction(() => document.querySelector('.pdf-link')?.href.startsWith('blob:'));
        const href = await f.p.locator('.title .pdf-link').getAttribute('href');
        await terminal(f); await f.stop();
        assert.equal(await f.p.locator('.title .pdf-link').getAttribute('href'), href);
        assert.equal(await f.p.evaluate(async href => (await fetch(href)).text(), href), PDF);
        assert.equal(await f.p.locator('#runStatusLabel').textContent(), 'GELB');
      } finally { await f.close(); }
    });
    await check('closing the observation stream does not abort an in-flight PDF', async () => {
      const f = await fixture(model(), { holdPdf: true });
      try {
        await f.p.waitForFunction(() => document.querySelector('.pdf-link')?.getAttribute('href') === '#');
        for (let i = 0; i < 50 && !f.held.length; i++) await f.p.waitForTimeout(10);
        assert.equal(f.held.length, 1); await terminal(f);
        f.held[0].writeHead(200, { 'content-type': 'application/pdf' }); f.held[0].end(PDF);
        await f.p.waitForFunction(() => document.querySelector('.pdf-link')?.href.startsWith('blob:'));
        await f.stop();
        assert.equal(await f.p.evaluate(async () => (await fetch(document.querySelector('.pdf-link').href)).text()), PDF);
      } finally { await f.close(); }
    });
    await check('a permanently closed transport cannot invent a completed run', async () => {
      const f = await fixture(model(), { denyReconnect: true });
      try { f.streams[0].end(); await transport(f.p, 'closed');
        assert.equal(await f.p.evaluate(() => events.readyState), 2);
        assert.equal(await f.p.locator('#runStatusLabel').textContent(), 'RUNNING');
      } finally { await f.close(); }
    });
    assert.deepEqual(errors, []); console.log(`${passed} real-network browser scenarios passed`);
  } finally { await browser.close(); }
})().catch(e => { console.error(e); process.exitCode = 1; });
