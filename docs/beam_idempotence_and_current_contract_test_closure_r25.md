# Beam Idempotence and Current-Contract Test Closure r25

Parent: `0.10.1839 r23`
ChangeClass: `MusicMutation`

## Zweck

Die auf r23 erneut eingeblendete All-Targets-Evidenz reduziert den verbliebenen
roten Main-Testbestand auf klar trennbare Ursachen. r25 schliesst nur die zwei
produktiven Beam-Idempotenzfehler und synchronisiert die eindeutig historischen
Test-Fixtures mit bereits anderweitig gebundenen heutigen Vertraegen. Die
Security-/Launcher-All-Targets-Haertung bleibt weiterhin ein separater Take und
wird nicht mit dieser MusicMutation vermischt.

## Produktive Correctness-Fixes

1. **Composer-Beam-Normalisierung**
   Die Entscheidung, ob eine strukturelle Beam-Gruppe bereits normalisiert ist,
   verglich bislang rekonstruiertes XML byteweise mit dem vorhandenen Notentext.
   Semantisch identische `begin/continue/end`-Beams in anderer Whitespace-Form
   wurden dadurch erneut als schreibbeduerftig klassifiziert. r25 liest die
   vorhandenen Beam-Level/-States und vergleicht sie mit dem typisierten Plan.
   Nur semantische Abweichung autorisiert die bestehende Materialisierung.

2. **Notation-Polish-Idempotenz**
   Das Entfernen bereits materialisierter Beam-Zeilen liess deren reine
   Einrueckungszeile stehen. Beim anschliessenden kanonischen Wiedereinfuegen
   konnte dadurch pro Durchlauf Whitespace akkumulieren. Beam-Tags, die allein
   auf einer Zeile stehen, werden nun zusammen mit genau dieser Beam-eigenen
   Whitespace-Zeile entfernt; Inline-Beams verlieren weiterhin nur den Tag.

## Testvertrag-Synchronisation ohne neue Musikpolicy

- Bassschluessel-Ganzrest: der heutige Gegen-Schluessel-Anker ist `B4`, der
  kanonische Bassanker `D3`; zwei alte `D5`-Fixtures werden daran gebunden.
- Zwei direkt angrenzende erlaubte Structural-Deletion-Ranges werden von der
  Authority absichtlich zu einer gebundenen Transaktion gemerged; der Test
  erwartet deshalb einen Writer-Commit statt zwei Einzelcommits.
- Der aktuelle Clef-Cost-Vertrag vermeidet einen zusaetzlichen Ruecksprung in
  den Bassschluessel fuer nur einen tiefen Schlussangriff. Drei Tests werden
  auf `F4>G2` statt `F4>G2>F4` gebunden.
- Die beiden Tie-Boundary-Tests verwenden nun einen eindeutig treble-wuerdigen
  gebundenen Hochregister-Angriff (`B5` mit hohem Akkord), sodass sie weiterhin
  exakt ihre eigentliche Invariante pruefen: eine notwendige Clef-Transition
  wird vor die offene Tie-Kette reankert und niemals in ihr materialisiert.

## Nicht geaendert

Keine neue Clef-Kostenregel, keine Rest-Placement-Regel, keine Layout-Policy,
kein Security-Gate, kein Launcher-Testgate und kein neuer Writer. r23 bleibt die
produktive Korpus-Baseline; r25 soll zunaechst ueber einen bekannten gruenen
Einzellauf committed und danach korpusweit regressiert werden.
