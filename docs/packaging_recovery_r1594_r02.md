# r1594 r02 – Paketierungskorrektur

Unabhaengiger neuer Kandidat auf dem weiterhin angenommenen r1593.
Der erste r1594-Kandidat wurde vor Uebernahme abgelehnt, SHA256
`b91916bf4297609250f62e94590a8b75897731d8a905a04cca78e7005f81bccc`.
Er ist Failure Evidence und kein akzeptierter Parent.

Ursache war die unbeabsichtigt mitverpackte lokale Datei
`tools/assistant_diagnostics/__pycache__/extract_clef_inputs.cpython-312.pyc`.
Die lokale Vergleichskopie enthielt denselben Cache; deshalb erkannte der
vorherige Vergleich ihn nicht als zusaetzlichen Paketinhalt. Die native
Diffpruefung hat den Take korrekt vor seiner Uebernahme gestoppt.

r02 entfernt diese Cachedatei und aendert ausschliesslich Paketidentitaet,
Deklaration und diese Dokumentation. Saemtliche Produkt-, Test-, Gate-,
Selektor-, Writer- und Cargo-Bytes bleiben gegenueber dem abgelehnten r1594
bytegleich. Die dort bestandenen lokalen 2911 Tests und vier externen Proben
werden als wiederverwendete Pruefevidenz ausgewiesen, nicht als neuer Lauf.
Der native Korpus-/PDF-Nachweis steht weiterhin aus.

Der neue Paketbaum wird direkt gegen die unveraenderten ZIP-Bytes des nativen
r1593-Source-Snapshots geprueft, nicht gegen eine durch Diagnose veraenderbare
Arbeitskopie. Dessen SHA256 ist
`08b709529706f210b95f0de0dbe585c963160e98a0e46984e614dc86bf875a3f`.
Cargo.lock bleibt gemaess bestehendem Managed-Tree-Vertrag ausserhalb des Takes.
Alle tatsaechlichen Aenderungen sind mit ihren finalen FNV64-Werten deklariert;
keine unveraenderten Dateien werden deklariert. Es bleiben keine pyc/pyo-Dateien
oder __pycache__-Eintraege. Der fertig gepackte und erneut entpackte Baum wird
nochmals gegen denselben sauberen Parent verglichen. Keine Gate-Abschwaechung.

Weiter unveraendert: `run.cmd --take --all`.
