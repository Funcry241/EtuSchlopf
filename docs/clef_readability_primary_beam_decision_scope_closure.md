# ClefReadabilityPrimaryBeamDecisionScopeClosure

## Version

`v0.10.1796 r06`


## r05 -> r06: symmetrische Tie/Beam-Fixpunkt-Closure

Der reale r05-Lauf bestand erneut Strict-Clippy, Build und 128/128 Launcher-Tests, wurde aber beim Master-MusicXML von `clef course left a lower clef transition inside a primary beam group` fail-closed gestoppt. Das ist kein neuer Writer- oder Rendererfehler, sondern beweist eine noch asymmetrische StructuralAnchor-Komposition.

r05 schloss bereits die Richtung `Tie startet innerhalb eines offenen Primary-Beams`: Der Tie-Origin erbt dort den beam-geschlossenen Clef-Anchor. Die inverse Richtung blieb jedoch roh. Wenn `beam 1 begin` selbst auf einer gematchten Lower-Staff-Tie-Continuation liegt, schrieb `open_primary_beams` weiterhin den XML-Start dieser Continuation als Beam-Origin. Dieser Punkt liegt vor dem Tie-Stop derselben Note und damit definitionsgemaess noch innerhalb der offenen Tie. Spaetere Noten der Balkengruppe konnten deshalb einen Preference-Candidate an genau diesem unzulaessigen Punkt erzeugen.

r06 bestimmt beim Oeffnen einer Primaerbalkengruppe deshalb zuerst den bereits vorhandenen Tie-Origin der Begin-Note. Bei einer gematchten Lower-Tie-Continuation wird dieser Tie-Origin zum Beam-Origin; bei einem normalen oder upper/cross-staff Beam-Begin bleibt der bisherige Begin-Start erhalten. Eine ungematchte Lower-Tie-Continuation darf keinen schreibbaren Beam-Origin erzeugen. Stop+Start-Tie-Segmente speichern ebenfalls den bereits kombinierten Clef-Transition-Anchor weiter, sodass die Closure nicht erneut verloren gehen kann.

Damit gilt in beiden Richtungen:

`Tie-Origin <-> Beam-Origin -> gemeinsamer fruehester StructuralAnchor`

Die Rueckprobe auf dem realen r05-Render von Etuede 3 zaehlt 11 Register-Preference-Candidates. Mit r05 sind genau zwei Candidate-Anker noch innerhalb einer offenen Struktur: Takt 15/G2 und Takt 26/F4. Mit r06 bleiben 11 Candidates, aber 0 liegen innerhalb einer offenen Lower-Tie oder Primaerbalkengruppe. Die Kostenfunktion und Candidate-Anzahl werden also nicht kuenstlich reduziert; nur ihre strukturelle Zulassigkeit wird geschlossen.

Ein Regressionstest bildet den inversen Fall direkt ab: Eine hohe Lower-Staff-Tie beginnt vor der Balkengruppe, ihre Stop-Note ist zugleich `beam 1 begin`, ein entfernbarer G2-Surface liegt danach im Beam, und das Ende folgt innerhalb derselben Gruppe. Der Candidate muss am Tie-Start statt am Beam-Begin liegen; die Materialisierung muss den inneren Clef entfernen und die Postconditions passieren.


## Paket-Closure r01 -> r02

`r01` wurde nicht installiert. Der Take enthielt versehentlich einen Root-`Cargo.lock`, der nicht zur verwalteten eingehenden Vollbaumoberflaeche gehoert. Das Take-Tree-Integrity-Gate hat ihn deshalb vor jeder Installation korrekt fail-closed abgewiesen. `r02` entfernt ausschliesslich diesen unzulaessigen Paketpfad und traegt eine eindeutige Revisionsidentitaet; der fachliche Clef-DecisionScope-Kandidat und seine produktiven Sourcebytes bleiben gegenueber r01 unveraendert.


## r04 -> r05: transitive Tie/Beam-Anchor-Closure

Der reale r04-Einzellauf bestand Clippy, Build und 128/128 Launcher-Tests, brach aber im Master-MusicXML fail-closed an der neuen Postcondition `clef course left a lower clef transition inside a primary beam group` ab. Der Export beweist damit, dass die neue Invariante selbst korrekt griff, der DecisionScope aber noch nicht transitiv geschlossen war.

