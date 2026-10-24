# r966 – ExactMoveBoundExpectedTransition

Parent: `v0.10.1839 r965 ArchitectureInventoryAssistantFastPathV1`

ChangeClass: `RefactorOnly`

## Zweck

r966 ist der erste enge Abbauversuch aus der r965-Architektur-Inventur. Er veraendert weder die bestehende Gesture-Ankerwahl noch Gauss-/BrokenGesture-/ConstrainedTail-Scoring, Ranking, Candidate-Auswahl oder XML-Writer-Primitiven.

Der Cut betrifft ausschliesslich bereits ausgewaehlte `CandidateMaterialExactNoteMoveCarrier`-Moves.

Vor r966 gilt fuer diese Moves sinngemaess:

```text
Anker gewaehlt
→ XML materialisiert
→ XML erneut geparst
→ onset/staff/voice aus dem Output erneut in CandidateMaterialAttackState uebernommen
```

r966 ersetzt fuer ExactMove genau diese produktive Rueckuebernahme durch:

```text
Anker gewaehlt
→ Quellfragment + konkretes Ankerfragment gegen den gebundenen Ausgangszustand binden
→ erwarteten Attack-Uebergang bestimmen
→ XML materialisieren
→ XML unabhaengig zuruecklesen
→ observed gegen expected pruefen
→ gebundenen expected Attack-State uebernehmen
```

Damit wird der nachgelagerte XML-Readback fuer bereits vor dem Write bestimmbare Attack-Semantik vom produktiven State-Producer zum Witness.

## Gebundener Zielzustand

Der bestehende ExactMove-Carrier traegt zusaetzlich ausschliesslich:

- die Byte-Spanne des bereits gewaehlten Ankerfragments im Ausgangs-XML;
- die bereits vom Writer verwendete `anchor.duration.max(1)` als erwartete Fragmentdauer.

Der Ziel-Onset wird **nicht** aus `anchor.start_tick` und **nicht** pauschal aus dem SoundingAttack des Ankers abgeleitet. Er stammt aus dem gegen dieselbe Vorher-Epoche gebundenen konkreten `CandidateMaterialFragmentState` des Ankers.

Aus diesem gebundenen Ankerfragment stammen fuer den erwarteten Attack-Uebergang:

- Measure;
- Onset;
- Staff;
- Voice.

Vom Quell-Attack erhalten bleiben:

- `SourceEventId`;
- `SoundingAttackId`;
- Existenz;
- Pitch;
- positive klingende Dauer.

`expected_sounding_end = expected_onset + preserved_sounding_duration`.

Die Representation-Fragmentdauer bleibt unveraendert die bestehende Writer-Regel `anchor.duration.max(1)` und wird nicht mit der klingenden Dauer gleichgesetzt.

## Unabhaengiger XML-Witness

Nach der Materialisierung muss das tatsaechlich geparste moved Fragment exakt den gebundenen Expected-Transition-Werten entsprechen:

- Measure;
- Onset;
- Fragmentdauer;
- Pitch;
- Staff;
- Voice;
- sichtbar;
- `<chord/>` vorhanden.

Tie-Rollen und Pitch-Erhalt bleiben zusaetzlich durch die vorhandene explizite Fragment-Pair-Pruefung fail-closed gebunden. Eine zeitliche Verschiebung eines `tie_start`-/`tie_stop`-Fragments bleibt verboten.

Erst nach erfolgreichem Soll/Ist-Vergleich wird der bereits vor dem Write berechnete Expected-Attack-State in die CandidateMaterialLineage uebernommen. Andere Owner benutzen weiterhin die unveraenderte generische Reconciliation.

## Explizit unveraendert

- `collect_visible_gesture_attack_slots`
- `anchor.start_tick`
- BrokenGesture-Scoring
- Gauss-Scoring
- GaussChain-Logik
- ConstrainedTail-Logik
- Candidate Ranking
- Candidate Acceptance Policy
- Anchor Selection
- `consensus_note_block_as_visible_chord` Writer-Semantik
- StructuralWriteAuthority-Primitiven
- AIC-1
- AIF-1
- CMS-1
- Architecture Essence

Es gibt kein `SemanticTransitionFramework`, keine Registry, kein neues Ledger und keinen neuen Writer.

## Baseline-Voraussetzung

Der Kandidat wurde gegen den r965 Physical-Source-Snapshot mit SHA-256

`84b26480fcd2544eabcbe366d32bd468fa03760df1a82d90a5bcfd148fdf7caf`

vorbereitet.

Vor Installation muss auf dem real installierten r965-Parent zuerst der angeforderte frische Vollkorpuslauf (`run.cmd --all`) als M0 gesichert werden. Diese Voraussetzung wird nicht durch ein neues Produkt-Gate dupliziert; sie ist Experimentdisziplin fuer den ersten Architekturabbau.

Nach Installation sind insbesondere Candidate-Erzeugung, Acceptance/Rejection, Winner, Material-State-Fingerprints, ExactMove-Carriers, `consensus_relocation_longitudinal_trace`, finale MusicXML-/PDF-Artefakte sowie CMS/AIC gegen M0 zu vergleichen.

## Erfolgskriterium

r966 ist nur dann ein erfolgreicher Architekturabbau, wenn der reale Lauf bestaetigt:

1. ExactMove uebernimmt bereits vorher bestimmbare Attack-Semantik nicht mehr produktiv aus XML;
2. der XML-Readback bleibt als unabhaengiger Witness erhalten;
3. Candidate- und Musikverhalten bleiben gleich oder jede Abweichung wird explizit als zuvor verdeckte Inkonsistenz ausgewiesen;
4. kein Scope-Ausbruch in Ankerwahl/Scoring/Heuristik ist erforderlich.

Bei unerwartetem Candidate-Delta oder Scope-Ausbruch keine Take-Serie anschliessen, sondern Hypothese neu bewerten.
