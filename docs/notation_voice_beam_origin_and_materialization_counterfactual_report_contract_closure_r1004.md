# r1004 NotationVoiceBeamOriginAndMaterializationCounterfactualReportContractClosure

r1004 baut direkt auf dem akzeptierten r1002 `NotationVoiceVisibleChordOccupancyAdapterClosure` auf. Der Kandidat r1003 `NotationVoiceBeamOriginAndMaterializationCounterfactualAudit` bleibt rejected Failure Evidence und ist kein Parent: Build, Strict-Clippy und integrierte Strict-Tests waren gruen, aber der reale 10er-Korpus endete 0/10 OK mit je genau einem Projektblocker.

Die statische Ursache liegt im finalen Report-Vertrag, nicht in Musik, Beam-Klassifikation oder Counterfactual-Planung. r1003 haengte innerhalb der bereits deklarierten Sektion `NOTATION-VOICE-PRE-SERIALIZATION-CONSTRAINT-CLOSURE-AUDIT` einen zweiten, top-level-artig formatierten Abschnitt `NOTATION-VOICE-MATERIALIZATION-COUNTERFACTUAL-AUDIT` samt Unterstreichung an. `CompactedReport::project_declared()` parst den erzeugten Text erneut und verwirft jede dort auftauchende Sektion, die nicht im Projektionsvertrag des Producers deklariert ist. Deshalb blockierte dieselbe Report-Contract-Verletzung jedes Projekt unabhaengig von dessen musikalischem Inhalt.

r1004 reappliziert die fachliche r1003-Analyse direkt auf r1002, aendert aber nur die Counterfactual-Darstellung innerhalb der bereits deklarierten Owner-Sektion: statt eines zweiten Report-Section-Headers wird ein normaler Body-Marker `MaterializationCounterfactual: read_only` mit `CounterfactualProjectionOnly: true` ausgegeben. Es entsteht keine zusaetzliche Report-Sektion und damit keine undeclared-section-Verletzung.

Die fachliche Analyse bleibt unveraendert:

- voice-freier Exact-Match des Beam-Witness gegen den unmittelbar pre-FinalChord vorhandenen Zustand;
- historische Beam-Owner-Authority bleibt ausdruecklich unbewiesen;
- read-only Batch-Counterfactual durch denselben bestehenden FrozenSerializationVoiceChannel-Planer;
- nur Kandidatenquellen erhalten im Shadow technische Lane 1 und repraesentierte statt sounding-basierter absolute Intervalle;
- SourceEventId, SoundingAttackId, Staff, Pitch und semantische Source-Zeilen bleiben unveraendert;
- Vergleich von Zielrelation sowie externer Channel-Partition und PresentationRoles;
- keine MusicXML-/PDF-/Klangmutation und kein Writer-, Repair-, Selection- oder Release-Recht.

Der Lauf soll wieder den vollstaendigen 10er-Korpus erreichen. Die Resultate duerfen weiterhin fachlich `BLOCKED_*` innerhalb des read-only Audits melden; solche Kandidatenstatus sind Diagnose und keine undeclared Report-Sektion. Erwarteter Strict-Testbestand bleibt 1.313.
