# Notendauern aus der freien Aufführung bis zum MIDI-Seed

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
ParentErrorClassContract: docs/selector_derivation_contract_written_duration_from_performance.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: An onset contains independent pitched events on the same track and channel. A missing successor leaves a release to estimate from the performance.
CausalFeature: Overlapping played attacks followed by closely grouped key releases support the same written end when the matched continuations of the sounding peers agree. An intervening attack of the same key precludes this continuation.
CausalFeature: When the whole chord lacks a matched successor, consecutive preceding pulses in the same local timeline can establish a common subdivision and its performed articulation. A delayed next onset leaves time for a rest after that written subdivision.
CausalFeature: Each aligned regular performance event carries a written onset and a release proposal. Continuation, repeated-release or corroborated chord evidence binds the duration. A rounded physical release alone leaves the written end open.
CausalFeature: For an open duration the primary piano course selects a written end using its existing onset, beat and rest rules. Each retained event enters the interval writer with its source identity and its decided interval. A cursor conflict can require a separate cursor run, but cannot select a new written release.
CausalFeature: The existing source-voice owner has priority when the input also qualifies for independent voice assignments.
NonCausalContextExcluded: Title, corpus membership, filename, absolute pitch, absolute bar position, a particular meter, reference score and global frequency of a candidate.

Das Zeitmodell verbindet Anschläge zu möglichen Stimmfortsetzungen. Ein
Folgeton kann nur eine solche Verbindung tragen. Beim Übergang von einem
mehrstimmigen Akkord zu weniger Stimmen fehlen deshalb manchmal
Folgeverbindungen, obwohl mehrere Tasten gemeinsam losgelassen werden.

Bei einem Ton ohne eigene Fortsetzung prüft der Owner die gespielten
Enden benachbarter Akkordtöne derselben Spur und desselben Kanals. Liegen
die Releases innerhalb der Streuung der Anschläge und belegen die
Fortsetzungen ein gemeinsames notiertes Ende, kann dieses Ende auch für
den auslaufenden Ton gelten. Ein abweichender Release oder eine
erneut angeschlagene gleiche Taste begrenzt diese Aussage.

Fehlt dem gesamten Akkord die Fortsetzung, können die zwei unmittelbar
vorhergehenden Pulse eine gleich artikulierte Unterteilung belegen.
Alle Akkordtöne beider Vergleichsstellen müssen bis zum selben relativen
Folgeeinsatz führen. Die gespielten Längen am aktuellen Einsatz müssen
zur dort beobachteten Artikulation passen. Liegt der nächste Anschlag
später, kann das Zeitmodell die belegte Dauer mit anschließender Pause
wählen. Fehlt diese Evidenz, gilt die bisherige Schätzung.

## Übergabe an den Notensatz

Die Aufführungsanalyse übergibt die Quellidentität, den ermittelten Einsatz
und den Status der Dauer. Eine zugeordnete Fortsetzung, ein bestätigtes
Akkordende oder ein belegtes Ende aus einer Wiederholung ergibt einen
entschiedenen Wert. Ein nur aus dem Loslassen der Taste gerundeter Wert
bleibt eine offene Prognose. Die ursprünglichen MIDI-Ereignisse mit den
tatsächlichen Spielzeiten bleiben erhalten.

Für offene Dauern verwendet der Klaviersatz seine bestehende Entscheidung
über das Primärintervall. Sie kennt die gewählte Systemzuordnung, den
Folgeeinsatz und die metrisch darstellbaren Pausen. Die Entscheidung
erfolgt am jeweiligen Kandidaten vor der Fragment- und Stimmenplanung.
Die übrigen Kandidaten teilen sich weiterhin die ursprüngliche
Aufführungsanalyse. Bei eigenständig belegten Quellstimmen entscheidet
deren bestehender Owner über die Darstellung.

Der im Elternstand angenommene Intervallvertrag
`docs/selector_derivation_contract_bound_written_duration.md` gilt für
die Übergabe der entschiedenen Werte. Der Seed trägt die regulären Töne
mit Einsatz und Ende durch die Verteilung auf Cursorläufe, Taktgrenzen
und Halteketten. Vorschläge behalten die Bindung an ihre Hauptnote.
Der Rücklesetest prüft die tatsächlich geschriebene Quellnote.
Die Referenz gehört ausschließlich zur Auswertung nach dem Programmlauf.
