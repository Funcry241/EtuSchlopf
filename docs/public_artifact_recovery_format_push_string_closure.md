# PublicArtifactRecoveryFormatPushStringClosure

## Scope

Dieser SecurityOnly-Take bereinigt genau die neun physischen
`format_push_string`-Fundstellen in
`src/security_control_plane/public_artifact_recovery.rs`.

Betroffen ist ausschliesslich `serialize_journal`. Die sechs Kopfzeilen und
drei Artefaktzeilen des Recovery-Journals werden direkt mit `writeln!` in den
bereits vorhandenen `String` geschrieben. Die vorherige Form erzeugte fuer
jede Zeile zuerst einen temporaeren `String` mit `format!` und kopierte ihn
danach mit `push_str`.

## Unveraenderte Vertragsbytes

Die folgenden Eigenschaften bleiben bytegleich:

- Feldnamen und Feldreihenfolge
- Dezimal- und Hexdarstellung
- Zustandsnamen
- Projektwurzel- und Projektnamenkodierung
- Artefaktindizes
- Ersatzwert `-` fuer nicht vorhandene SHA-256-Werte
- genau ein LF nach jeder Journalzeile

Die bestehende direkte Parse-Readback-Pruefung in `persist_state_journal`
bleibt unveraendert und prueft weiterhin die vollstaendige Serialisierung vor
der atomaren Veroeffentlichung.

## Sicherheitsbindung

Public-Artifact-Recovery und Build-Policy sind FNV-gepinnt. Deshalb werden
koordiniert aktualisiert:

- `PUBLIC_ARTIFACT_RECOVERY_FNV1A64`
- der entsprechende Pin in `src/identity_debt_build.rs`
- `IDENTITY_BUILD_POLICY_FNV1A64`
- beide Compile-Time- und Test-Bindungen im Trust Anchor

Recovery-Zustaende, Transaktionspfade, Marker, Restore-/Commit-Entscheidungen,
Fail-Closed-Pruefungen, SHA-256-Vertraege, Musik, Writer und Reports bleiben
unveraendert.

## Erwartete Diagnosewirkung

Der bestaetigte v0.10.1669-Lauf meldet 1.639 Deep-Clippy-Diagnosen. Genau neun
davon sind die hier behandelten `format_push_string`-Stellen. Erwarteter
Folgestand nach realer Abnahme: 1.630.
