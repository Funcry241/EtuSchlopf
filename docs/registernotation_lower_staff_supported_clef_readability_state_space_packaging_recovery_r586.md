# r586 RegisterNotationLowerStaffSupportedClefReadabilityStateSpacePackagingRecovery

## Autoritative Basis

r586 basiert direkt auf dem installierten, dunkelgruenen r584 `ClefReadabilityFeasibleTransitionStateSpaceClosure`.

Der zuvor gelieferte r585-Kandidat `RegisterNotationLowerStaffSupportedClefReadabilityStateSpaceClosure` wurde vor jeder Installation vom Take-Tree-Integrity-Gate fail-closed abgewiesen, weil das eingehende ZIP versehentlich den lokal-transaktionalen Root-Pfad `Cargo.lock` enthielt. r585 ist damit keine Zwischenautoritaet und darf nicht als Parent verwendet werden.

r586 uebernimmt den fachlichen r585-Source-Payload bytegleich in `src/registernotation/mod.rs`, korrigiert aber die Paketstruktur auf den bereits in r584 geltenden Managed-Tree-Vertrag. Der Root-`Cargo.lock` ist nicht TakeManaged und ist im r586-Take nicht enthalten.

## Ausgangsbefund

Der vollstaendige r584-Korpus ist technisch dunkelgruen. Der anschliessende editorische Literatur- und Sichtaudit hat jedoch einen generischen RegisterNotation-Grenzfall sichtbar gemacht: Im unteren System kann eine Oktavspanne gewinnen, obwohl dieselbe klingende Passage in einem fuer Klavier bereits unterstuetzten Violinschluesselzustand ohne entsprechende Registerlast lesbar waere.

Der klarste reproduzierbare Korpusfall ist Etuede 8, Takt 31 unten. Dort bewertet RegisterNotation die Loco-Darstellung vor r586 ausschliesslich gegen die feste untere Systemzone MIDI 36..60. Die spaetere Clef-Readability-Planung verwendet fuer diese Passage dagegen einen Violinschluessel. Dadurch entsteht ein kuenstlicher Oktavierungsgewinn: Staff-Identitaet wird mit Clef-Lesbarkeit gleichgesetzt.

## Fehlerklasse

RegisterNotation besitzt vor der finalen Clef-Planung keine zulaessige Abhaengigkeit von einem spaeter materialisierten Schluessel. Eine Ruecklesung des finalen Clef-Kurses wuerde die Pass-Reihenfolge zyklisch machen, weil ClefReadability seinerseits die bereits materialisierte Oktavdarstellung bewertet.

Die vorhandene Lower-Staff-Sonderbehandlung ab MIDI 69 loest diese Eigentumsfrage nur als starres Veto. Sie kann weder ein vollstaendiges zusammenhaengendes Fenster bewerten noch extrem hohes Material korrekt gegen eine weiterhin notwendige Oktavdarstellung abwaegen.

## Fachliche Aenderung

r586 erweitert ausschliesslich den bestehenden RegisterNotation-Lesbarkeitszustandsraum des unteren Systems:

- Fuer jedes vollstaendige Kandidatenfenster werden die bereits vorhandenen Registerzonen 36..60 und 60..84 getrennt aufsummiert.
- Als Loco-Kosten gelten die guenstigeren Gesamtkosten eines einzigen dieser beiden Zustaende fuer das gesamte Fenster.
- Dieselbe ganze-Fenster-Gegenrechnung wird fuer jede 8va/15ma/8vb/15mb-Darstellung durchgefuehrt.
- Es gibt ausdruecklich keine tonweise Minimum-Bildung; ein Fenster erhaelt daher keinen kostenlosen Schluesselwechsel pro Ton.
- Die bisherige feste MIDI-69-Vetoschwelle entfaellt. Extrem hohes Lower-Staff-Material kann weiterhin eine Oktavdarstellung gewinnen, wenn auch die unterstuetzte hoehere Registerzone nicht ausreichend lesbar ist.
- Die bestehende Verbesserungsschwelle, die quadratische Registerkostenformel, die 15ma/15mb-Bezeichnungsstrafe, die r578-Spanzuschnittslogik und alle strukturellen Grenzen bleiben unveraendert.

Damit wird nicht der spaetere Clef-Kurs vorweggenommen. RegisterNotation vergleicht nur die zwei bereits im System vorhandenen, fuer das untere Klaviersystem grundsaetzlich unterstuetzten Lesbarkeitszustaende als feasible alternatives.

## Regression

Die im produktiven Source-Payload enthaltenen generischen Tests belegen:

- E8-T31-artiges Lower-Staff-Material `[68, 67, 67, 64]` wird nicht mehr kuenstlich mit 8va versehen.
- Extrem hohes Lower-Staff-Material `[96, 97, 98, 99]` darf weiterhin 8va waehlen.
- Ein gemischtes Fenster `[48, 72, 48, 72]` wird als Ganzes bewertet; eine tonweise freie Clef-Wahl ist ausgeschlossen.
- Auch der optimale r578-Spanplaner verwendet dieselbe ganze-Fenster-Semantik.
- Bestehende Upper-Staff- und tiefe Lower-Staff-Entscheidungen behalten ihre bisherige Kostenbasis.

## Nicht geaendert

Keine Aenderung an:

- ClefReadability-DP oder r584 StructuralScope-Closure,
- finaler Clef-Materialisierung,
- Pass-Reihenfolge oder Writer-Authority,
- Verbesserungsschwelle oder Registergewichten,
- r578-Grenz- und Tie-Break-Reihenfolge,
- Pitch, Onset, Duration, Voice, Beam, Tie oder Slur,
- Renderer,
- SchemaSchlopf oder XSD,
- Repair- oder Postprocessing-Schichten.

Es gibt keine Etueden-, Takt-, Tonhoehen- oder Hilfslinien-Sonderregel.

## Packaging-Recovery

Der einzige Unterschied zwischen dem abgewiesenen r585-Kandidaten und dem produktiven fachlichen r586-Source-Payload ist die korrekte Kandidatenidentitaet und Paketstruktur: r586 ist eine neue Revision direkt auf r584, dokumentiert die r585-Ablehnung und enthaelt keinen Root-`Cargo.lock`. Der produktive `src/registernotation/mod.rs`-Payload bleibt bytegleich zum abgewiesenen r585-Kandidaten.
