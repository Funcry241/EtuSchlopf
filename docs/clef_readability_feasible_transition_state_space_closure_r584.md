# r584 ClefReadabilityFeasibleTransitionStateSpaceClosure

## Ausgangsbefund

Der isolierte Etüde-1-Lauf auf r578 scheitert fail-closed vor MuseScore in der Primary-Clef-Projektion. Die gezielte r583-Diagnose hat den Planverlust eindeutig lokalisiert:

- `RemovalNormalizationGreen yes`
- `InsertionNormalizationGreen no`
- die jeweils abgelehnte Insertion liegt innerhalb einer offenen Primary-Beam-Gruppe
- `octave_boundary_precedes=no`

Für den stabil reproduzierten Takt-12-Fall wird dadurch ein ansonsten gültiger Plan mit entfernbaren Clef-Surfaces vollständig verworfen; die finale Postcondition findet anschließend weiterhin einen Lower-Clef-Übergang innerhalb der offenen Primärbalkengruppe und blockiert die Render-Finalisierung.

## Fehlerklasse

Die Clef-Readability-DP darf eine Transition nur dann als auswählbare Zustandskante behandeln, wenn dieselbe StructuralScope-Bedingung erfüllt ist, die bereits für sichtbare Clef-Surfaces und die finale Postcondition gilt.

Vor r584 wurden Candidate-Transitionen im DP ohne diese StructuralScope-Prüfung bewertet. Erst die nachgelagerte Insertions-Normalisierung erkannte die unzulässige Position. Da die Normalisierung fail-closed den Gesamtplan verwirft, konnte eine einzelne unzulässige Transition auch sichere Removals vernichten.

## Änderung

r584 bindet die bereits vorhandene StructuralScope-Regel direkt an die Candidate-Transition im bestehenden `solve_readability_course`:

- offene Lower-Staff-Tie-Struktur => Transition nicht auswählbar,
- offene Primary-Beam-Gruppe => Transition nicht auswählbar,
- außer die Position ist bereits eine vorhandene Lower-Staff-Octave-Shift-Grenze.

Der Omit-Pfad der DP bleibt immer zulässig. Es wird keine Transition verschoben, keine neue Candidate-Quelle erzeugt und keine Kostenfunktion verändert.

Die bestehende Insertions-Normalisierung bleibt unverändert als fail-closed Backstop. Die finale Tie-/Beam-Postcondition bleibt ebenfalls unverändert.

## Regression

Ein neuer synthetischer Cross-Voice-Test erzeugt bewusst eine Readability-Transition innerhalb einer fremden, noch offenen Primary-Beam-Gruppe. Der rohe Readability-Candidate existiert weiterhin, darf aber vom DP nicht mehr als materialisierte Clef-Insertion gewählt werden.

Damit wird die Fehlerklasse geprüft und nicht Etüde 1 oder Takt 12 selektiert.

## Nicht geändert

Keine Änderung an:

- RegisterNotation,
- Clef-Readability-Kosten oder Gewichtungen,
- Candidate-Erzeugung,
- Removal-Normalisierung,
- Insertions-Normalisierung,
- Tie-/Beam-Postconditions,
- StructuralWriteAuthority-Grenzen,
- Renderer,
- SchemaSchlopf oder XSD,
- Pitch, Onset, Duration, Voice, Beam oder Tie.

Es gibt keinen Repair-Layer und keine Gate-Lockerung.
