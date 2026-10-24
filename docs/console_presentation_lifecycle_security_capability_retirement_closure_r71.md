# ConsolePresentationLifecycleSecurityCapabilityRetirementClosure r71

## Revisionsprovenienz

Autoritativer Parent ist EtuSchlopf `0.10.1839 r67`
`ConsolePresentationContractRenderPolicyFoundation`, real committed mit Reexec 0.

Die r68-Paketfamilie wurde vor Installation korrekt fail-closed abgewiesen: zuerst wegen
eines unmanaged Root-`Cargo.lock`, danach wegen mehrdeutiger Pakete derselben Revision.
r69 beseitigte diese Paketidentitaetsprobleme und erreichte den Semantic-Capability-Diff,
wurde dort jedoch ebenfalls vor Installation korrekt abgewiesen. Der Grund war fachlich
berechtigt: der Console-Lifecycle-Cutover entfernt zwei historische
`SecurityNegativeCheck`-Capabilities des alten Phase-/Globalfarben-Vertrags, ohne die bereits
installierte `SecurityCapabilityRetirement`-Authority explizit zu benutzen.

Keine r68- oder r69-Revision wurde installiert. r70 erreichte PRE-TAKE-DIFF-GATE GRUEN und wurde transaktional installiert, scheiterte jedoch im Reexec vor Launcher-Kompilation an einem syntaktischen Defekt in einem neuen Regressionstest (`mismatched closing delimiter`). Der Take-Bootstrap fuehrte daraufhin korrekt `TAKE-TRANSACTION ROLLBACK` aus und stellte r67 vollstaendig wieder her.

r71 ist deshalb ein neuer, eindeutiger SecurityOnly-Kandidat auf dem weiterhin autoritativen r67-Baum. Gegenueber r70 aendert r71 ausschliesslich die Syntax dieses neuen Regressionstests sowie die dadurch notwendig neu gebundenen Launcher-/Identity-Fingerprints und Revisionsmetadaten; die produktive Runtime-Semantik des Console-Cutovers bleibt unveraendert.

## Exakte SecurityCapabilityRetirements

r71 pensioniert ausschliesslich zwei in r67 vorhandene negative/security Checks aus
`src/bin/etuschlopf_launcher.rs`:

1. `launcher_console_phase_cache_tracks_visible_phase_transitions#1`
   - prior FNV64: `a2a3fb6952594b50`
2. `launcher_console_phases_use_distinct_role_colors#1`
   - prior FNV64: `c747245574f08f61`

Stable reason fuer beide Eintraege:

`console_legacy_phase_color_security_checks_retired`

Die Eintraege sind append-only in `src/build_gate/security_capability_retirements.txt` an
CapabilityKind, Pfad, Symbol und den exakten r67-Rejection-Fingerprint gebunden.

Die pensionierten Checks gehoeren zum alten Console-Vertrag:

- ein testinterner Cache fuer sichtbare `Phase`-Transitionen;
- die gekoppelte ANSI-/Windows-`color`-Rollenpalette.

Der neue Console-Vertrag besitzt weder den globalen Windows-Color-State noch dieselbe
Phase-Cache-Authority. Ein Stub, Rename oder verschobener Nachbau dieser beiden Checks waere
keine Pensionierung und ist ausdruecklich nicht Bestandteil von r71.

## Produktiver Runtime-Cutover

Der produktive Console-Payload entspricht dem in r69 bereits vorgeprueften Cutover:

- `run.cmd` besitzt keine `color XX`-Authority mehr;
- der Launcher startet kein `cmd /C color` mehr;
- tiefe Launcher-`std::process::exit(...)`-Pfade werden als internes
  `LauncherRequestedExit` bis `main()` zurueckgegeben;
- der prozesslokale `ConsoleSession` kann dadurch normal abgebaut werden;
- stdout und stderr werden unabhaengig als Terminal/Redirect erkannt;
- `NO_COLOR`, `ETUSCHLOPF_COLOR`, `ETUSCHLOPF_PROGRESS` und
  `ETUSCHLOPF_ASCII_PROGRESS` werden an die r67-Render-Policy gebunden;
- Redirect bleibt plain und zeilenorientiert;
- Child-Heartbeats erhalten explizite `PresentationRole`-Werte statt Label-Parsing;
- insbesondere ist `Rust-Tests alle Targets` explizit `PresentationRole::Test`;
- Wrapper-`FEHLER:`-Zeilen gehen auf stderr;
- Take-Bootstrap bindet denselben std-only Shared-Source-Vertrag ohne Verlust seiner
  Compile-/Recovery-Isolation ein.

## Neue Negativchecks statt Relocation

Die r71-Launcher-Tests beweisen den neuen Vertrag direkt, unter anderem:

- typisierte Progress-Rollen statt `phase_for_heartbeat`-Stringanalyse;
- kein `cmd /C color` im produktiven Launcher;
- kein produktives tiefes `std::process::exit` im Launcher/Build-Control;
- die verbleibende ANSI-Rollenpalette ist nur ein temporaerer terminal-lokaler Rendereradapter;
- finale Outcome-Farben bleiben an die bestehende `TruthfulRunOutcomeProjection` gebunden.

Diese Tests sind keine umbenannten oder verschobenen Koerper der pensionierten r67-Checks.

## Security-Control-Plane-Pins

Da der Retirement-Ledger selbst gepinnte Security-Architektur ist, aktualisiert r71 regulaer:

- `PINNED_SECURITY_CAPABILITY_RETIREMENTS_FNV1A64` in `identity_debt_build.rs`;
- den entsprechenden Ledger-Trust-Anchor;
- den daraus geaenderten `IDENTITY_BUILD_POLICY_FNV1A64` im Trust Anchor.

Die bereits durch den Console-Cutover notwendigen Pins fuer Launcher, Take-Bootstrap und
`run.cmd` bleiben synchronisiert. Kein Gate wird gelockert oder umgangen.

## Bewusst unveraendert

Keine Aenderung an:

- Musiklogik oder Notationsentscheidungen;
- MusicXML oder PDF;
- RendererAuto, Renderer-Readback oder Renderer-Feasibility;
- Joint Page/System Planning;
- Candidate-Erzeugung oder Selection;
- run-lokalen No-Goods;
- DemandBinding, Thresholds oder Kapazitaeten;
- Writer, Materialisierung oder Structural Write Authority;
- Assistant-Export-Semantik;
- Neuro-Evidence-Semantik;
- Public-Artifact-Transaction-Semantik;
- Take-Transaktions-/Recovery-Semantik;
- ReleaseDecision, RunSeverity oder finaler Outcome-Projektion;
- Child-Logrouting;
- audit-only `--all`-Evidence-Transport
  `ETUSCHLOPF: HorizontalLayoutGlobalPageCountPreferenceShadow:`.

ChangeClass: `SecurityOnly`.

## Abnahme

Autoritativer Lauf:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartet werden insbesondere:

- eindeutige Auswahl von r71 trotz liegenbleibender r68/r69-Pakete;
- `ChangeDependencyPreflight` gruen;
- der Semantic-Capability-Diff akzeptiert exakt die zwei neuen
  `SecurityCapabilityRetirement`-Eintraege;
- PRE-TAKE-DIFF-GATE GRUEN;
- Clippy/Build/Rust-Tests/Programm gruen;
- kein globaler Windows-Color-State aus Wrapper/Launcher;
- TAKE-TRANSACTION COMMIT nach Reexec 0.
