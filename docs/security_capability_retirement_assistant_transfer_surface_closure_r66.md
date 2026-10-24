# SecurityCapabilityRetirement Assistant Transfer Surface Closure — r66

## Zweck

r66 ist der erste konkrete Anwendungsfall des in r63 eingefuehrten und in r65
regressionsbewiesenen `SecurityCapabilityRetirement`-Vertrags.

Die produktive Trust-Domain-Regel ist bereits vor r66 aktiv:

- `assistant_export_latest.zip` bleibt im EtuSchlopf-Projektstamm.
- Der Assistant Export besitzt keinen produktiven Writer nach `EtuSchlopfTransfer`.
- `EtuSchlopfTransfer` bleibt Machine-Evidence-Domain fuer
  `etuschlopf_neuro_evidence_*.zip`.
- Die persistierte Page-Count-Preference verwendet bereits First-Class-
  Resultvokabular ohne produktives `Shadow`/`legacyThin`.

Bis r65 blieb in `assistant_export.rs` noch ein `#[cfg(test)]`-Modul mit den
historischen Assistant-Archive-Symbolen erhalten. Der Semantic-Capability-
Scanner inventarisierte sieben dieser Funktionen weiterhin als
`SecurityNegativeCheck`, weil die historische Modul-level-`cfg(test)`-Grenze
nicht Teil der einzelnen Funktionssignatur war.

r66 entfernt dieses pensionierte Modul vollstaendig und bindet die sieben
bisherigen Capability-Identitaeten append-only an
`src/build_gate/security_capability_retirements.txt`.

## Pensionierte Capabilities

Alle Eintraege sind exakt an CapabilityKind, Pfad, Symbol und bisherigen
FNV64-Fingerprint gebunden. Stable reason:

`assistant_export_transfer_archive_surface_retired`

Pensioniert werden ausschliesslich die sieben bereits historisch erhaltenen
Assistant-Archive-Negativchecks. Es wird kein produktiver Security-Check,
Writer oder Transferpfad ersetzt.

## Trust-Domain-Buildregel

`identity_debt_build.rs` verlangt nach r66 nicht mehr die Existenz eines
test-only Archive-Tombstones. Stattdessen gilt fail-closed:

- Archive-Ziel-/Env-/Publisher-Bezeichner duerfen im Assistant-Export-Modul
  nicht zurueckkehren.
- Auch die pensionierten Helper-/Test-Symbole duerfen dort nicht
  wiedereingefuehrt werden.
- Der Retirement-Ledger muss den gebundenen Publisher-Retirement-Eintrag
  enthalten.
- Assistant- und Neuro-Evidence-Domaene bleiben getrennt.

## Nicht geaendert

Keine Aenderung an Musik, MusicXML, Renderer, Joint Planning,
Renderer-Feasibility, Selection, No-Goods, Demand, Thresholds, Writergraph,
PDF-Entscheidung oder Resultwerten.

ChangeClass: `SecurityOnly`.
