# v0.10.1791 – ClefTieChainDecisionScopeClosure

## r02 – Strict-Clippy Dead-Code Closure

Der r01-Einzellauf wurde nach erfolgreicher Take-Anwendung ausschließlich im Strict-Clippy/Compile gestoppt: `AttackEvent::start` war nach der tie-chain-geschlossenen Candidate-Verankerung nicht mehr gelesen und wurde durch `#![deny(dead_code)]` korrekt als harter Fehler abgewiesen. r02 entfernt ausschließlich dieses verwaiste Feld und seine Initialisierung. Die produktive Clef-DecisionScope-Closure, ihre Tie-Chain-Verfolgung, Transition-Anker, Kosten, Schwellen, Security-Gates und Writer-Grenzen bleiben gegenüber r01 unverändert; es wird kein `#[allow(dead_code)]` eingeführt.

## Frische Evidenz

Etüde 2, Takt 12, unteres System zeigte auf v0.10.1789 r02 einen vom produktiven Clef-Readability-Owner erzeugten G-Schlüssel exakt zwischen `tie start` und `tie stop` eines C4/C#4-Fortsetzungspunkts. Der Master trug diese zusätzliche Clef-Surface nicht; sie entstand erst in der produktiven Readability-Course-Materialisierung.

## Ursache

Der bestehende Owner erzeugte Transition-Candidates an Starts von Register-Preference-Runs. Die Attack-Gruppierung kannte Akkorde, aber die Candidate-Grenze kannte keine laufende Tie-Chain. Dadurch konnte ein starker Treble-Vorteil auf einer gebundenen Fortsetzung eine neue Clef-Surface mitten in die Haltebogenstruktur setzen.

## Closure

Es gibt keinen neuen Repair-Writer und keine Recovery-Schicht. Der bestehende `ClefReadabilityPrimaryCourse`-Owner erweitert ausschließlich seinen DecisionScope:

- untere Tie-Chains werden strukturell über Voice + geschriebenen Pitch verfolgt;
- beginnt eine Registerpräferenz auf einer gebundenen Fortsetzung, wird derselbe Transition-Candidate an den Ursprung der passenden Tie-Chain rückgebunden;
- eine nicht auflösbare Tie-Fortsetzung ist keine schreibfähige Clef-Grenze;
- `normalize_clef_readability_insertions` verweigert zusätzlich fail-closed jede geplante Primär-Clef-Insertion, wenn an der Insertionsposition bereits eine untere Tie-Chain offen ist.

Damit bleibt `DecisionScope = MaterializationScope` tie-chain-geschlossen. Kosten, Schwellen, Ledger-Pressure-Formeln und Clef-Transition-Kosten bleiben unverändert.

## Security-Voraussetzung

Dieser Wurf baut auf v0.10.1790 `NoNewRepairRecoveryAuthoritySecurityGate` auf. Neue produktive Repair-/Recovery-Authorities sind damit bereits vor dieser MusicMutation fail-closed gesperrt.

## Regression

Ein gezielter Unit-Test bildet die Struktur `F4 -> C4 tie start -> C4 tie stop + E4/G4 chord` nach und verlangt, dass eine notwendige Treble-Transition vor dem Tie-Start materialisiert wird und keine Insertion zwischen Start und Stop liegt.
