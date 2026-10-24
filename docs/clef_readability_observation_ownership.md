# ClefReadabilityObservationOwnership

## Scope

v0.10.1708 ist Take 1 des vierteiligen Clef-Readability-Blocks. Der Take beantwortet ausschließlich die Frage **„Was beobachten wir?“**. Er führt keine neue Clef-Policy, keine neue Kursentscheidung und keine MusicXML-Mutation ein.

## Gemeinsamer Observation Owner

Neu ist `src/clef_readability_observation.rs`. Die Datei besitzt nur writer-freie Fakten:

- `ClefPitchObservation`: diatonische Pitchposition einer beobachteten MusicXML-Note.
- `PianoClef`: die für den aktuellen Lower-Staff-Kurs relevanten kanonischen Clefs G2 und F4.
- `PianoClef::ledger_lines(...)`: reine Notationsgeometrie, also wie viele Hilfslinien ein beobachteter Pitch in G2/F4 benötigt.
- `observe_lower_staff_octave_display_shift(...)`: beobachteter Staff-2-Oktavdisplayzustand in diatonischen Schritten.

Diese Fakten wurden zuvor in `notation.rs` und `structural_write_authority/rustig_clef_planning.rs` getrennt rekonstruiert.

## Bewusst nicht verschoben

Folgende Semantik bleibt unverändert bei ihren bisherigen Eigentümern:

- lokale `>3`-Hilfslinienentscheidung,
- `SUSTAINED_CLEF_*`-Grenzen,
- `clef_course_pressure(...)`,
- Rustig `CLEF_CHANGE_COST` und `REDUNDANT_CLEF_COST`,
- Hysterese und Surface-Policy,
- A4-basierte frühe Clef-Erzeugung,
- Clef-Materialisierung und Structural Write Authority.

Damit gilt weiterhin: Hilfsliniengeometrie ist Observation; die Frage, wie teuer Hilfslinien gegenüber Schlüsselwechseln sind, ist Policy und bleibt für Take 2 reserviert.

## Golden Control

B4 wird im gemeinsamen Observation Owner weiterhin geometrisch als

- F4/Bassschlüssel: 4 Hilfslinien,
- G2/Violinschlüssel: 0 Hilfslinien

beobachtet. Daraus folgt in Take 1 ausdrücklich **keine** neue Entscheidung. Der reale Kontrollfall Etüde 6, Takt 32 unten muss gegenüber v0.10.1707 weiterhin denselben Clef-Verlauf F → G → F besitzen.

## Write- und Musikneutralität

Der neue Owner enthält keine XML-Ersetzung, keine Dateioperation, keine Materialisierung, keinen Candidate-Score und keine autorisierte Write Capability. `notation.rs` und Rustig konsumieren nur dieselben beobachteten Fakten; ihre bisherigen Entscheidungen und Writer bleiben unverändert.

## Take-2-Grenze

Erst `ClefReadabilityShadowCoursePlanning` darf die editorische Frage untersuchen, ob beispielsweise vier Hilfslinien für einen sehr kurzen Ausflug weniger Lesekosten verursachen als zwei zusätzliche Schlüsselwechsel. Diese Policy ist absichtlich nicht Bestandteil dieses Takes.

## Statischer Extraktionsbeweis vor Packaging

Der vorhandene v0.10.1707-Assistant-Export wurde für die Observation-Äquivalenz vollständig über Master- und Render-MusicXML ausgewertet:

- 20 MusicXML-Dateien,
- 21.013 Note-Fragmente,
- 819 Direction-Fragmente,
- 0 Abweichungen der diatonischen Pitchbeobachtung zwischen alter Notation-, alter Rustig- und neuer gemeinsamer Semantik,
- 0 Abweichungen der Staff-2-Oktavdisplay-Beobachtung.

Die physische Rust-Zeilenbilanz des Source-Baums vor Packaging ist 118.202 → 118.244, also netto +42 Zeilen. Take 1 optimiert bewusst Ownership statt Zeilenzahl; der verpflichtende Netto-Rückbau ist für Take 4 reserviert.
