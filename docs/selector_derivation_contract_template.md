# Selector Derivation Contract Template

Dieses Template erzwingt eine zweistufige Ableitung. Eine neue kausale Fehlerklasse wird zuerst in einem eigenen Take dokumentiert und eingefroren. Selector-sensitiver Klassifikations- oder Repair-Code darf erst in einem spaeteren Take auf diesen bereits akzeptierten Vertrag verweisen.

## Stufe A – Error-Class-Definition, noch ohne Selector-Code

Konkreter Pfad: `docs/selector_derivation_contract_<class-name>.md`

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: <independent causal feature>
NonCausalContextExcluded: <known discriminative but noncausal context>
```

Dieser Vertrag darf in seinem Take **nicht** zusammen mit selector-sensitivem neuem Rust-Code entstehen. Er muss zuerst als Bestandteil eines akzeptierten Parents eingefroren werden.

## Stufe B – Error-Class-Classifier in einem spaeteren Take

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_<accepted-class-parent>.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: <implementation of the frozen causal class>
NonCausalContextExcluded: <known discriminative but noncausal context>
```

Der referenzierte Error-Class-Vertrag muss bereits im installierten Parent vorhanden und im Implementierungs-Take byteunveraendert sein.

## Stufe C – Repair-Scope, falls spaeter ueberhaupt gerechtfertigt

```text
SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_<accepted-class-parent>.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: <existing class membership plus independently proven authority/safety condition>
NonCausalContextExcluded: <witness-specific discriminative context>
```

Auch hier darf der Parent-Error-Class-Vertrag im selben Take weder neu erzeugt noch veraendert werden.


## Multiplicity-/Duplikations-Invarianz

Ab r1035 ist fuer jede selector-sensitive Implementierung zusaetzlich `candidate_duplication_v1` verbindlich. Die Existenz eines weiteren **unabhaengigen** gleichartigen Kandidaten darf die Entscheidung eines bereits zulaessigen Kandidaten weder aufheben noch auf einen global gerankten "ersten" Kandidaten umlenken. Eindeutigkeit bleibt nur innerhalb einer explizit bewiesenen gemeinsamen Konflikt-/Transaktionsgruppe zulaessig. Scoreweite Kardinalitaet, globales Ranking oder `max_actions=1` besitzen fuer voneinander unabhaengige lokale Kandidaten keine Schreibautoritaet.

## Authority Retirement ist kein vierter Selector-Modus

`authority_retirement_v1` steht bewusst **neben** `causal_class_first_v1` und `candidate_duplication_v1`. Ein Retirement darf keinen Selector neu definieren, verengen oder verbreitern und keine Multiplicity-Entscheidung veraendern. Der normale Proxy-Identity-/Multiplicity-Gate bleibt deshalb aktiv.

Wenn ein bestehender Writer pensioniert werden soll, muss ein separater `docs/authority_retirement_contract_*.md`-Vertrag beweisen, dass ausschliesslich produktive Writer-Authority streng monoton sinkt. Candidate-Klasse, Target-Auswahl, Ranking/Uniqueness und CalibrationThreshold bleiben in diesem Take unveraendert. Ein spaeterer Cleanup read-only gewordener Selector-/Threshold-Struktur ist eine eigene semantische Transition.
