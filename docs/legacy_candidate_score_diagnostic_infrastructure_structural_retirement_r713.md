# r713 – LegacyCandidateScoreDiagnosticInfrastructureStructuralRetirementAfterTemporalClosure

Parent ist der real grüne r712 `FinalChordTieStopSameOnsetTemporalClosureRegressionExpectationRecovery`. Der r712-Fix in `structural_write_authority/serialization_plan.rs` bleibt byteidentisch und unangetastet.

Dieser Take ist ausschließlich `RefactorOnly`. Er entfernt die nicht mehr autoritative additive Legacy-CandidateScore-Diagnoseinfrastruktur, nachdem r707 die produktive Auswahl bereits auf die lexikographische `CandidateDecisionKey`-Autorität umgestellt hat. Entfernt werden insbesondere `CandidateScoreBreakdown`, `score_candidate`, `rule_math`, Legacy-Score-Felder und deren rein diagnostische Ableitungen/Projektionen. Unabhängige fachliche Scores anderer Domänen bleiben erhalten.

Es gibt keine Änderung an produktiver Candidate-Entscheidung, Lineage, Writer-Authority, MusicXML-/Render-Semantik, Schwellen, Retention, HumanPlayability oder am r712-FinalChord-Fix.

Physische Rust-Zeilen (`src/`, `tests/`, `tools/`): 204006 -> 203089, also -917 (-0,45 %).

Abschlussbedingung ist der reale Windows-Lauf `run.cmd --take --all`: Strict-Clippy, Build und alle Rust-Tests grün; 10/10 Projekte OK; 0 Blockerprojekte; 10/10 vollständige Ausgaben; 10/10 Rendererbeleg; FinalExit 0.
