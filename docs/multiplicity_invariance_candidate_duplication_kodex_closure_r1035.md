# r1035 - MultiplicityInvarianceCandidateDuplicationKodexClosure

## Zweck

r1035 schliesst die nach r1034 sichtbar gewordene zweite Proxy-Identity-Luecke. `causal_class_first_v1` verhindert target-conditioned Merkmalsverengung bei **neuer** Selector-Logik, aber vorhandene und neue Regeln koennen trotzdem eine an sich allgemeine Kandidatenklasse ueber scoreweite Einzigkeit, Ranking oder einen globalen Cap wieder auf einen Einzelort reduzieren.

## Neue Regel: candidate_duplication_v1

Fuer voneinander unabhaengige lokale Kandidaten gilt:

```text
Decision(a | {a}) = Decision(a | {a,b})
Decision(b | {b}) = Decision(b | {a,b})
```

wenn `a` und `b` keinen explizit bewiesenen gemeinsamen Konflikt und keine gemeinsame Transaktionsalternative bilden.

Daraus folgt:

- scoreweites `len()==1` besitzt fuer unabhaengige lokale Kandidaten keine Schreibautoritaet;
- globales Ranking mit anschliessendem `first`, `next` oder `take(1)` besitzt keine lokale musikalische Authority;
- `max_actions=1` oder vergleichbare Caps duerfen nicht als versteckte Zielselektion wirken;
- Takt-/Measure-Reihenfolge darf unabhhaengige Kandidaten nicht zum Gewinner/Verlierer machen;
- Eindeutigkeit bleibt innerhalb einer **explizit bewiesenen gemeinsamen Konflikt-/Transaktionsgruppe** fail-closed zulaessig.

Der Take-Change-Control erkennt ab r1035 neu hinzugefuegte multiplicity-sensitive Decision-Primitiven auch dann, wenn der bisherige selector-sensitive Trigger sie allein nicht erkannt haette. Selector-Implementierungsvertraege muessen zusaetzlich folgende Felder tragen:

```text
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
```

## Legacy-Debt-Inventar

`docs/assistant_memory/multiplicity_invariance_debt_inventory_v1.json` friert den aktuellen r1034-Bestand in drei Klassen ein:

- **A**: hoher Beweisgrad fuer Multiplicity-Verletzung / Proxy-Selektion; Abbauziel.
- **B**: legitime Eindeutigkeit innerhalb derselben Transaktion oder Identity-Bindung; erhalten.
- **C**: Konflikt-/Authority-Scope noch nicht ausreichend bewiesen; vor Aenderung rekonstruieren.

Stand r1035: A=12, B=2, C=4. Das Inventar ist Arbeitsgedaechtnis und keine MusicMutation-/Writer-Authority.

Der erste geplante Retirement-Block ist `MI-A01` (`RhythmicSpineOneTenutoRenderWrite`). Der aktuelle r1034-Korpus zeigt dort E1=0 Kandidaten/kein Write und E2-E10 jeweils mehrere Kandidaten, aber trotzdem genau einen produktiven Tenuto-Write pro Projekt. Diese Laufverteilung ist Evidenz fuer die Dringlichkeit, aber nicht Bestandteil der Kodexdefinition.

## Keine Legacy-Mutation in r1035

r1035 veraendert keinen der inventarisierten Writer. Es fuehrt keine MusicMutation, keine neue Writer-/Repair-Authority und keine neue musikalische Selektionslogik ein. Die bestehenden Schulden werden erst in nachfolgenden Takes gegen die neue Regel abgebaut.
