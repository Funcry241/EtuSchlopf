# r320 ProductiveSemanticMutationReachabilityOwnershipAudit

## Ausgangslage

r318 ist 10/10 Vollkorpus-GREEN und erweitert die Beobachtbarkeit produktiver MusicXML-Semantikflaechen, ohne den physischen Security-Scanner umzudeuten. Der r318-Audit findet 129 Kandidatenfunktionen, davon 61 hochkonfidente Flaechen ausserhalb der engeren physischen Inventur.

Diese 61 Flaechen duerfen nicht direkt als konkurrierende Writer interpretiert werden. Unter ihnen befinden sich String-/Fragment-Builder, State-Adoptionen, direkte Buffer-Rebindings und Hilfsfunktionen. Fuer eine DomainOwnership-Konsolidierung fehlt damit noch die Information, ob und wie diese Flaechen von der produktiven Pipeline erreicht werden und ob spaetere Flaechen derselben Domaene seriell folgen.

## Revisionshinweis

r319 wurde vom Take-Tree-Integrity-Gate vor Installation abgewiesen, weil das ZIP versehentlich `Cargo.lock` als unbekannten Top-Level-Pfad enthielt. Der Kandidat wurde nicht installiert und wird nicht wiederverwendet. r320 basiert deshalb weiterhin direkt auf r318. Der Rust-Audit-Inhalt ist gegenueber dem verworfenen r319-Kandidaten fachlich unveraendert; die Recovery betrifft ausschliesslich die erlaubte Paketstruktur und Revisionsmetadaten.

## r320

r320 erweitert denselben Audit um einen konservativen source-basierten Callgraphen:

- produktive Rust-Funktionen werden mit derselben read-only Quellmaskierung wie r318 erfasst;
- Call-Kanten werden nur fuer im produktiven Baum eindeutige Funktionsnamen projiziert;
- Einstiegspunkte sind `src/pipeline.rs::run` und das bestehende `real_main` im Pipeline-Orchestrator;
- fuer jede hochkonfidente r318-Flaeche werden Erreichbarkeit, Root-Distanz, direkte eingehende Caller und spaeter erreichbare starke Flaechen derselben Domaene ermittelt;
- eine lexikalische Rollenbeobachtung trennt CandidateBuilder, DecisionNamedSurface, StateAdoption, BufferRebind, StringReconstruction und UnresolvedSurface.

## Wahrheitsgrenze

Der Graph ist diagnostisch. Eindeutige Namensaufloesung minimiert falsche Kanten, ist aber kein vollstaendiger Rust-Name-Resolver. Nicht aufgeloeste Erreichbarkeit beweist daher keinen toten Code. Gleichfalls ist `SerialSameDomainCandidates` nur ein Signal fuer moegliche Ownership-Ueberlagerung und kein Beweis, dass zwei Funktionen dieselbe fachliche Entscheidung besitzen.

r320 veraendert keine Pipeline-Reihenfolge, keine MusicXML-Semantik, keine Schwelle, keinen bestehenden Owner und keine Security-Policy. Es schafft ausschliesslich die Evidenz, die vor einer spaeteren Domaenenkonsolidierung fehlt.