Die Ursache liegt in der Kopplung bereits vorhandener Tie- und Beam-Closure. Beginnt eine Tie auf einer Note innerhalb einer laufenden Primaerbalkengruppe, berechnet `scan_lower_pitches()` fuer genau diese Note bereits den beam-geschlossenen Clef-Transition-Anchor am `beam 1 begin`. Beim Oeffnen der Tie speicherte `open_ties` bislang jedoch wieder den rohen XML-Start der Tie-Start-Note. Spaetere Tie-Continuations erbten dadurch einen Anchor im Inneren der frueheren Balkengruppe. Das ist besonders in den realen Etuede-3-Strukturen um Takt 15/16 und 26 relevant; der frische Takt-25-Fall selbst benoetigt diese transitive Kombination nicht.

r05 aendert deshalb ausschliesslich die gespeicherte Tie-Origin-Wahrheit: Eine neu geoeffnete Tie speichert den bereits vollstaendig Tie-/Beam-geschlossenen `clef_transition_anchor` und faellt nur dann auf den bisherigen Note-Start zurueck, wenn kein solcher Anchor existiert. Damit bleibt der einmal geschlossene strukturelle Ursprung ueber die gesamte Tie-Chain erhalten. Kosten, Praeferenz, Beam-Tags, Tie-Tags, Writer und Materialisierer bleiben unveraendert.

Eine synthetische Regression bildet genau den Gegenbeweis: Eine tie-startende Low-Note liegt auf dem Ende einer Primaerbalkengruppe, deren Beginn durch hohes Material G2 bevorzugt. Die spaetere Tie-Continuation darf keinen zweiten F4-Candidate am rohen Tie-Start im Beam-Innenraum erzeugen; alle Candidates muessen ausserhalb offener Primaerbalkengruppen liegen.

## r03 -> r04: Boundary-Entscheidung statt Attack-Kollision

`r03` bestand real Strict-Clippy, Build und 128/128 Launcher-Tests, brach aber im Etuede-3-Programmlauf fail-closed ab und wurde transaktional auf 1795 zurueckgerollt. Die anschliessende statische Rueckpruefung des autoritativen Etuede-3-Master- und Render-MusicXML zeigt eine unvollstaendige Modellierung innerhalb derselben neuen Beam-Closure: In Takt 15 besitzen zwei Attack-Zustaende innerhalb einer einzigen Level-1-Balkengruppe gegensaetzliche Einzelpraeferenzen. Beide werden in r03 korrekt auf denselben `beam 1 begin`-Ursprung zurueckgebunden, bleiben danach aber zwei getrennte Transition-Candidates.

Damit ist die eigentliche Einheit der Entscheidung klar: Nicht jeder Attack innerhalb der geschlossenen Struktur entscheidet separat, sondern der gemeinsame strukturelle Anker. r04 gruppiert alle writer-frei beobachteten Attack-Pitches mit demselben Clef-Transition-Anchor, summiert darauf die bereits unveraenderten F4/G2-Ledgerkosten und erzeugt fuer diesen Anker hoechstens einen Preference-Candidate. Tie- und Beam-Anker teilen damit dieselbe Boundary-Semantik. Unmatched Continuations bleiben weiterhin ohne schreibbaren Anker.

Das ist keine neue Kostenpolicy: dieselbe Ledger-Pressure-Funktion wird lediglich einmal auf den vollstaendigen bereits gebundenen DecisionScope der Boundary angewendet statt mehrfach auf Teilangriffe, die anschliessend auf dieselbe Boundary kollabieren.

Der autoritative Stand v0.10.1795 r01 ist 10/10 dunkelgruen und besitzt bewusst kein automatisch gewaehltetes Reparaturziel. Die frische Sichtpruefung von Etuede 3, Takt 25, untere Hand zeigt jedoch ein neues konkretes Gegenbeispiel: Der produktive ClefReadabilityPrimaryCourse setzt den Wechsel F4 -> G2 innerhalb einer bereits laufenden Level-1-Balkengruppe. Im aktuellen Render steht vor dem letzten Beat zunaechst ein Cis4-Sechzehntel mit `beam 1 begin`; erst vor dem folgenden Fis4-Sechzehntel mit `beam 1 continue` wird der G2-Clef materialisiert. Die musikalisch zusammengehoerige Balkengruppe wird dadurch visuell von einem Schluesselwechsel durchschnitten, obwohl der aktuelle Postwrite-Audit den Kurs als bereits bevorzugt (`Decision KEEP`) bewertet.

Das ist kein Layout-Reparaturfall und kein Anlass fuer einen spaeten Clef-Fix. Der bestehende Clef-Owner hat seine Register-, Tie-Chain- und Kostenwahrheit, aber sein DecisionScope kennt die rhythmische Grenze der bereits autoritativ materialisierten Primaerbalkengruppe noch nicht.

## Ursache

