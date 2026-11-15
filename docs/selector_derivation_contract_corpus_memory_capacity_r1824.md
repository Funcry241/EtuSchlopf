# RAM-Budget im Vollkorpus

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
ParentErrorClassContract: docs/selector_derivation_contract_runtime_worker_memory_capacity.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: simultaneously resident project processes and solver workers draw from the same finite memory budget
CausalFeature: remaining memory in an active container hierarchy can be lower than available host memory
CausalFeature: the automatic schedule can reduce active processes and workers while preserving the original search budget and deterministic candidate ordering
NonCausalContextExcluded: project name, piece, measure, corpus size as a memory proxy and specific runner identity

Im Vollkorpus begrenzt die Automatik die gleichzeitigen Projektprozesse und
Solver-Worker. Sie berücksichtigt damit auch die Anzahl gleichzeitig möglicher
MuseScore-Prozesse. Das Suchbudget beträgt wie bisher 192 Kandidaten und 5 Züge.
Der Einzelprojektmodus verwendet weiter seinen bisherigen Parallelitätsplan.

Linux berücksichtigt `MemAvailable` und den kleinsten sichtbaren verbleibenden
Spielraum der cgroup-v2-Hierarchie. Ist der Speichercontroller nicht auslesbar,
wählt die Automatik einen Projektslot mit einem Worker. Windows verwendet eine
lesende CIM-Abfrage des OS-Speichers aus der Korpuspipeline. Windows-Job-Objektlimits
und Linux cgroup v1 werden nicht ermittelt. Ein fehlender Messwert erscheint als
`unknown`. Die Zeile `KorpusExecutionPlan` zeigt die eingesetzten Slots und Worker.

Die Startwerte des Schätzmodells betragen 512 MiB Reserve für den Lauf, 1.536 MiB
pro aktivem Projekt einschließlich Renderer und 256 MiB pro Solver-Worker.
Das Modell ist eine konservative Planungsannahme, keine gemessene Obergrenze.
Ein einzelnes Projekt und ein Worker bleiben auch unterhalb der Modellschwelle
möglich. Wenn deren tatsächlicher Bedarf das Limit überschreitet, hilft eine
weitere Parallelitätsbegrenzung nicht.

Am Windows-Einstieg kann der Runner bei Bedarf vor `run.cmd --all`
`set "ETUSCHLOPF_CORPUS_JOBS=1"` setzen. Der direkte Programmaufruf unterstützt
weiter `--all --jobs 1`. Ein expliziter Programmwert hat Vorrang vor der
Umgebungsvariablen. Die Einstellungen gelten für den Korpuslauf. Der Build und
seine vollständige Strict-Prüfkette folgen ihrem bestehenden Vertrag.

## Abnahme

Auf demselben Runner bei unveränderten Inputs und Renderer zuerst den Parent,
dann den Kandidaten ausführen. Den kalten Start und den anschließenden Vollkorpus
getrennt messen. Für den Korpus den Peak des gesamten Prozessbaums samt MuseScore
und die Laufzeit aufzeichnen. Der Korpus muss vollständig und mit Exit 0 enden,
alle Master- und Render-MusicXML-Dateien müssen bytegleich sein und die PDFs
müssen dieselben gerenderten Seiten zeigen. Die native Messung entscheidet, ob
das konkrete Runner-Limit genügt.
