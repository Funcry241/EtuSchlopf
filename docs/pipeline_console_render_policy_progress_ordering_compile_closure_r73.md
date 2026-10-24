# PipelineConsoleRenderPolicyProgressOrderingCompileClosure r73

## Parent

Autoritativer Parent ist EtuSchlopf `0.10.1839 r71`
`ConsolePresentationLifecycleSecurityCapabilityRetirementClosure`, real committed.

r72 war der erste RefactorOnly-Versuch des Pipeline-Console-Cutovers. Der echte
Windows-Strict-Clippy-Lauf zeigte 12 Compile-Fehler: der alte `colorize`-Helper war
zusammen mit der doppelten Policy-Logik entfernt worden, obwohl bestehende explizite
Artifact-/Progress-Renderer weiterhin einen Segment-Styler benoetigen; ausserdem blieb
in der PDF-Quellenprojektion ein Callsite des bewusst entfernten
`progress_color_context_for_label` zurueck. r72 wurde nicht als gruener Stand
autoritativ.

## r73-Ziel

r73 uebernimmt den beabsichtigten r72-Refactor auf dem committed r71-Parent und
schliesst die beiden Compile-Kanten ohne Wiederherstellung der alten semantischen
Stringheuristiken.

- Die Pipeline bindet weiterhin den gemeinsamen std-only
  `console_presentation_contract` und cached die aufgeloeste stdout-Render-Policy
  pro Prozess.
- `NO_COLOR`, `ETUSCHLOPF_COLOR`, `ETUSCHLOPF_PROGRESS`,
  `ETUSCHLOPF_ASCII_PROGRESS` und Terminal/Redirect werden weiterhin nur ueber
  diesen Shared Contract aufgeloest.
- Der bestehende Name `colorize` bleibt als kleiner Segment-Renderer erhalten und rendert ANSI-Segmente ausschliesslich anhand
  der bereits aufgeloesten `StreamRenderPolicy`. Er besitzt keine eigene TTY-,
  Environment- oder Domain-Semantik.
- Die entfernten Labelparser `progress_color_context_for_label` und
  `progress_artifact_tag_for_label` bleiben entfernt.
- Die PDF-Quellenprojektion nutzt die bereits vorhandene typisierte Wahrheit
  `render_xml_selected_for_pdf` statt den Displaystring `pdf_source_label`
  semantisch zurueckzulesen.
- Generischer paralleler Fortschritt bleibt explizit `CORE`; explizite
  `ConsoleArtifactLine`-Werte bleiben Artifact-Authority.
- Die monotone Anzeige-Koaleszierung bleibt nur an der Presentation-/Sink-Grenze.
  Der Koaleszer verwendet den logischen Roh-Labelschluessel statt einen bereits
  gestylten Displaystring. Timing-/Technical-Capture sieht weiterhin alle Ticks.

## Bewusst unveraendert

Keine Aenderung an Musik, MusicXML, PDF-Inhalt, Renderer, Planner, Feasibility,
Candidate/Selection, No-Goods, DemandBinding, Thresholds, Kapazitaeten, Writer,
Materialisierung, Child-Prozessrouting, Launcher/Take-Bootstrap-Runtime,
Assistant Export, Neuro Evidence, ReleaseDecision, RunSeverity, finaler
Outcome-Authority oder dem audit-only `--all` PageCountPreference-Transport.

## ChangeClass

`RefactorOnly`.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartet: RefactorOnly-Preflight und Semantic-Capability-Diff gruen; danach
Strict-Clippy ohne die r72-E0425-Fehler, Build/Tests/Programm-PDF gruen und
`TAKE-TRANSACTION COMMIT`.
