# Retired LocalBest/Browser-Cursor Trace Projection Retirement

Stand: v0.10.1587 r02

## Revision r02

`r01` wurde im Strict-Clippy-Gate vor der Installation verworfen. Nach dem
Rückbau der pensionierten Trace-Projektionen blieb im aktiven
`RawCursorInvariantHardGateAudit` das Feld `render_measures` gespeichert,
obwohl die Maßzahl nur noch unmittelbar für `pipeline_line` benötigt wird.
Wegen `#![deny(dead_code)]` war der Take deshalb nicht installierbar.

`r02` entfernt ausschließlich diese tote Speicherung und die beiden
Testinitialisierungen. Der Render-Maßzähler, die erzeugte Pipelinezeile, der
aktive Hard-Gate-Scan sowie Blocker- und Releasewirkung bleiben unverändert.
`r01` darf nicht als Ausgangspunkt verwendet werden.

## Ausgangsbefund

Der grüne Lauf von v0.10.1586 r02 / Export 526 bestätigt den vorherigen
Cross-Staff-Rückbau:

- Strict-Build und Strict-Clippy erfolgreich,
- `FinalExit 0`,
- Master- und Render-MusicXML gegenüber v0.10.1585 nur mit geändertem
  `<software>`-Versionsstring,
- alle sechs PDF-Seiten pixelidentisch.

Im anschließenden Architektur-/Laufzeitblick zeigte sich eine weitere bereits
pensionierte Familie. Elf LocalBest-/TieBreaker-/PreCandidate-Projektionen,
eine InternalPreCandidate-Projektion, ein CursorLaneOrigin-Trace sowie sechs
FinalCursor-Projektionen wurden weiterhin bei jedem Projekt konstruiert.

Diese Objekte besaßen ausschließlich:

- einen `pipeline_line`-String,
- teilweise einen `done_fragment`-String,
- Zähler über vorhandene XML-Snapshots oder bereits bekannte Auditwerte.

Sie besaßen kein Entscheidungs-, Auswahl-, Writer-, Verifier- oder
Release-Recht. Im kompakten finalen Ergebnisbericht von Export 526 erscheint
keine ihrer Kennungen.

## Unnötiger Laufzeitpfad

Besonders teuer war `build_final_cursor_audit_projection_set`:

1. Master-MusicXML wurde vollständig in eine zweite Cursor-/Rest-Risk-Map
   gescannt.
2. Render-MusicXML wurde vollständig in dieselbe parallele Risk-Darstellung
   gescannt.
3. Daraus wurden sechs historische Retirement-Zeilen konstruiert.
4. Diese Zeilen wurden anschließend nicht in den finalen Bericht projiziert.

Der aktive `RawCursorInvariantHardGate` führt weiterhin seinen eigenen,
releasewirksamen Master-/Render-Scan aus. Der pensionierte Risk-Map-Pfad war
daher keine Sicherheitsautorität, sondern eine zweite, nicht projizierte
Darstellung derselben groben XML-Domäne.

## Checkpoint-Trace

Die sechs `RenderXmlCheckpointState::capture(...)`-Aufrufe und fünf
vollständige Stage-Outcome-XML-Snapshots existierten nur,
um die pensionierten Trace-Projektionen mit Zwischenständen zu versorgen.
Nach deren Entfernung gibt es keinen Consumer mehr.

Darum entfallen gemeinsam:

- `RenderXmlCheckpointTraceEntry`,
- `RenderXmlCheckpointTrace`,
- sechs Capture-Aufrufe,
- Arc-Sharing ausschließlich für Trace-Snapshots,
- `into_current_and_trace`.

Der Checkpoint-State besitzt wieder genau einen `String`. Ein No-Write bleibt
eine echte Nulloperation; ein bestätigter Write ersetzt den String durch den
materialisierten Checkpoint.

## Entfernte Autoritätsattrappen

Vollständig gelöscht werden:

- `localbest_identity_tiebreaker_core.rs`,
- `localbest_staffvoice_authority_tail.rs`,
- elf pensionierte LocalBest-/TieBreaker-/PreCandidate-Projektionen,
- InternalPreCandidateEventDelta-Retirement,
- CursorLaneOrigin-Retirement,
- die FinalCursor-Risk-/Contrast-/Causality-Retirement-Familie,
- der nicht sichtbare `STAFF-VOICE-FIBER-AUDIT-CHAIN-INDEX-RETIRED`-Abschnitt,
- die nur dafür nötigen Report-Traits und Sequenzadapter,
- die nur dafür erzeugten Snapshot-Listen.

## Unveränderte Autorität

Unverändert bleiben:

- `RawCursorInvariantHardGate`,
- dessen Blocker-/Quarantäne-Logik,
- sämtliche produktiven Writer,
- die Passreihenfolge,
- MusicXML- und Render-MusicXML-Auswahl,
- PDF-Quellenauswahl,
- musikalische Policies,
- RestPlacement,
- Release-/Outcome-Projektion.

## Drei-Achsen-Bilanz

### Schönheit

Der Checkpoint-State besitzt keinen historisch begründeten Trace-Nebenowner
mehr. Der Orchestrator baut keine Projektionen, die später vollständig
verschwinden.

### Geschwindigkeit

Garantiert entfallen pro Projekt:

- ein vollständiger Master-MusicXML-Risk-Scan,
- ein vollständiger Render-MusicXML-Risk-Scan,
- sechs Checkpoint-Captures,
- fünf vollständige XML-Zwischenkopien aus Stage-Outcomes,
- mehrere Snapshot-Vector-Aufbauten und -Erweiterungen,
- Snapshot-Nonempty-Zählungen,
- mindestens zwanzig Retirement-Stringformatierungen,
- die anschließende polymorphe Pipeline-Line-Sequenz.

Ein Sekundenwert wird erst durch den realen Lauf bewertet.

### Komplexität

- zwei Rust-Dateien gelöscht,
- paralleles Cursor-Risk-Modell gelöscht,
- Trace-Typfamilie aus dem Checkpoint-Owner gelöscht,
- Report-Adapter und tote Projektionsfläche gelöscht,
- keine Ersatzabstraktion,
- kein Shadowpfad,
- kein neuer Guard.

## Beweisgrenze

Der Take ist `RefactorOnly`. Die entfernten Kennungen sind im finalen
v0.10.1586-Ergebnisbericht nicht vorhanden. Der aktive Raw-Cursor-Hard-Gate
bleibt bestehen. Daher muss die Abnahme zeigen:

- Strict-Build/Clippy grün,
- `FinalExit 0`,
- Master-/Render-MusicXML gegenüber v0.10.1586 nur im Versionsstring
  verschieden,
- PDF pixelidentisch,
- keine neue Report-/Release-Abweichung.
