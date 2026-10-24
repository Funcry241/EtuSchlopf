# ChordAnchoredRestSpanSafetyClosure

Stand: v0.10.1749 r02

## Revisionshistorie

`r01` wurde nicht installiert. Das zuerst gelieferte r01-Paket wurde vom Take-Tree-Integrity-Gate wegen eines unzulaessigen Root-`Cargo.lock` korrekt fail-closed abgewiesen. Ein danach nur strukturell korrigiertes Paket mit derselben Revision fuehrte bei gleichzeitig vorhandenen r01-ZIPs erwartungsgemaess zur Same-Version-/Same-Revision-Ambiguitaet. `r02` enthaelt deshalb denselben fachlichen 1749-Source-Payload mit eindeutiger Revisionsidentitaet; keine Writer-, Gate-, Musik- oder Capability-Aenderung wird gegenueber dem korrigierten r01-Payload hinzugefuegt.

## Anlass

v0.10.1748 r02 passierte die formalen Gates und den Einzelprojektlauf dunkelgruen. Der anschliessende 1747->1748-Endvergleich der Render-MusicXML zeigte jedoch eine semantische Beweisluecke in Etuede 1, Takt 17 oben. Die Koordinate ist ausschliesslich Regressionadresse.

Der sichtbare Rest-Coalescer interpretierte zwei aufeinanderfolgende obere Pausen als Anfangsstille, obwohl an diesen Pausenankern klingende, aber `print-object="no"` versteckte `<chord/>`-Events derselben Voice haengen. In MusicXML erbt ein Chord-Event den Onset seines vorangehenden Nicht-Chord-Ankers. Wird der zweite Restanker entfernt, kann deshalb ein daran gebundenes klingendes Chord-Event seinen Onset veraendern, obwohl Pitch, Dauer und Anzahl der Noten unveraendert bleiben.

Im beobachteten Gegenbeispiel wurde ein sichtbares Fis-Chord-Event von Onset 96 auf Onset 0 umgeankert. Damit ist v0.10.1748 r02 trotz gruenem Prozesslauf keine musikalisch autoritative Basis fuer einen Korpuslauf.

## Architekturkorrektur

Der bestehende Writer `apply_upper_staff_initial_rest_and_beat_local_coalescing` behaelt Symbol, WriterReachability und Structural-Write-Authority unveraendert.

Die Zulassung einer beatuebergreifenden Anfangspausen-Konsolidierung verlangt jetzt zusaetzlich einen echten Voice-Stille-Beweis fuer die gesamte Paarspanne:

- gleiche Voice der beiden sichtbaren Restanker;
- zusammenhaengende Zeitspanne;
- Beginn bei Taktzeit 0;
- `MetricRestNotation` plant genau einen Restwert fuer die Gesamtspanne;
- im halboffenen Intervall `[start, end)` liegt kein gepitchtes/klingendes Event derselben Voice - auch kein hidden Chord-Event;
- die Staff-Zuordnung dieses klingenden Events ist dabei absichtlich irrelevant, damit Cross-Staff-Aktivitaet ebenfalls die Behauptung "Voice-Stille" widerlegt.

Damit gilt erneut strikt:

`Staff-Leere != Voice-Stille != sichtbare Pausenglyphe`.

Chord-Events und hidden pitched Events werden nicht mehr aus dem Stillebeweis ausgeblendet. Ein Rest, der als MusicXML-Anker fuer klingendes Material dient, darf nicht als frei verschmelzbarer Stillespann behandelt werden. Zusaetzlich vergleicht der bestehende Local-Visual-Rest-Trial vor einer Uebernahme jetzt alle gepitchten Events inklusive hidden Material als sortiertes Ereignis-Multiset aus Scope, Staff, Voice, Onset, Dauer, Hidden-Status, Chord-Status und MIDI. Damit ist die bisher nur sichtbare `sounding_signature` fuer diesen Writer nicht mehr der alleinige Musikbeweis.

## Regressionstests

Zwei lokale Tests binden den Vertrag an die konkrete Semantik statt an eine Etueden- oder Taktidentitaet:

1. Zwei echte Viertelpausen am Taktanfang vor dem ersten klingenden Voice-Event duerfen bei metrischem Beweis zu einer Halben Pause werden.
2. Dieselbe Restfolge mit `print-object="no"` chord-gebundenen gepitchten Events an den Restankern darf nicht konsolidiert werden und muss bytegleich bleiben.

## Invarianten

Unveraendert bleiben muessen alle klingenden Pitch-, Onset-, Dauer-, Staff- und Voice-Ereignisse, die Takt-/Voice-Timeline-Abdeckung, Backup-/Forward-Balance, Cross-Staff-Semantik, Ganztaktpausen-Ownership, vertikale Restposition und alle bestehenden Writerwurzeln.

Der 1749-Einzelprojektbeweis muss insbesondere zeigen:

- Etuede 1, Takt 2 bleibt `192+96+48+24+24`;
- der unsichere Takt-17-Coalesce aus 1748 findet nicht mehr statt;
- 1748->1749 darf den von 1748 verursachten Chord-Onset-Delta rueckgaengig machen, danach muss der finale 1749-Render im klingenden Ereignisraum wieder mit dem autoritativen 1747-Stand uebereinstimmen;
- erst danach ist ein Vollkorpuslauf wieder zulaessig.
