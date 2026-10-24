# r777 – LauncherPhaseCompatibilityRetirementCompileClosure

## Authority

r777 basiert ausschließlich auf dem real `dark_green`/`released` angenommenen r775
`ReportSanitizerClosedWorldLegacyVocabularyRetirement`.

ChangeClass ist `KodexOnly`, weil die betroffene Launcher-/Visual-Preflight-Fläche zum
Kodex-/Control-Plane-Vertrag gehört. Der Schnitt erweitert keine produktive Autorität.

## Failure Evidence r776

r776 `LauncherPhaseCompatibilityRetirement` ist verworfen und kein Parent. Der reale Windows-Reexec
erreichte nach erfolgreichem PRE-TAKE-DIFF-GATE den Rust-Build und scheiterte dort ausschließlich
an E0716: `progress_line_presentation(progress_role, &format!(...))` lieh eine temporäre `String`-
Instanz über das Statement hinaus. r777 übernimmt denselben Retirement-Schnitt vollständig und
bindet die Inline-Progress-Nachricht vor dem Borrow an `inline_message`. Keine Präsentationssemantik
und keine Capability ändert sich durch diese Compile-Closure.

## Ursache

Der Launcher besaß bereits den typisierten Präsentationsvertrag
`PresentationRole + LocalStatus + Attention + Emphasis`, leitete aber weiterhin einen Teil
der Ausgabe über die ältere Kompatibilitätsschicht

```text
Phase
  -> legacy_presentation_line(...)
  -> PresentationRole / LocalStatus / Attention / Emphasis
```

Zusätzlich übersetzte `phase_for_presentation_role(...)` die bereits typisierte Rolle für
Progress-Ausgaben wieder zurück in `Phase`. Diese Rückübersetzung besaß keine eigenständige
fachliche Autorität; sie konservierte nur die historische Darstellung.

## Schnitt

r777 pensioniert die Kompatibilitätsinsel vollständig:

- `enum Phase` wird entfernt,
- `legacy_presentation_line(...)` wird entfernt,
- `phase_for_presentation_role(...)` wird entfernt,
- `Launcher::line(...)` konsumiert den typisierten Präsentationsvertrag direkt,
- die verbleibenden Launcher-, Build-Control-, Visual-Audit- und No-Seed-Callsites geben ihre
  bereits bekannte Rolle, lokalen Status, Aufmerksamkeit und Betonung direkt an,
- der nach der Umstellung tote `Launcher::inline(...)`-Wrapper wird entfernt.

Für Progress-Zeilen bleibt die bisherige sichtbare Gruppierung eng lokal erhalten:
`Kodex/Check/Test -> Check`, `Program/Renderer/Artifact -> Program` und
`Export/Result -> Result`. `progress_line_presentation(...)` materialisiert nur diese
Darstellungsprojektion; es entsteht keine neue fachliche oder Writer-Autorität.

## Fail-closed Rückkehrschutz

Der bereits vorhandene Launcher-Kodex-Test
`launcher_console_runtime_has_no_global_color_subprocess_or_process_exit` wird ohne neue
Testfunktion um drei Negativbedingungen erweitert. Er weist künftig fail-closed zurück,
wenn eine der pensionierten Kompatibilitätsformen wieder in den Launcher gelangt:

```text
enum Phase
legacy_presentation_line(...)
phase_for_presentation_role(...)
```

Damit ist die Entfernung nicht nur ein aktueller Quelltextzustand, sondern als
Nicht-Rückkehr-Vertrag gebunden.

## Trust-Bindung

Da `src/bin/etuschlopf_launcher.rs` Kodex-/Control-Plane-Bytes trägt, werden die bestehenden
Fingerprint-Eigentümer ausschließlich auf die finalen r777-Bytes neu gebunden:

- `src/build_gate/pinned_policy_kodex.rs`,
- `src/identity_gate_trust_anchor.rs`.

Es wird kein Pin umgangen und keine Trust-Grenze gelockert.

## Closure-Beweis

Für die aktive Launcher-Fläche gilt nach dem Schnitt:

```text
Phase type references                 = 0
legacy_presentation_line references   = 0
phase_for_presentation_role references= 0
semantic_line parallel API            = 0
```

Der bestehende Negativtest bindet die drei pensionierten Legacy-Formen gegen Rückkehr.

## Nicht geändert

Keine Änderung an MusicXML-, PDF- oder Musiksemantik, Selection, Writer,
StructuralWriteAuthority, Repair/Recovery, öffentlicher Artifact-Recovery-Reihenfolge,
Release Authority oder Export Authority. Es entsteht keine etüdenspezifische Logik und keine
neue Reparaturschicht.
