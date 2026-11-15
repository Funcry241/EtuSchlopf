# Quellbindung bei Handwechsel und Rhythmusteilung

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
AdditionalAcceptedErrorClassContract: docs/selector_derivation_contract_chord_head_split_support_displacement.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the hand-rhythm stage creates existing AtomicPatch actions with owner receipts and may carry exact representation-transfer evidence
CausalFeature: projecting those actions to plain XML variants erases declared material effects and transfer evidence before evaluation
CausalFeature: the existing atomic input path keeps the original patch evidence so evaluation can reconcile the raw patch and the separate rhythm normalization in order
CausalFeature: the existing rhythm owner supplies a forward ledger that names every fragment of one previously bound representation
CausalFeature: the readback conserves that fragment's complete interval, pitch, staff, voice, visibility, chord role and outer tie endpoints
CausalFeature: inner ties join each adjacent fragment and sustain the existing attack identity through the complete interval
CausalFeature: sequential fragments of a physical chord head displace an unsplit dependent member. A head split requires the existing source-bound cohort transport when a chord member depends on the head. At the candidate boundary that requires invariant written onsets, the owner retains the head interval and chord attachment if that transport is unavailable.
NonCausalContextExcluded: project, measure, pitch, duration value, report timing, candidate count, corpus membership and renderer outcome

## Hand- und Rhythmussuche

Die geführte Hand- und Rhythmussuche behält die vom Patch stammenden
Materialeffekte, Transferbelege und Belege der XML-Änderung gemeinsam.
Bytegleiche Zwischenfassungen werden in Erzeugungsreihenfolge dedupliziert.
Die gewählte Fassung durchläuft die bestehende Prüfung nach GuidedSearch.

## Vorwärtsbindung geteilter Noten

Die bestehende Quell- und Anschlagskennung bleibt an allen Fragmenten derselben
gebundenen Kette erhalten. Die Zuordnung stammt aus den Vorwärtszeilen des
Rhythmus-Owners. Onset und Dauer dienen dem Nachweis lückenloser Abdeckung.
Unvollständige Ketten und veränderte äußere Haltebogenenden brechen weiterhin ab.
Jede unabhängige Kette belegt ihre eigenen Zielpositionen.

## Akkordkopf in der Rhythmusnormalisierung

Bei atomaren Kandidaten mit deklarierten Materialeffekten prüft der
nachgelagerte Beleg unveränderte Einsätze. Für diese Prüfung erhält der
Rhythmusnormalisierer den Akkordkopf als ein Element, wenn ihm die
Bindung zur gemeinsamen Teilung fehlt. Der Export kann weiterhin die
belegte Zwischenform für seine nachfolgende Akkordkodierung erzeugen.
