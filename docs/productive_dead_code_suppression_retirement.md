# ProductiveDeadCodeSuppressionRetirement

## r02: reale Strict-Clippy-Feldclosure

Der r01-Lauf meldete fuenf Fehler und keine Warnung. Entfernt werden nur die
tatsaechlich ungelesenen Felder und ihre ausschliesslichen Erzeuger:

- `RenderXmlPianisticRestRepairAction`: `old_rest_count`, `new_rest_count`,
  `beat_aligned`, `risk`,
- `RenderXmlPianisticMissingRestInsertAction`: `onset`, `duration`,
  `beat_aligned`, `risk`,
- `RenderXmlRestConsolidationCandidate.last_onset`,
- `PianistHelpfulArticulationRow.beat_unit`,
- rekursiv `RenderXmlMissingRestInsertCandidate.forward_fragment` und
  `RenderXmlForwardSpan.fragment`; ihre einzige Verbraucherkette war die
  entfernte Risikoformatierung.

Die stafflokale Gruppierung des Cross-Staff-Ledgers bleibt identisch; lediglich
die nie verwendete lokale Bindung der Staff-Komponente wird nicht mehr erzeugt.
Rest-Auswahl, Scores, Sortierung, XML-Replacements, Invariantengates und
Verdicts bleiben unveraendert.

Bilanz r02:

- `#[allow(dead_code)]`: weiterhin 13,
- physische Rust-Zeilen: 115.009 -> 114.971,
- Netto r01 -> r02: -38,
- Netto gegen v0.10.1595 r06: -570.


## Ausgangspunkt

Der autoritative Stand v0.10.1595 r06 ist hellgruen abgenommen. Im gesamten
Quellbaum stehen 44 `#[allow(dead_code)]`-Attribute. Die Attribute werden nicht
blind geloescht, weil Modulreichweite in Main-, Launcher-, Bootstrap-, Build-
und Testzielen unterschiedlich ist.

## Klassifikation

### Entfernt: 31 produktionsnahe Unterdrueckungen

Diese Attribute lagen direkt auf produktiven Datenmodellen oder privaten
Audit-/Row-/Action-Typen. Nach ihrer Entfernung wurden alle vom Compilerlint
vorher verdeckten write-only-Felder, Formatierer, Diagnosefunktionen und
Transportparameter rekursiv entfernt.

Die groessten abgeschlossenen Familien sind:

1. zehn wurzellose Sheaf-/Noether-Projektions- und Ledger-Audittypen,
2. RuleSignal/RuleStateVector samt ausschliesslich dafuer erzeugtem Hash- und
   Gruppierungszustand; `score_penalty` summiert weiterhin exakt dieselbe
   gewichtete Energie je Issue,
3. Gauss-Reject-Diagnostik, deren Anker-, Kernel-, Smoothness-, RuleId- und
   Begruendungsergebnisse nie konsumiert wurden,
4. Rest-/Cross-Staff-/Ledger-Actions und Rows mit nie gelesenen Feldern,
5. produktiv verwendete Key-, Phrase-, Cursor-, Oktav-, Hidden-Scope-,
   Staccato- und Artikulationstypen ohne pauschale Lintunterdrueckung.

### Beibehalten: 13 explizite Grenzfaelle

- `src/main.rs`: drei gemeinsam eingebundene Module,
- `src/bin/etuschlopf_launcher.rs`: gemeinsamer Corpus-Manifestvertrag,
- `src/bin/etuschlopf_take_bootstrap.rs`: gemeinsamer Take-Control-Vertrag,
- `src/identity_debt_build.rs`: vier Build-Skript-Module,
- `src/identity_debt_inventory.rs` und
  `src/productive_write_path_inventory.rs`: Test-Support-Module,
- `tests/global_identity_debt_inventory.rs`: Test-Support-Import,
- `src/structural_write_authority/thirty_surface_productive_authority.rs`:
  namentlich manifestgebundener, nullfeldiger und autoritaetsloser Tombstone.

Diese 13 Ausnahmen werden spaeter als eigene Ziel-/Modulgrenzen behandelt.
Eine pauschale Entfernung in diesem Take wuerde den produktiven Dead-Code-
Rueckbau mit Build- und Testarchitektur vermischen.

## Verhaltensgrenze

Nicht geaendert werden:

- autorisierte Datei-Writer und deren Reihenfolge,
- musikalische Regeln, Kandidatenreihenfolge und Adoption,
- Master-/Render-MusicXML-Mutationspfade,
- Blocker-, Release-, PDF-Quellen- und Exitautoritaet,
- die Reihenfolge produktiver `&mut best`-Operationen.

Es gibt keine Lint-Suppression, keinen Dummy-Read und keine Ersatzsenke.

## Bilanz

- `#[allow(dead_code)]`: 44 -> 13
- physische Rust-Zeilen: 115.541 -> 115.009
- Netto: -532

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Bis zum gruenen Lauf bleibt v0.10.1595 autoritativ.
