# r87 – SecurityNegativeCheck Parent-Module Relocation Contract Foundation

## Zweck

r87 fuehrt **noch keine** bestehende SecurityNegativeCheck-Relocation aus.

Der Take ergaenzt den in r83 eingefuehrten exakten Pfad-Relocation-Vertrag um
einen **separaten**, enger begrenzten Vertrag fuer einen spaeteren echten
Rust-Modul-Schnitt.

Der Anlass ist `src/identity_debt_build.rs`: Der Feedback-Validator soll spaeter
nicht per `include!` textuell ausgelagert werden, sondern als eigenes Rust-Modul.
Dafuer muss eine bisher private Funktion am Ziel exakt `pub(super)` werden,
damit nur das unmittelbare Parent-Modul sie aufrufen kann.

## Harte Grenzen

Der neue Vertrag ist opt-in und nur fuer `SecurityOnly` zulaessig.

Er erlaubt ausschliesslich:

- echte Pfadverlagerung eines vorhandenen `SecurityNegativeCheck`,
- Quelle exakt private `fn`,
- Ziel exakt `pub(super) fn`,
- identisches Funktionssymbol,
- identische Rejection-Surface,
- identische kanonische Signatur nach Entfernung **nur** von `pub(super)`,
- identischen normalisierten Funktionskoerper,
- identischen statisch nachweisbaren Security-`main`-Ausfuehrungsanker,
- exakte `#[path = "..."] mod ...;`-Bindung im bisherigen Owner,
- qualifizierten Aufruf ueber genau dieses Parent-Modul.

Nicht erlaubt sind insbesondere `pub(crate)`, `pub`, Koerperaenderungen,
fehlende Modulbindung, verlorene Ausfuehrungsanker oder die gleichzeitige
Deklaration mit der r83-Exact-Relocation.

## Unveraendert

- r83 `closed_exact_negative_check_relocation_v1` bleibt unveraendert.
- `SecurityCapabilityRetirement` bleibt ein separater Vertrag.
- Es wird in r87 kein vorhandener negativer Security-Check verschoben oder
  pensioniert.
- Keine Musik-, Writer-, Renderer-, Pipeline-, Console-, Recovery- oder
  Artefaktsemantik wird geaendert.

## Folgeschritt

Erst nach einem gruen committed r87 darf ein Folgetake den grossen
Feedback-Validator aus `identity_debt_build.rs` als echtes Child-Modul
extrahieren.

ChangeClass: `SecurityOnly`.