`collect_readability_transition_candidates()` leitet Kandidaten aus maximalen Registerpraeferenz-Runs ab. Seit der Tie-Chain-Closure kann ein Kandidat auf den Ursprung einer gebundenen Fortsetzung zurueckankern. Fuer Balkengruppen wurde dagegen bislang nur der aktuelle Attack-Start verwendet. Beginnt eine neue Registerpraeferenz auf einem `beam 1 continue`, darf der Solver deshalb exakt dort einen Clef-Surface behalten oder neu einsetzen.

Gleichzeitig durfte ein bereits vorhandener entfernbarer Clef-Surface innerhalb einer offenen Primaerbalkengruppe vom Course-Solver weiterhin als gueltige `KEEP`-Alternative bewertet werden. Der Postwrite-Invariant pruefte nur offene Tie-Chains.

## Closure

v0.10.1796 erweitert ausschliesslich den DecisionScope des bestehenden `ClefReadabilityPrimaryCourse`:

1. Die writer-freie Pitch-/Attack-Beobachtung verfolgt pro Voice den Ursprung einer sichtbaren Level-1-Balkengruppe. Cross-Staff-Verlaeufe derselben Voice werden dabei mitbeobachtet; der Clef-Owner schreibt dadurch keine Beam-Tags und erfindet keine neue Rhythmuswahrheit.
2. Beginnt Registermaterial auf `beam 1 continue` oder `beam 1 end`, wird sein vorhandener Transition-Anchor auf den zugehoerigen `beam 1 begin`-Attack zurueckgebunden. Alle Attack-Pitches, die dadurch denselben strukturellen Anchor teilen, werden vor der Praeferenzentscheidung zu genau einer Boundary-Gruppe zusammengefasst; pro Anchor kann deshalb hoechstens ein F4/G2-Candidate entstehen. Ein Fortsetzungszustand ohne beobachtbaren Gruppenanfang bleibt fail-closed kein schreibbarer Clef-Anker.
3. Ein sichtbarer entfernbarer Lower-Clef-Surface innerhalb einer laufenden Primaerbalkengruppe darf nicht mehr in situ retained werden. Der bestehende Solver kann ihn nur auslassen und, falls seine unveraenderten Lesbarkeitskosten das verlangen, an der gruppengeschlossenen Grenze neu einsetzen.
4. `normalize_clef_readability_insertions()` verweigert weiterhin jeden nicht strukturell gebundenen Insert und nun zusaetzlich Inserts im Inneren einer Level-1-Balkengruppe.
5. Nach Materialisierung gilt neben der vorhandenen Tie-Chain-Invariante nun fail-closed: kein sichtbarer Lower-Clef-Surface innerhalb einer laufenden Primaerbalkengruppe.

Es gibt keinen neuen Writer, keinen neuen Checkpoint, keine neue Repair-/Recovery-Authority, keine neue Kostenfunktion und keinen Etueden-/Takt-/Pitch-Selektor. Die bestehende Structural Write Authority materialisiert weiterhin ausschliesslich den vom gleichen ClefReadability-Owner geplanten Kurs.

## Invarianten

- Pitch unveraendert,
- Onset unveraendert,
- Duration unveraendert,
- Staff/Voice unveraendert,
- Beam-Tags unveraendert,
- Rest-/Artikulations-/Octave-Shift-Semantik unveraendert,
- Clef-Kosten und ShortExcursion-Gewichtung unveraendert,
- Tie-Chain-Closure bleibt erhalten,
- kein neuer produktiver Writer,
- kein Repair/Recovery,
- kein projekt- oder taktidentitaetsbasierter Sonderfall.

Zwei synthetische Regressionstests sichern die neue Grenze: Ein innerhalb einer laufenden Primaerbalkengruppe stehender G2-Surface wird an den `beam 1 begin`-Attack zurueckgebunden; ein ungematchtes `beam 1 continue` erzeugt keinen neuen Clef-Anker.

## Erste Abnahme

```powershell
.\run.cmd --take --strict --writer-trace=25 "Etüde Nr. 3"
```

Erwartung fuer Takt 25, untere Hand: Kein sichtbarer Clef-Wechsel verbleibt zwischen `beam 1 begin` und dem zugehoerigen `beam 1 end`. Falls der unveraenderte Lesbarkeits-Solver G2 fuer diesen Run weiterhin bevorzugt, liegt der Wechsel vor dem Gruppenanfang; andernfalls wird er ausgelassen. Pitch-/Onset-/Duration-/Staff-/Voice- und Beam-Semantik muessen stabil bleiben. Anschliessend Etuede 3 insgesamt visuell pruefen; erst danach Vollkorpus.
