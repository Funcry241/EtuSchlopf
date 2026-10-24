# TypedFinalWriterOutcomeAndCorpusManifestTruthClosure

**Version:** 0.10.1612 r02
**Change-Klasse:** OutcomeProjection
**Ausgang:** 0.10.1611

## Revisionsabschluss r02

`r01` wurde vor jeder Installation vom Architektur-Pin-Gate abgelehnt. Der
Launcher ist compile-time gepinnt und darf nur zusammen mit
`src/identity_debt_build.rs` und `src/identity_gate_trust_anchor.rs` geaendert
werden. r02 aktualisiert deshalb den Launcher-Fingerprint, prueft im
Build-Policy-Eigentuemer explizit die Reihenfolge
Rendererbindung -> Launcherdiagnose -> finale Berichtsendbindung und bindet
diesen Vertrag im Trust-Anchor. Der fachliche Outcome-/Manifestcode ist gegen
r01 unveraendert.

Der in r01 deklarierte, aber nicht vorhandene README-Diff ist in r02 ebenfalls
geschlossen: README enthaelt nun diese Revisionshistorie und der
Take-Change-Manifest-Diff entspricht dem realen Vollbaum.

## Problem

Der reale Zehnerkorpus von v0.10.1611 enthielt zehn erfolgreiche
Projektpipelines und zehn explizit beobachtete Renderer-Outcomes. Das
Korpusmanifest projizierte dennoch nur acht Writerzeilen als beobachtet und
leitete daraus faelschlich auch `renderer_outcome_observed=false` ab.

Die Projektion kannte nicht alle im vorhandenen finalen Writerbericht bereits
sichtbaren Schreibursachen. Zwei reale Faelle fielen aus:

1. Higher-Beam-Preservation mit `SourceHigherBeamNotesRestored > 0`.
2. Strukturelle Cross-Staff-Restunterdrueckung ohne klassischen Route- oder
   Beam-Lost-Begin-Zaehler.

Der anschliessende Fehlerpfad erweiterte `corpus_emergence.result.txt` noch um
die Launcherdiagnose. Weil die Manifestbindung nur bei ProgramExit 0 erneuert
wurde, war danach auch der Berichtshash veraltet.

## Neue Evidenztrennung

`CorpusRunProjectEvidence` und `CorpusRunManifest` unterscheiden nun zwei
unabhaengige Beobachtungen:

- `renderer_outcome_observed`: stammt ausschliesslich aus der expliziten
  `RendererOutcome:`-Zeile des Projektberichts.
- `writer_outcome_observed`: bedeutet, dass die vorhandenen finalen
  Writerzeilen vollstaendig geparst wurden und ihre Zaehler, Flags und
  Invarianten zusammenpassen.

Der Renderer kann damit nicht mehr durch das Fehlen eines bestimmten
Writerzaehlers unsichtbar werden.

## Schreibursachen

Die Manifestprojektion fuehrt getrennt:

- `routes_written`,
- `forward_nodes_replaced`,
- `forward_duration_replaced`,
- `beam_lost_begin_notes_written`,
- `cross_staff_rest_suppressions`,
- `source_higher_beam_notes_restored`,
- `rest_vertical_anchors_written`,
- `ledger_bounded_clef_boundaries_written`.

Ein `render_xml_write_committed` ist nur konsistent, wenn mindestens eine
dieser Ursachen positiv ist. Umgekehrt darf bei null Ursachen kein
Render-Write behauptet werden.

## Cross-Staff-Bilanz

Die vorhandene Writerzeile enthaelt bereits zwei voneinander unabhaengige
Bilanzen. Die Projektion berechnet:

```text
rest_delta = VisibleRestsBefore + ForwardNodesReplaced - VisibleRestsAfter
note_delta = NoteCountBefore    + ForwardNodesReplaced - NoteCountAfter
```

Nur wenn `rest_delta == note_delta`, wird der Wert als
`cross_staff_rest_suppressions` akzeptiert. Ein Unterlauf, Ueberlauf oder eine
abweichende Bilanz verwirft die gesamte Writerprojektion.

## Mutationsdimensionen

Der Manifestvertrag verwendet folgende eindeutige Oberflaechen:

- `render_xml_write_committed`,
- `render_notation_mutation`,
- `sounding_event_mutation`,
- `declared_music_mutation`,
- `master_musicxml_mutation`.

`declared_music_mutation` bewahrt die historische Writerdeklaration, ohne sie
mit einer klingenden Ereignismutation gleichzusetzen. Die
`sounding_event_mutation` wird aus der bereits vorhandenen
`PitchedEventMultisetStable`-Evidenz abgeleitet. Fuer den finalen
v0.10.1611-Writerkorpus ist sie erwartungsgemaess `false`.

## Hash-Endbindung

Der Rust-Launcher ruft die Manifest-Endbindung nach dem Anhaengen seiner
`RUN-COMMAND-DIAGNOSTIC` nun immer auf:

- Bei ProgramExit 0 muss das erneuerte Manifest vollstaendig valide sein.
- Bei einem bereits fehlgeschlagenen Programmlauf wird die tatsaechliche
  Enddatei dennoch neu gebunden; eine weiterhin ungueltige fachliche
  Manifestprojektion wird nicht in einen Erfolg umgedeutet.

Damit kann eine Launcherdiagnose die bereits gebundene Berichtsdatei nicht
mehr unbemerkt veraendern.

## Erwartungsprojektion fuer Export 552

Die vorhandenen v0.10.1611-Projektberichte ergeben ohne neue Musikberechnung:

```text
RendererOutcomeObserved:                10/10
WriterOutcomeObserved:                  10/10
WriteProjects:                          7
RoutesWritten:                          12
ForwardNodesReplaced:                   10
ForwardDurationReplaced:                912
CrossStaffRestSuppressions:             15
SourceHigherBeamNotesRestored:           9
RestVerticalAnchorsWritten:              6
LedgerBoundedClefBoundariesWritten:      8
RenderNotationMutation:                 ja
SoundingEventMutation:                  nein
DeclaredMusicMutation:                  ja
MasterMusicXmlMutation:                 nein
```

Diese Zahlen sind eine Parser-Reproduktion aus den gelieferten
v0.10.1611-Berichten. Die autoritative v0.10.1612-Abnahme entsteht erst auf dem
Zielsystem.

## Unveraenderte Bereiche

Der Take aendert nicht:

- MusicXML- oder PDF-Erzeugung,
- Writerreihenfolge,
- Writerautoritaet,
- Kandidatenwahl,
- Takt-, Etueden-, Projekt-, Pitch- oder Locatorlogik,
- Schwellenwerte,
- MasterApply,
- Noten-, Onset-, Dauer-, Staff- oder Voice-Daten.
