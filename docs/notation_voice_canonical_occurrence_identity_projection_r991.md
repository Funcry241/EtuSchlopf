# r991 NotationVoiceCanonicalOccurrenceIdentityProjection

## Parent und Klasse

r991 baut direkt auf dem real akzeptierten r990 `NativeInventoryArchitectureEvidenceClosure` auf. r990 lief mit 1.285 Strict-Tests und 10/10 Vollkorpus gruen. Die ARCH-2-/Inventory-Aenderungen aus r990 bleiben vollstaendig erhalten. Der Take ist `RefactorOnly`, weil er eine schmale read-only Projektion an der bestehenden StructuralWriteAuthority-Grenze `FinalPdfSourceOccurrenceContinuation` hinzufuegt. Es entsteht keine neue Musikmutation und kein neues Writer-, Repair-, Selection- oder Release-Recht.

Die zuvor lokal vorbereitete gleichnamige r990-Variante wurde nie installiert und ist kein Parent. Wegen der parallel akzeptierten r990-Inventory-Closure wird derselbe fachliche Identitaets-Schnitt hier konfliktfrei auf r990 rebasiert.

## Anlass

r989 bewies erstmals kausal, dass technische Sounding-Lanes und sichtbare Notationsstimmen nicht identisch sind. Von 367 Multi-Lane-Primary-Beam-Ketten wurden 177 als `PROVEN_SAME_NOTATION_VOICE` bestaetigt, darunter Etuede 6 Takt 1. Gleichzeitig blieben 387 Faelle `UNRESOLVED_VOICE_FREE_SOURCE_MAPPING`, weil der Audit finale physische Beam-Mitglieder nur ueber den rekonstruierten Locator `(measure,onset,staff,pitch)` an gefrorene Sounding-Attacks binden konnte.

Diese Unschaerfe ist kein Mangel der kanonischen Daten. FinalChord besitzt bereits eine transaction-lokale, bytegebundene Occurrence-Continuation. Fuer jede kanonisch besessene gepitchte Output-Occurrence enthaelt sie `SourceEventId`, `SoundingAttackId`, `measure` und `pitched_ordinal`. Diese Continuation wird durch die autorisierten finalen Notations- und committed Cross-Staff-Beam-Projektionen mit lokaler Driftpruefung transportiert.

## Read-only Deklassifizierung

r991 erweitert `FinalPdfSourceOccurrenceContinuation` genau um:

`declassified_occurrence_identity_rows()`

Die Projektion gibt ausschliesslich Tupel

`(SourceEventId, SoundingAttackId, measure, pitched_ordinal)`

zurueck. Private Receipt-Felder wie MaterializationKind, FragmentOrdinal, Fingerprints und Omission-Proof bleiben privat. Die Methode veraendert weder XML noch Receipt, trifft keine Auswahl und kann keine Mutation autorisieren.

Der Aufruf erfolgt im bestehenden Metric/Final-Notation-Orchestrator erst nach der committed Cross-Staff-Beam-Projektion und vor Horizontal-Layout. Zu diesem Zeitpunkt ist die Continuation bytegenau an die XML-Quelle gebunden, auf der auch der Audit seine Beam-Evidenz liest.

## Exakte gepitchte Occurrence-Identitaet

Der Beam-Audit zaehlt pro Measure einen `pitched_ordinal` ueber jede gepitchte Nicht-Rest-Note in physischer XML-Reihenfolge. Hidden Notes und Chord Members zaehlen mit, auch wenn sie selbst nicht als Mitglieder einer sichtbaren Primary-Beam-Kette qualifizieren. Das entspricht der FinalChord-Occurrence-Adressierung.

Ein Beam-Mitglied wird damit nicht mehr ueber Pitch/Onset erraten, sondern zuerst exakt ueber

`(measure,pitched_ordinal) -> (SourceEventId,SoundingAttackId)`

gebunden. Erst danach wird die bereits gefrorene Sounding-Lane-Evidenz ueber `(SourceEventId,SoundingAttackId)` gelesen.

Fehlt die kanonische Output-Occurrence oder ist sie mehrfach, bleibt der Fall `UNRESOLVED_CANONICAL_OCCURRENCE_IDENTITY`. Fehlt oder vervielfacht sich die gefrorene Attack-Identitaet, bleibt er `UNRESOLVED_SOURCE_ATTACK_IDENTITY`.

## Rematerialisierte Fragmente

Ein transaction-lokaler Output kann ein spaeteres Tie-/Rematerialisierungsfragment derselben Attack-Identitaet darstellen. Das darf nicht als neuer Lane-Wechsel am Fragment-Onset interpretiert werden. Deshalb verlangt r991 nach dem exakten Occurrence-Join zusaetzlich die Gleichheit von Measure, Attack-Onset, Staff und Pitch zwischen physischem Beam-Mitglied und gefrorenem Attack.

Bei Abweichung bleibt der Fall fail-closed `UNRESOLVED_OCCURRENCE_ATTACK_MISMATCH`.

## Kausaler Beweis bleibt unveraendert

Erst nach erfolgreicher kanonischer Bindung gilt weiterhin der r989-Beweis:

- die sichtbare Primary-Beam-Kette ist notatorisch lueckenlos;
- mindestens zwei technische Sounding-Lanes sind beteiligt;
- fuer jede benoetigte niedrigere Lane existiert am Attack-Start ein aktiver Blocker;
- jeder erforderliche Blocker gehoert zum selben Satz kanonisch gebundener SourceEvent-/Attack-Identitaeten.

Nur dann lautet die Auditentscheidung `PROVEN_SAME_NOTATION_VOICE`. Externe Blocker bleiben `UNRESOLVED_EXTERNAL_LANE_PRESSURE`. `technical_lane` ist kausale Evidenz, keine direkte Voice-Authority. Die aktuelle MusicXML-`<voice>` dient ausschliesslich der Diagnose.

## Keine produktive NotationVoice

r991 schreibt weiterhin keine NotationVoice. Insbesondere bleiben unveraendert:

- `FrozenSerializedVoicePlan` und seine Primary/Secondary-Rollen;
- Beam-, Rest-, Clef- und Ottava-Writer;
- FinalChord- und Sounding-Lane-Semantik;
- Master-/Render-MusicXML und PDF-Materialisierung.

Der Take verbessert ausschliesslich die Identitaetsqualitaet des read-only Beweises. Ein spaeteres Schreibrecht darf erst nach realem Korpuslauf und gemeinsamer Auswertung folgen.

## Tests

Gegen r990 kommen zwei generische Tests hinzu:

1. Primary-Beam-Evidenz zaehlt `pitched_ordinal` ueber hidden und chord pitched notes hinweg exakt wie die FinalChord-Occurrence-Adressierung.
2. Ein rematerialisiertes physisches Fragment, dessen Onset nicht dem kanonischen Attack-Onset entspricht, bleibt `UNRESOLVED_OCCURRENCE_ATTACK_MISMATCH`.

Der bestehende Kausaltest wird ausserdem auf die kanonische Occurrence-Identitaet umgestellt und beweist, dass ein gleichlautender Oberflaechen-Locator die Entscheidung nicht mehr mehrdeutig macht.

Ausgehend von r990 mit 1.285 Strict-Tests werden 1.287 Tests erwartet. Der reale Windows-Lauf entscheidet fail-closed ueber Acceptance.
