# r722 PayloadStableManagedTreeApply

## Zweck

r716 hat direkt belegt, dass der bisherige Full-Tree-Apply auch byteidentische Dateien physisch neu schreibt und dadurch Cargo-Units allein ueber neue mtimes invalidiert. r722 schliesst exakt diese belegte Ursache.

## Aenderung

Der Take-Bootstrap behaelt die bestehende vollstaendige Payload-Pruefung, Transaktionssicherung und den bytegenauen Installed-Tree-Readback. Nur die Materialisierung des bereits zugelassenen Payloads wird inkrementell:

- byteidentische Datei: nicht anfassen,
- geaenderte Datei: kopieren,
- neue Datei: anlegen,
- entfallene Managed-Datei / entfallener Managed-Unterordner: entfernen,
- Datei/Ordner-Typwechsel: fail-closed materialisieren und anschliessend bytegenau verifizieren.

Auch die TakeManaged-Rootdateien werden bei identischen Bytes nicht mehr neu geschrieben. Root `Cargo.lock` bleibt unveraendert `LocalTransactional` und ist kein eingehender Take-Payload.

## Unveraenderte Sicherheitsgrenzen

Keine Aenderung an:

- Take-Tree-Integrity,
- Change-Control oder deklarierter Diff-Pflicht,
- Semantic-Capability-Diff,
- Kandidaten-SHA / Rejection-Persistenz,
- Backup vor Apply,
- `apply_and_verify()`-Readback nach Apply,
- Rollback / Recovery,
- Root-Cargo.lock-Resolution,
- Successful-Take-Package-Clean,
- Clippy-, Build- oder Testbreite,
- Produkt-, Musik-, Writer-, Renderer- oder PDF-Semantik.

## Regression

Zwei Bootstrap-Unit-Regressionen binden den Materialisierungsvertrag:

1. identische Datei -> `sync_file_payload()` meldet `false` und kopiert nicht; geaenderte Datei -> `true` und neue Bytes,
2. Managed-Directory-Mirror behaelt vorhandene gleiche Payloads, materialisiert neue/geaenderte Payloads und entfernt entfallene Eintraege.

Der bestehende transaktionale Commit-/Rollback-Test bleibt unveraendert und der Installed-Tree-Readback bleibt die abschliessende Byte-Authority.

## Beweisgrenze

Der Take entfernt nur mtime-Invalidierung durch den Apply. Er entfernt bewusst nicht den bestehenden EtuSchlopf-Package-Clean und konsolidiert keine Kodex-/Proof-Arbeit. Der reale Zeitgewinn wird erst durch den normalen Zielsystemlauf sichtbar.
