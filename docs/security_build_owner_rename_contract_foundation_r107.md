# r107 – Security Build Owner Rename Contract Foundation

## Zweck

Nach r106 ist `src/identity_debt_build.rs` kein Identity-Debt-Monolith mehr, sondern ein kleiner compile-time Security-Orchestrator mit vier bewusst parentgebundenen SecurityNegativeCheck-Primitiven. Der historische Dateiname beschreibt diese Rolle deshalb nicht mehr korrekt.

Ein echter Rename des Cargo-Build-Scripts darf jedoch nicht durch Stubs, `include!`, Dummy-Aufrufe oder eine Lockerung der SecurityNegativeCheck-Relocation erzwungen werden. Der bisherige Relocation-Vertrag bindet den Security-main-Ausfuehrungsanker absichtlich an den physischen Quellpfad und kann daher einen Rename des Build-Owners selbst nicht als normalen Child-Modul-Schnitt behandeln.

r107 fuehrt deshalb nur einen geschlossenen Rename-Vertrag ein. Der eigentliche Dateiname bleibt in r107 noch unveraendert.

## Neuer geschlossener Vertrag

Der Manifest-Marker

`security_build_owner_rename=closed_exact_cargo_build_owner_rename_v1`

ist ausschliesslich fuer `SecurityOnly` zulaessig und gegenseitig exklusiv zu den bestehenden Security-Relocation-Markern.

Ein spaeterer Rename ist nur zulaessig, wenn gleichzeitig gilt:

- `Cargo.toml` aendert ausschliesslich den exakten `package.build`-Pfad;
- alter und neuer Build-Owner sind Rust-Security-Pfade im selben Parent-Verzeichnis;
- der alte Owner wird geloescht und der neue Owner neu angelegt, ohne Parallelowner;
- alte und neue Build-Script-Datei sind byteidentisch;
- entfernte und hinzugefuegte SecurityNegativeChecks sind bijektiv und besitzen dieselben Symbole, Signaturen, Funktionskoerper und Rejection-Surfaces;
- der einzige zulaessige Execution-Anchor-Unterschied ist der physische Rename von `alter_build_owner::main#1` zu `neuer_build_owner::main#1`;
- produktionsnahe Nicht-Security-Referenzanpassungen sind auf `src/productive_write_path_inventory.rs` begrenzt und muessen bytegenau auf die reine alte→neue Pfadsubstitution zurueckfuehrbar sein;
- andere Relocation-Marker duerfen nicht gleichzeitig gesetzt sein.

## Keine Verhaltensaenderung

r107 benennt noch keine Datei um und aendert weder Cargo-Build-Owner noch Security-Ausfuehrungsreihenfolge. Keine Musik-, Writer-, Renderer-, Pipeline-, Report-, Console-, Recovery- oder Kalibrierungssemantik aendert sich.

Der Folge-Take darf nach real gruener r107-Abnahme ausschliesslich den historischen Build-Owner-Namen durch `src/security_build_orchestrator.rs` ersetzen und alle aktiven Pfadbindungen atomar nachziehen.
