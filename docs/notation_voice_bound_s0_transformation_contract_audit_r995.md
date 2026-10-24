# r995 NotationVoiceBoundS0TransformationContractAudit

## Parent und Klasse

r995 baut direkt auf dem im aktuellen vollständigen Assistant-Export belegten r994 `NativeInventoryEvidenceModelCapabilityIdentityClosure` auf. r994 ist der aktuelle Source-Snapshot; r992 bleibt darin als letzte NotationVoice-Fachbasis erhalten. Der Take ist `AuditOnly`: Er erweitert ausschließlich die bestehende read-only NotationVoice-Projektion. Es entsteht keine MusicMutation und kein Writer-, Repair-, Selection- oder Release-Recht.

## Anlass

Nach r992 ist die Architekturfrage nicht mehr, ob `SoundingLane` und `NotationVoice` identisch sind. Sie sind getrennte Wahrheiten. Vor einem produktiven Voice-Writer muss jedoch für einen gebundenen Ausgangszustand S0 nachvollziehbar sein, welche Ereignisse die vorgeschlagene SAME-Line-Komponente bilden, wie ihre kanonische Identität aussieht, ob physische und semantische Zeit übereinstimmen oder eine autorisierte Divergenz noch fehlt, und welche Beweise vor einer Materialisierung noch nicht geschlossen sind.

Der visuelle Katalog bleibt Abnahme-Oracle und niemals Selektor. Etüde 6 Takt 1 ist Witness/Regressionstest, keine Sonderregel.

## Neue S0-Projektion

Die bestehende `NOTATION-VOICE-CAUSAL-LANE-PRESSURE-AUDIT`-Zeile erhält für jede beobachtete Primärbalkenkette zusätzlich:

- eine stabile `proof_chain_id` ausschließlich aus der geordneten Folge `(SourceEventId, SoundingAttackId)`, unabhängig von aktuellen Voice-Labels und unabhängig vom XML-Fingerprint,
- den realen Measure-/Staff-/Intervall-Locator als Diagnoseevidenz,
- pro Mitglied `pitched_ordinal`, physischen Onset, repräsentierte Dauer, Pitch, aktuelle Voice, SourceEventId, SoundingAttackId, semantischen Onset, technische Lane, MaterializationKind und FragmentOrdinal,
- `identity_mapping=total_unique`, wenn jedes sichtbare Beam-Mitglied eindeutig kanonisch gebunden ist,
- `time_provenance=equal_semantic_and_physical_onset`, wenn für die Kette keine Zeitdivergenz vorliegt,
- andernfalls ausdrücklich `representation_time_divergence_requires_authorized_origin_proof` statt einer stillen Freigabe,
- einen expliziten Analyseblocker, weil vollständige Occupancy-/Komponenten-Konfliktdeckung und die sichere Materialisierung noch nicht bewiesen sind.

Damit werden Stable Chain Identity und Standbindung getrennt: Die Ketten-ID ist stabil über eine spätere Voice-Korrektur, während die vorhandene Occurrence-Receipt-Bindung weiterhin den konkreten S0-Ausgabestand bindet.

## Bewusst noch nicht behauptet

r995 setzt `ConflictCoverageStatus: INCOMPLETE_BEAM_LOCAL_ONLY` und `MaterializationSafetyStatus: NOT_PROVEN`. Ein bestehendes `PROVEN_SAME_NOTATION_VOICE` wird deshalb nicht in produktives Schreibrecht umgedeutet. Insbesondere sind noch nicht vollständig geschlossen:

1. sichtbare Occupancy über die Beam-Kette hinaus, einschließlich Akkord-/Staff-Kontext,
2. komponentenweite negative Constraints unabhängig von Bearbeitungsreihenfolge,
3. autorisierte Herkunft jeder Render-/Semantic-Onset-Divergenz,
4. Consumer-/Phase-Wirkung einer konkreten Materialisierung,
5. unabhängige S1-Abnahme einer tatsächlich ausgeführten Transformation.

Fehlende Evidenz bleibt damit von einem bewiesenen Widerspruch unterscheidbar.

## Musikalische Invarianten

Der Analysevertrag bindet für einen späteren NotationVoice-Eingriff strikt:

- Pitch unverändert,
- semantischer Onset unverändert,
- semantische Dauer unverändert.

Eine Repräsentationsänderung erzeugt dafür keine Ausnahme.

## Tests

Zu r994 kommen zwei generische Tests hinzu:

1. Stable ProofChainId, realer Locator, eindeutige Identitätsbindung und der konservative Materialisierungsblocker werden gemeinsam projiziert; die Ketten-ID enthält keine Voice-Nummer.
2. Eine Onset-Divergenz wird ausdrücklich als fehlender autorisierter Origin-Proof ausgewiesen und bleibt fail-closed.

Ausgehend von r994 mit 1.295 Strict-Tests werden 1.297 Strict-Tests erwartet. Der reale Windows-Lauf entscheidet über Acceptance.

## Acceptance

Autoritativ ist `./run.cmd --take --all --inventory`. Erwartet werden Build und Clippy grün, 1.297 Strict-Tests, 10/10 Vollkorpus sowie unveränderte MusicXML-/PDF-Musiksemantik. Der neue Bericht muss E6 T1 anhand stabiler kanonischer Mitglieder lokalisierbar machen, ohne Takt 1 als Selektor zu verwenden. Ein produktiver NotationVoice-Writer ist ausdrücklich nicht Bestandteil dieses Takes.
