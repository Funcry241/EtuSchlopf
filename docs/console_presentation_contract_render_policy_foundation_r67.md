# ConsolePresentationContractRenderPolicyFoundation r67

## Ausgangspunkt

Autoritativer Parent ist EtuSchlopf `0.10.1839 r66`
`SecurityCapabilityRetirementAssistantTransferSurfaceClosure`.

Die Console-Inventur hat vier EtuSchlopf-eigene Output-Produzenten bestaetigt:
`run.cmd`, `etuschlopf_take_bootstrap`, `etuschlopf_launcher` und die eigentliche
Pipeline. Launcher und Pipeline besitzen heute getrennte Farb-/Progressvertraege;
`run.cmd` und Launcher mutieren ausserdem den globalen Windows-Console-Farbzustand.
Der Launcher projiziert Child-Prozesse kompakt, waehrend deren vollstaendige Ausgabe
in technische Logs geht.

## Thread-4-Grenze

Interactive Console und Technical Log sind zwei logische Projektionen, aber nicht
notwendig genau zwei physische Dateien oder Streams. Insbesondere besitzen Clippy,
Rust-Tests und Programmlauf heute unterschiedliche technische Logziele.

Raw Child Output ist kein `PresentationLine`-Strom. Child-Prozesse liefern Exitcode,
Dauer und technische Ausgabe; der EtuSchlopf-Orchestrator besitzt die sichtbare
Zusammenfassung. Der normale Launcherpfad leitet stdout/stderr der Child-Prozesse
bereits direkt in Dateien um und erzeugt dadurch keinen klassischen Pipe-Deadlock.

Der `--all`-Pfad besitzt eine wichtige historische Ausnahme: Der audit-only
`BatchChildPreferenceCapture` liest eine exakt gepraefixte stdout-Zeile
`ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreferenceShadow:` als bereits
akzeptierten Evidence-Transport. Dieser Vertrag darf durch den Console-Refactor
nicht still umformatiert, unterdrueckt oder umgeleitet werden. Eine spaetere
Entkopplung waere ein eigener Audit-/Evidence-Schnitt und ist nicht Teil dieses
Console-Takes.

## Ziel dieses Takes

r67 fuehrt ausschliesslich einen kleinen, authority-freien Presentation- und
Render-Policy-Vertrag als leaf-like Shared Source ein:

- `PresentationComponent`
- `PresentationRole`
- `LocalStatus`
- `Attention`
- `Emphasis`
- `Visibility`
- `PresentationChannel`
- `PresentationContext`
- `PresentationLine`
- `PresentationProgress`
- getrennte stdout-/stderr-faehige `StreamCapability`
- `TerminalCapability`
- `ConsolePreferences`
- reine `resolve_stream_render_policy(...)`-Logik

`Attention` ist absichtlich **nicht** `RunSeverity`. Der finale `RunOutcome`,
`ReleaseDecision` und `RunSeverity` bleiben ausserhalb dieses Moduls und damit
alleinige finale Authority.

## Redirect- und NO_COLOR-Grundlage

Die reine Render-Policy setzt bereits die spaetere harte Regel fest:

- Redirect ist immer `Plain`.
- Redirect ist immer zeilenorientiert, niemals Inline-Progress.
- Auch `ColorPreference::Always` darf Redirect nicht mit Terminal-Escapes
  verschmutzen.
- `ColorPreference::Never` laesst ein echtes Terminal interaktiv, aber plain.
- stdout und stderr werden unabhaengig aufgeloest.
- Unicode-/ASCII-Glyphen sind ein eigener Vertrag und keine Farbentscheidung.

Die Runtime liest in r67 noch keine Environmentvariablen in diesen Vertrag ein.
`NO_COLOR`, `ETUSCHLOPF_COLOR`, `ETUSCHLOPF_PROGRESS` und
`ETUSCHLOPF_ASCII_PROGRESS` werden erst beim produktiven Cutover gebunden.

## Bewusst noch kein Runtime-Cutover

r67 bindet den neuen Vertrag **nicht** in Launcher, Take-Bootstrap oder Pipeline ein.
Das ist Absicht:

1. `etuschlopf_launcher.rs` und `etuschlopf_take_bootstrap.rs` sind gepinnte
   Security-Control-Plane-Flaechen.
2. Der Take-Bootstrap besitzt eine harte Compile-Isolation fuer Recovery.
3. Der Launcher verwendet noch tiefe `std::process::exit(...)`-Pfade; eine echte
   `ConsoleSession` waere vor deren Lifecycle-Cutover keine belastbare RAII-Garantie.
4. `run.cmd color XX` und `cmd /C color XX` bleiben deshalb in r67 byteidentisch.

Der Shared-Source-Vertrag wird stattdessen durch einen eigenen Integrationstest
kompiliert und auf Leaf-Eigenschaften geprueft. Erst der folgende Security-aware
Cutover darf Launcher/Bootstrap an diese Quelle anbinden.

## Nebenlaeufigkeit

Die Inventur bestaetigt ausserdem, dass parallele Pipeline-Worker ihre
Fortschrittswerte zwar atomar zaehlen, die sichtbaren `report_progress(...)`-Aufrufe
aber in anderer Threadreihenfolge am Sink eintreffen koennen. Ein spaeterer
Pipeline-Presentation-Sink darf deshalb stale Progressupdates an der Sink-Grenze
koaleszieren, ohne einen Mutex in die musikalische Domain zu tragen. r67 aendert
hiervon noch nichts.

## Tests

`tests/console_presentation_contract.rs` prueft:

- Role, LocalStatus und Attention sind unabhaengige Dimensionen.
- Redirect bleibt selbst bei erzwungener Farbe plain und zeilenorientiert.
- No-Color-Praeferenz erzeugt plaines, aber weiterhin interaktives Terminalrendering.
- stdout und stderr koennen unabhaengig aufgeloest werden.
- ASCII-Praeferenz/-Capability ist getrennt von Farbe.
- der Contract importiert keine Domain-, Outcome-, Prozess- oder Renderingauthority.
- Technical-only Visibility ist explizit statt textheuristisch.
- der bestehende `--all`-PageCount-Evidence-Transport bleibt exakt adressierbar.

## Nicht geaendert

Keine Aenderung an:

- Musiklogik
- MusicXML
- PDF
- Renderer
- Joint Planning
- Renderer-Feasibility
- Candidate-Erzeugung oder Selection
- No-Goods
- DemandBinding
- Thresholds
- Writern oder Materialisierung
- Neuro-Evidence-Semantik
- Assistant-Export-Semantik
- ReleaseDecision / RunSeverity
- stdout/stderr-Runtimeverhalten
- Child-Prozess-Pipes
- Console-Farben
- `run.cmd`
- Launcher-/Bootstrap-Lifecycle

ChangeClass: `RefactorOnly`.

## Naechster Take

Der folgende Cutover darf auf dieser Grundlage:

- Launcher-`process::exit` in Rueckgabecodes schneiden,
- eine prozesslokale `ConsoleSession` einfuehren,
- Launcher und Take-Bootstrap an denselben Shared-Source-Vertrag binden,
- `run.cmd color XX` und Launcher-`cmd /C color` entfernen,
- `NO_COLOR`/Redirect/TTY-Fakten zentral binden,
- Heartbeats typisieren,
- stdout/stderr-Presentationrouting explizit machen.

Erst danach folgt der Pipeline-/Visual-Hierarchy-Cutover.
