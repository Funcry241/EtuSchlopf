# r1036 r09 - AuthorityRetirementKodexClosure

## Recovery aus r07

r05 wurde vor Installation vom bestehenden Semantic-Capability-Diff abgewiesen, weil die Implementierung die bereits vorhandene Funktion `validate_semantic_change_with_kodex_relocation_options` in einen delegierenden Wrapper umformte. Dadurch erschien ihr bestehender negativer Kodex-Check als entfernt, obwohl er nur in eine neue Hilfsfunktion verschoben worden war.

r06 bewahrte diesen Owner korrekt und passierte den PRE-TAKE-DIFF-GATE, scheiterte aber erst nach Installation am Architektur-Build-Gate `RUST SOURCE FACT OWNER BINDING REGRESSION`. Ursache war ausschliesslich, dass `take_change_control.rs` seinen historisch path-gebundenen Import `use crate::rust_source_scan_primitives::fnv1a64;` zu einem Sammelimport mit `mask_non_code` erweitert hatte. Damit entstand eine neue direkte Source-Fact-Bindung des TakeChangeControl-Owners.


r07 passierte PRE-TAKE-DIFF-GATE, Architektur-Build-Gate, Clippy und den normalen Build. Erst beim Kompilieren der integrierten Rust-Tests wurde eine bestehende testinterne `ChangeManifest`-Fixture sichtbar, die nach Einfuehrung des neuen optionalen Felds `authority_retirement_contract` nicht um `None` ergaenzt worden war. r08 korrigiert ausschliesslich diese vollstaendige Struct-Initialisierung; die Retirement-Governance selbst bleibt gegenueber r07 unveraendert.

r08 passierte PRE-TAKE-DIFF-GATE, Architektur-Build-Gate, Clippy und Build und kompilierte die Tests vollstaendig; das Bootstrap-Testtarget blieb jedoch rot. Die statische Rekonstruktion zeigte, dass die synthetische Retirement-Fixture nach Entfernen von `write_musicxml(xml)` den Parameter `xml` weiterhin im Incoming-Body referenzierte. Zusammen mit dem Namen `apply_candidate` blieb der Surface dadurch nach der bestehenden Writer-Heuristik selbst als Writer klassifiziert; der Witness stellte also keinen reinen Writer-Authority-Verlust dar. r09 korrigiert ausschliesslich diesen Test-Witness: Selector- und Threshold-Zeilen bleiben bytegleich, und im Incoming-Body entfaellt nur der produktive `write_musicxml(xml)`-Aufruf.

r07 stellt den exakten bisherigen FNV-Import wieder her und verwendet fuer die Retirement-Invarianz die in TakeChangeControl bereits vorhandene Added-/Removed-Line-Analyse. `validate_semantic_change_with_governance_options` ruft weiterhin zuerst die unveraenderte bestehende Semantic-Capability-Policy auf und fuehrt `authority_retirement_v1` erst danach als zusaetzliche fail-closed Pruefung aus. Es wird keine bestehende Negative-Check-Capability pensioniert oder verlagert und keine neue Source-Fact-Owner-Beziehung erzeugt.

## Anlass

r1036/r02/r03 versuchten die bereits in r1035 als MI-A01 inventarisierte produktive `RhythmicSpineOneTenuto`-Proxy-Authority zu pensionieren. Die Kandidaten wurden korrekt abgelehnt: zuerst wegen neuer Writer-Reachability aus einem Test, dann wegen gleichzeitiger Writer-/CalibrationThreshold-Aenderung und zuletzt durch den Proxy-Identity-Selector-Gate.

Der letzte Befund zeigte eine Governance-Luecke: TakeChangeControl konnte einen reinen Verlust bereits vorhandener Writer-Authority nicht von einer selector-sensitiven Neudefinition unterscheiden. Die Loesung darf weder `causal_class_first_v1` noch `candidate_duplication_v1` abschwaechen.

## Neue Semantik

`authority_retirement_v1` ist eine zusaetzliche fail-closed Beweisrelation fuer spaetere `MusicMutation`-Takes. Sie ist **keine neue Change-Class** und **kein Gate-Bypass**.

Ein deklarierter Retirement-Take ist nur zulaessig, wenn:

- ein konkreter produktiver Parent-Surface gebunden ist;
- alle geaenderten produktiven Rust-Pfade exakt im Retirement-Scope stehen;
- der Semantic-Capability-Diff einen reinen monotonen Writer-Authority-Verlust erkennt;
- keine neue oder modifizierte positive Writer-Capability entsteht;
- Writer-Reachability nur strikt kleiner werden darf;
- `CalibrationThreshold` byte-/semantisch unveraendert bleibt;
- Candidate-Klasse, Selector-Relation, Target-Auswahl und Multiplicity-Entscheidung unveraendert bleiben;
- kein Ersatzwriter entsteht;
- der bestehende Proxy-Identity-/Multiplicity-Kodex weiterhin normal ausgefuehrt wird.

## Wichtige Konsequenz fuer RhythmicSpineOneTenuto

Der naechste MusicMutation-Candidate darf nicht den r03-Umbau wiederholen. Insbesondere duerfen `candidates.into_iter().next()`, Hinge-/Target-Auswahl und der bestehende `max_size`-Threshold in diesem Retirement-Take nicht geaendert werden. Sie koennen read-only weiterlaufen. Entfernt wird zuerst ausschliesslich die produktive Mutation/Writer-Reachability.

Erst ein spaeterer, separat klassifizierter Take darf ueber den Cleanup des dann read-only gewordenen Legacy-Selectors oder des Thresholds entscheiden.

## r1036 r09 selbst

Dieser Take ist `KodexOnly`. Er veraendert keine Musikpipeline, keinen Writer, keine Candidate-Klasse, keine Gewichtung und keinen Threshold. Er fuehrt nur den ausfuehrbaren Retirement-Vertrag, seine statischen Negativbedingungen und die Projektessenz-Dokumentation ein.
