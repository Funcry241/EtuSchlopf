# r83 – SecurityNegativeCheck Exact Relocation Contract Foundation from r80

## Parent

Real current parent: EtuSchlopf 0.10.1839 r80 `LauncherConsoleCompactStartupHeaderClosure`.

This package intentionally preserves the complete r80 launcher and its documentation byte-for-byte. It does not revert, extend or reinterpret the compact startup-header change.

## Ziel

`identity_debt_build.rs` ist historisch zu einem großen Security-Orchestrator mit mehreren fachlich verschiedenen Schutzbereichen angewachsen. Ein sauberer Ownership-Schnitt darf bestehende `SecurityNegativeCheck`-Capabilities jedoch nicht durch Umbenennung, Retirement, Stubs oder scannerfreundliche Wrapper vortäuschen.

r83 führt deshalb **noch keine Extraktion aus `identity_debt_build.rs` durch**. Dieser Take schafft ausschließlich den fail-closed Vertrag, mit dem ein späterer Take eine bestehende negative Security-Capability zwischen Security-Ownern **nachweislich unverändert** verschieben darf.

## Neuer expliziter Take-Vertrag

Ein SecurityOnly-Take kann künftig optional deklarieren:

`security_negative_check_relocation=closed_exact_negative_check_relocation_v1`

Ohne diesen Marker bleibt jede entfernte `SecurityNegativeCheck`-Capability weiterhin verboten, sofern sie nicht durch den bestehenden und semantisch getrennten `SecurityCapabilityRetirement`-Vertrag exakt pensioniert wird.

## Relocation-Beweis

Der Marker autorisiert keine allgemeine Refactor-Ausnahme. Er wird nur akzeptiert, wenn:

1. `change_class=SecurityOnly` gilt.
2. Alle geänderten `.rs`-/`.cmd`-Pfade der Security-Domäne gehören.
3. Mindestens eine negative Security-Capability entfernt und mindestens eine hinzugefügt wird.
4. Anzahl der entfernten und hinzugefügten Negative-Checks identisch ist.
5. Für jeden verschobenen Negative-Check in Alt- und Neubaum mindestens ein statisch nachweisbarer Security-`main`-Ausführungsanker existiert.
6. Die Menge dieser Security-`main`-Anker vor und nach der Verschiebung identisch bleibt; ein bytegleich verschobener, aber nicht mehr erreichbarer Check ist damit verboten.
7. Die beiden Capability-Multimengen bijektiv identisch sind hinsichtlich Funktionssymbol, Fingerprint der Rejection-Surface, normalisierter Signatur, vollständigem normalisiertem Funktionskörper und Fingerprint der unveränderten Security-`main`-Ausführungsanker.
8. Zusätzliche, fehlende oder veränderte Negative-Checks den Vertrag fail-closed scheitern lassen.

Nur die exakt nachgewiesenen entfernten Capability-Keys werden für die normale SecurityOnly-Policy als Relocation autorisiert.

## Retirement bleibt getrennt

`SecurityCapabilityRetirement` wird nicht abgeschwächt und nicht umgedeutet. Eine semantisch nur verschobene Negative-Security-Funktion darf weiterhin nicht als Retirement durchgehen.

Damit gibt es zwei verschiedene Operationen:

- **Retirement:** Schutzrecht wird bewusst und append-only pensioniert.
- **Relocation:** Schutzrecht bleibt vollständig erhalten und wechselt ausschließlich seinen Security-Pfad/Owner.

## Adversarial Closure

Die bestehende Identity-Build-Policy pinnt den neuen Vertrag selbst. Abgedeckt sind exakte Pfadverschiebung, fehlender Marker, Body-/Signatur-/Symboländerung, verlorener Security-main-Ausführungsanker, zusätzlicher Negative-Check und die exakte Marker-Syntax.

`take_change_control.rs` und `semantic_capability_diff.rs` bleiben bytegepinnt; die abhängigen Trust-Anker werden explizit neu gebunden. Zusätzlich bleibt der bereits in r80 geänderte Launcher-Fingerprint erhalten.

## Bewusste Nicht-Ziele

r83:

- verschiebt keine bestehende SecurityNegativeCheck-Funktion,
- teilt `identity_debt_build.rs` noch nicht auf,
- verändert den r80-Launcher nicht,
- ändert keine Musiklogik,
- ändert keine Writer-Authority,
- ändert keine Schwellenwerte,
- ändert keine weitere Console-/Presentation-Logik,
- ändert keine Take-Rollback- oder Recovery-Semantik,
- pensioniert keine Security-Capability.

## Folge

Erst nach einem grünen r83 darf ein Folge-Take den ersten echten Ownership-Schnitt vornehmen. Der bevorzugte erste Kandidat bleibt die Feedback-Security-Boundary.
