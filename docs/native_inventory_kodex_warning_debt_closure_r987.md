# r987 NativeInventoryKodexWarningDebtClosure

r987 folgt direkt auf den real ausgefuehrten r985 `NativeInventoryArchitectureExport`. r985 hat den kompletten neuen `--inventory`-Pfad funktional erfolgreich ausgefuehrt: Vollkorpus 10/10 OK, CMS-1 `0/0/0`, `EtuSchlopf.inventory.json` und `EtuSchlopf.inventory.xlsx` wurden current-run-gebunden erzeugt, validiert und in den Assistant Export aufgenommen. Der Run endete mit Exit 0, wurde aber wegen neun Clippy-Warnings korrekt gelb projiziert.

Der Folgekandidat r986 enthielt bereits die richtige rein mechanische Bereinigung, wurde jedoch vor Installation fail-closed abgelehnt: `RefactorOnly` darf den Kodex-Pfad `src/bin/etuschlopf_launcher/inventory_export.rs` nicht aendern. Die Change-Control-Regel erlaubt fuer diesen Pfad `KodexOnly`; zugleich erlaubt der bestehende `warning_debt_closure`-Vertrag explizit `RefactorOnly` oder `KodexOnly`. r986 bleibt daher rejected Failure Evidence und ist kein Parent.

r987 reappliziert dieselben neun Clippy-Bereinigungen direkt auf accepted parent r985, nun unter `KodexOnly`:

- verschachtelte `mod`/`trait`/`impl`-Parserbedingungen werden als aequivalente Let-Chains formuliert;
- der generische Brace-Skip wird ohne verschachteltes `if` formuliert;
- `find_item_open_brace(...)` iteriert direkt ueber `tokens.iter().enumerate().take(end).skip(start)` statt den Slice nur ueber einen Range-Index zu lesen;
- die Sichtbarkeits-Rueckwaertssuche verwendet das aequivalente Char-Array-Pattern fuer `rfind`.

Keine ESI-1-, ARCH-1-, CMS-1- oder XLSX-1-Regel wird geaendert. Symbolscope, Symbolidentitaet, Callgraph-/Referenzableitung, A/B/C/D/OFFEN-Klassifikation, Removal-Proof, Tabellenstruktur, AutoFilter, Freeze-Pane, atomare Publikation, Stale-Cleanup und Assistant-Export-Bindung bleiben semantisch unveraendert. Es gibt keine Musik-, MusicXML-, PDF-, Writer-, Repair-, Selection-, Release- oder MasterApply-Aenderung.

Erwarteter Strict-Testbestand bleibt 1.280. Ziel ist ausschliesslich `TotalWarnings: 0` und damit die erste gruene ESI-1/ARCH-1/CMS-1-Inventur-Baseline.
