# r1288 – SSC-Vertrag v4, Abschluss

Parent: accepted r1287 `SscAdoptionEnforcementV4`. Klasse `KodexOnly`, benannte Revision `SscK2GovernanceRevisionKodex`.

## Stand bei Abschluss

| | Wert |
|---|---|
| Released Owner-Events | 1438, alle `V4Permitted` |
| Unreleased Commits (verworfene Kandidaten) | 12 635, alle Permit |
| `V4AllCommitsPermitted` | yes (10/10) |
| `PermitEnforcement` | active an beiden A7-Grenzen (r1287) |
| r1238-Restmenge offener Exit-Pflichten | leer |
| Musik | 20/20 XML bytegleich, 50/50 PDF-Seiten gerendert gleich |

## Abschlussnachweis

`src/build_gate/ssc_exit_closure_v1.txt` trägt `exit_claim=closed`. K2 kompiliert den Text ein und lässt ihn nur unter diesen Bedingungen gelten (`validate_exit_closure`, in `validate_tree`):

1. Die Restmenge `ssc_exit_remaining_v1.txt` ist leer.
2. Beide A7-Grenzen sind verdrahtet (`commit_owner_receipts`, `RenderLineage::commit`), und ihr Rumpf ruft `ssc_adoption_verdict` auf.
3. Der Bericht meldet `PermitEnforcement: active` und nirgends `none`.
4. `ssc_adoption_verdict` entscheidet über `classify_bound_origin`, also den Bericht-Klassifikator, und `adoption_decision` mit genau einem Permit.

Der Nachweis kann nie zurückgezogen werden (`validate_exit_closure_transition`). Solange er besteht, scheitert jeder Folge-Take, der eine der Bedingungen bricht, etwa die Verdrahtung entfernt, Enforcement abschaltet oder die Entscheidung aufweicht. Auch ein GovernedChange-Take bricht damit seine eigene Baumprüfung.

## Warum ein eigener Nachweis statt `exit_claim=closed` in der Restmengen-Datei

Die Laufzeit-Projektion `append_ssc_remaining_projection_v2` ist per K2 tokengleich an das eingefrorene A4-Dokument gebunden und verlangt in der Restmengen-Datei die Zeile `exit_claim=not_closed`. Diese Zeile umzustellen hätte zwei bis drei zusätzliche Takes gekostet, weil die Laufzeit nur GovernedChange und A4 nur KodexOnly ändern darf. Die Zeile bleibt deshalb als eingefrorenes Schema-Token der r1238-Projektion stehen. Der Abschluss steht im Nachweis, der genau das auch benennt.

## Shadow-only-Texte

`ssc_evidence_proof_shadow_contract_v1.txt` (`runtime_permit_enforcement=forbidden`, `exit_claim=not_closed`) und `ssc_reference_firewall_v2.txt` (`enforcement=forbidden`) sind vom Parent einkompilierte, byte-eingefrorene Dokumente. Kein Take kann sie ändern, auch keine benannte Revision; der kompilierte r1287-Validator lehnt das ab (lokal nachgewiesen). Der Abschlussnachweis nennt beide Aussagen ausdrücklich als abgelöst (`superseded_shadow_statement`). Rechtsverbindlich ist der Nachweis, weil K2 ihn gegen den tatsächlichen Baum prüft.

Die strukturellen Gates bleiben unverändert in Kraft: Senken ohne Entscheidungs-Accessor, geschlossene Beweis-Firewall und Escape-Schutz außerhalb der verdrahteten Grenzen. Sie sind keine Shadow-only-Gates, sondern schützen das Enforcement.

## Tests

Neu sind `exit_closure_holds_only_with_empty_remaining_set_and_enforcing_a7_boundaries` (positiv plus sieben Negativfälle) und `exit_closure_can_never_be_withdrawn`. Beide nutzen nur synthetische Inhalte, keine Live-Dateien.
