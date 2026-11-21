# Erhaltene Beobachtungen beim Staccato- und Cross-Staff-Bericht

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
ParentErrorClassContract: docs/selector_derivation_contract_retained_payload_outside_future_observation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: A smaller retained observation must reproduce the same candidate count, tie order, persisted mark and status after every reachable continuation.
CausalFeature: Pre- and post-trial measurements may share a value only when they describe the same unmodified XML bytes and the writer cannot be reached.
NonCausalContextExcluded: Project identity, performer, piece, corpus frequency, score position as a catalogue identifier, rendering duration and the measured size of the reduction.

## Staccato

Die Hauptstimme des oberen Systems liefert weiterhin die Paare aus einem sichtbaren Einzelton und einer unmittelbar folgenden Pause. Beide Ereignisse liegen innerhalb derselben metrischen Einheit. Dauern, Darstellbarkeit des gemeinsamen Notenwerts, Reihenfolge bei Gleichstand und die bisherige Obergrenze von zwölf bleiben Teil der Beobachtung. Die Obergrenze beschränkt die gemeldete Anzahl. Für den ersten Kandidaten liest die Pipeline die gespeicherte Staccatomarkierung am passenden Einzelton zurück. Ein nachfolgender Akkordton schließt die Bestätigung aus.

Die kleinere Kandidatenstruktur enthält Position, Tonhöhe und Dauer für dieselben Vergleiche. Auf dem üblichen Pfad nutzt die Prüfung das vorhandene XML ohne Kopie und liest die Persistenz der Markierung einmal. Die bisherigen Schutzprüfungen, die bedingte Zuordnung eines Schreibbelegs und die Prüfung des Akkordplans bleiben beim bisherigen Owner. Die Anzahl allein erzeugt keinen Beleg. Das Vervielfachen unabhängiger Kandidaten erteilt keine Schreibberechtigung.

## Cross-Staff

Der Versuch beginnt und endet mit denselben XML-Bytes. Er wertet Tonbestand, Pausen, Klangsignatur, Parsebarkeit und rhythmische Bilanz einmal aus. Der Bericht verwendet die gewonnenen Werte an beiden bisherigen Ausgabestellen, einschließlich fehlerhafter Eingaben. Der gebundene Funktionsname bleibt erhalten und lehnt einen direkten Aufruf ab. Die Auswertung öffnet keinen Ausgabeweg.

## Nachweis

Die synthetischen Staccato-Vergleiche decken positive Rücklesung, Akkordgrenze, fehlende Markierung, mehrdeutige Positionen und die Begrenzung der Anzahl ab. Für Cross-Staff werden vollständige Berichtszeile und Zustandswerte verglichen. Vorhandene Notenbilder und archivierte MusicXML-Dateien dienen dem separaten Ergebnisvergleich. Die ursprünglichen Auswahlbedingungen ergeben sich aus den bereits zugelassenen Quellpfaden.
