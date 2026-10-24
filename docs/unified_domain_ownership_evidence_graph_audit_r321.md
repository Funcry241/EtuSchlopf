# r321 UnifiedDomainOwnershipEvidenceGraphAudit

## Ausgangspunkt

r320 `ProductiveSemanticMutationReachabilityOwnershipAudit` ist real installiert und 10/10 Vollkorpus-GREEN. `FinalExit 0`, `ReleaseDecision released`, `ActiveRepairTarget none`.

r318 hat die semantischen XML-Rekonstruktions-/Adoptionsflaechen neben dem engeren physischen Write-Inventar sichtbar gemacht. r320 hat fuer die 61 starken r318-Flaechen einen konservativen source-basierten Reachability-/Same-Domain-Graphen geliefert. Damit sind zwei getrennte Wahrheiten vorhanden, aber noch keine gemeinsame Domaenenkarte: physisch inventarisierte Authority-Flaechen und r318/r320-Semantikflaechen werden bislang in verschiedenen Projektionen ausgewiesen. Die bereits vorhandenen `OwnerPattern`-Vertraege sind zudem nur als Reportzeilen ausgedrueckt.

## r321

r321 ist der letzte geplante grosse Audit-Schritt vor einer produktiven Domain-Konsolidierung. Der Take vereinigt ausschliesslich bereits vorhandene Evidenzklassen in einer read-only Projektion:

1. `PRODUCTIVE_WRITE_PATH_SURFACES` als bestehendes physisches Build-Inventar,
2. alle r318-Semantikflaechen einschliesslich Authority-intern/delegiert/high-confidence,
3. den r320-Quellgraphen mit Root-Distanz, direkten Callern und Same-Domain-Nachfolgern,
4. die bereits vorhandenen `OwnerPattern`-Aussagen als typisierte read-only Evidenzliste.

Die vorhandenen OwnerPattern-Berichtstexte werden nicht umgedeutet. Ihre Namen, Domaenen und bereits explizit benannten Decision-/Materialization-Evidenzen werden lediglich so typisiert, dass die neue Projektion sie ohne Reporttext-Parsing korrelieren kann.

## Neue Projektion

Unter der bestehenden `MUTATION-OWNERSHIP-EVIDENCE-BOUNDARY` erscheint zusaetzlich:

`UnifiedDomainOwnershipEvidenceGraphAudit`

Sie berichtet pro Domaene:

- Zahl der vereinigten Surface-Schluessel,
- physisch inventarisierte Surface-Anteile,
- semantische Kandidaten,
- Authority-interne und delegierte Flaechen,
- produktive Erreichbarkeit im konservativen Quellgraphen,
- seriellen Same-Domain-Druck ueber die vereinigte Surface-Menge,
- vorhandene explizite OwnerPattern-Evidenz.

Ein Detail-Row behaelt alle Evidenzklassen getrennt. Eine physische Surface ist nicht automatisch ein Decision Owner; eine semantische Surface ist nicht automatisch ein Writer; eine delegierte Surface ist kein Bypass; ein Same-Domain-Nachfolger ist noch kein Beweis fuer doppelte Ownership.

## Wahrheitsgrenze

Der Graph ist diagnostisch. Er weist keine neue Ownership zu und pensioniert keinen bestehenden Owner. Insbesondere gilt weiterhin:

- unresolved reachability != dead code,
- serial same-domain proximity != duplicate decision ownership,
- source role observation != write authorization,
- OwnerPattern evidence != neuer produktiver Contract.

r321 veraendert keine Passreihenfolge, keine MusicXML-/Render-/PDF-Semantik, keine Schwelle, keine Candidate-Auswahl, keine Security-Policy und keine StructuralWriteAuthority. Der ProductiveWritePath-Scanner bleibt unveraendert.

## Entscheidung nach r321

Nach realem 10/10-Korpusbeweis wird kein weiterer breit angelegter Audit automatisch erzeugt. Stattdessen wird aus der vereinigten Domaenenkarte genau eine produktive Domaene ausgewaehlt. Kriterium ist nicht die groesste Trefferzahl, sondern ein belastbarer Nachweis, dass mehrere produktiv erreichbare Stufen dieselbe fachliche Entscheidung erneut treffen oder eine bereits entschiedene Semantik spaeter erneut interpretieren.

Erst dann beginnt ein enger Domain-Refactor mit dem Zielbild:

`Observation -> DecisionOwner -> Plan -> Materialization -> Readback`

`ChangeClass=AuditOnly`. Erwarteter Testbestand bleibt 1071/1071.
