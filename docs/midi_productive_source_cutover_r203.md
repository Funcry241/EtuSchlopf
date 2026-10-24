# r203 MidiProductiveSourceCutoverSecurityAuthorityRecovery

Die beiden vorangehenden Kandidaten wurden vor Installation fail-closed abgewiesen. r201 verletzte unter RefactorOnly die Cargo.toml-Inhaltsgrenze durch eine rein kosmetische Beschreibungszeile. r202 behob dies, fuehrte aber zwei neue writer-erreichbare Wrapper-Funktionen in src/flp.rs ein; das NO-NEW-REPAIR-AUTHORITY-Gate blockierte diese neue Aufrufschicht korrekt. r203 setzt deshalb weiterhin direkt auf dem autoritativ gruenen r200 auf und fuehrt den Cutover innerhalb des bereits vorhandenen writer-erreichbaren Seed-Owners aus, ohne einen neuen Writer-Caller einzufuehren.

## Ziel
Standard-MIDI ersetzt FLP als produktive Rohnotenquelle, nachdem r200 fuer die gelieferten Quellen exakte kanonische Ereignisidentitaet nachgewiesen hat.

## Autoritaetsgrenze
- MIDI: produktive Quellnotenautoritaet.
- WAV: bestehende Audio-/Referenzevidenz.
- FLP: nur optionale Uebergangsevidenz; kein Solver-/Writerrecht.
- Tempo/Taktart aus MIDI: beobachtet, aber in r203 ohne neues musikalisches Entscheidungsrecht.

## Identitaetsstrategie
Der produktive Decoder erzeugt dieselben `(pos,len,midi)`-Quellereignisse auf 96 Divisions. Vor dem bestehenden Hand-/Solvercode werden sie nach der Note-On-Quellordinalzahl geordnet und mit demselben historischen MIDI60-Initialsplit in den unveraenderten Seedpfad uebergeben.

## Abnahme
Etude 6 muss dunkelgruen bleiben. Danach ist der Vollkorpus 10/10 aus WAV/MIDI auszufuehren und Master-/Render-MusicXML gegen r200/r199 zu vergleichen. Erst danach folgt ein eigener FLP-Retirement-Take; bis dahin bleibt FLP ausschliesslich optionale Uebergangsevidenz.
