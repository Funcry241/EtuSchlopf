# r992 NotationVoiceOccurrenceAttackMismatchDecomposition

## Parent und Klasse

r992 baut direkt auf dem real akzeptierten und dunkelgruenen r991 `NotationVoiceCanonicalOccurrenceIdentityProjection` auf. r991 lief mit 1.287 Strict-Tests und 10/10 Vollkorpus gruen. Der Take ist `RefactorOnly`, weil die bereits in r991 eingefuehrte read-only Deklassifizierung an der privaten `FinalPdfSourceOccurrenceContinuation` um zwei vorhandene Receipt-Felder erweitert wird. Es entsteht keine MusicMutation und kein Writer-, Repair-, Selection- oder Release-Recht.

## Anlass

r991 beseitigte die eigentliche Identitaetsunschaerfe: ueber 9.166 kanonisch gebundene Output-Occurrences blieben im Vollkorpus `UnresolvedCanonicalOccurrenceIdentity = 0`. Die 387 frueheren `UNRESOLVED_VOICE_FREE_SOURCE_MAPPING`-Faelle verschwanden aber nicht als musikalische Klasse. 386 Beam-Ketten wurden nun praeziser als `UNRESOLVED_OCCURRENCE_ATTACK_MISMATCH` abgewiesen, weil mindestens eine physische Output-Occurrence in Measure, Onset, Staff oder Pitch nicht exakt dem gefrorenen Sounding-Attack entsprach.

Damit ist die naechste Frage nicht mehr "wem gehoert diese Note?", sondern "warum unterscheidet sich die kanonisch besessene Output-Occurrence von ihrem urspruenglichen Attack?".

## Read-only Materialisierungsprojektion

`FinalPdfSourceOccurrenceContinuation::declassified_occurrence_identity_rows()` bleibt read-only und gibt zusaetzlich zur bereits deklassifizierten Identitaet nun auch

- `MaterializationKind` als stabilen lokalen Rank,
- `FragmentOrdinal`

zurueck.

Die drei vorhandenen Materialisierungsklassen bleiben semantisch unveraendert:

- `0 = SurvivingSourceNote`
- `1 = TiedAnchorChain`
- `2 = AuxiliaryLane`

Private `FragmentId`, Fingerprints, Omission-Proofs und Receipt-Konstruktion bleiben weiterhin privat. Die Projektion kann weder XML noch Receipt veraendern und erzeugt keine neue Autoritaet.

## Mismatch-Dekomposition

Der bestehende kausale NotationVoice-Audit behaelt seine r991-Entscheidungslogik unveraendert. Insbesondere bleibt jeder Occurrence/Attack-Widerspruch fail-closed `UNRESOLVED_OCCURRENCE_ATTACK_MISMATCH`.

r992 transportiert fuer diese bereits unresolved Klasse lediglich Diagnoseevidenz:

- beteiligte Materialisierungsklasse(n),
- beteiligte FragmentOrdinal-Werte,
- Anzahl physischer Mitglieder mit Abweichung,
- Measure-Delta,
- Onset-Delta,
- Staff-Delta,
- Pitch-Delta.

Der Report zaehlt ausserdem, wie viele Mismatch-Ketten

- `SurvivingSourceNote`, `TiedAnchorChain` oder `AuxiliaryLane` enthalten,
- mehrere Materialisierungsklassen enthalten,
- mindestens eine positive FragmentOrdinal besitzen,
- nur ein Onset-Delta besitzen,
- eine andere einzelne Dimension oder mehrere Dimensionen verletzen.

Diese Zaehler besitzen keinerlei Selection- oder Writer-Authority.

## Warum noch kein produktiver Merge

Ein spaeteres Tie-Fragment derselben kanonischen Attack-Identitaet kann erwartungsgemaess einen anderen physischen Onset besitzen. Daraus folgt aber noch nicht automatisch, dass die Sounding-Lane des urspruenglichen Attacks fuer die spaetere physische Occurrence als NotationVoice-Beweis wiederverwendet werden darf. r992 trennt deshalb zuerst legitime Rematerialisierung von Staff-/Pitch-/sonstiger Strukturabweichung, ohne irgendeinen dieser Faelle produktiv freizugeben.

Erst der reale Korpuslauf darf zeigen, ob die 386 Faelle ueberwiegend erwartbare Tie-/Fragment-Rematerialisierung oder heterogene Strukturabweichungen sind.

## Unveraendert

Unveraendert bleiben insbesondere:

- `FrozenSoundingTemporalLanePlan`,
- `FrozenSerializedVoicePlan`,
- aktuelle MusicXML-Voice-Werte,
- Beam-, Rest-, Clef- und Ottava-Writer,
- FinalChord-Projektionssemantik,
- MusicXML/PDF-Ausgabe,
- r991 `PROVEN_SAME_NOTATION_VOICE`-Entscheidungen.

## Tests

Gegen r991 kommen zwei generische Tests hinzu:

1. Ein `TiedAnchorChain`-Mismatch mit positiver FragmentOrdinal wird als Onset-only-Differenz im Report ausgewiesen.
2. Ein Pitch-Mismatch einer `SurvivingSourceNote` bleibt diagnostisch von einem Tie-Fragment getrennt.

Der bestehende Rematerialisierungs-Test wird um MaterializationKind/FragmentOrdinal-Evidenz erweitert. Ausgehend von r991 mit 1.287 Strict-Tests werden 1.289 Tests erwartet. Der reale Windows-Lauf entscheidet fail-closed ueber Acceptance.
