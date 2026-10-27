# Packaged EXE: renderer startup projection

The missing-renderer startup text is:

```text
MuseScore ist nicht installiert. Die Notendateien werden trotzdem erstellt.
Für eine PDF installiere MuseScore kostenlos: https://musescore.org/de/download

```

The distribution EXE queries the embedded engine using the internal hidden
`--renderer-available` option before the input prompt or output workspace.
Exit 0 means the existing render.rs discovery/validation finds an existing
renderer executable, exit 3 means absent. Other query failures warn and continue
the ordinary requested run.
The query reads the same candidate population: explicit configured renderer,
Registry/App-Paths, standard Windows paths and PATH; bare fallback names alone
are not treated as installation evidence. It writes no notation or PDF and
returns before project preparation and cleanup. Presence does not prove a
successful PDF export; that remains the real renderer's result.

Without an available renderer the bootstrap prints the exact text and invokes
the ordinary engine with --no-pdf, so the truthful not-requested report mode is
used and notation processing continues. An explicit --no-pdf remains honored
when MuseScore exists. The closing message mentions opening the PDF only if the
actual per-run PDF is a regular file. Previous runs are not searched for a PDF.
Without MuseScore, the closing message first gives the installation link and
only then tells the user to open the Render file after installing it.

The bootstrap supplies a fresh private `--internal-work-dir` inside its
per-process runtime. Existing intermediate renderer evidence is stored there;
the musician's result directory contains the persistent products, input copies
and reports. Product paths and report artifact locators are not relocated or
rewritten. Native invocations without this internal option retain their usual
project-local target directory.

Package validation must use real engine/package bytes on two Windows setups:
without MuseScore (message present, notes/report, no PDF, no PDF invitation)
and with MuseScore (actual PDF, successful source bindings, PDF invitation).
Additionally test installed MuseScore with explicit --no-pdf. Hash original
inputs and all earlier result trees before and after repeat and failure runs.
Unit/stub tests cannot substitute for these real package checks. These checks
are required before publishing the corrected pre-release, not claimed here.
