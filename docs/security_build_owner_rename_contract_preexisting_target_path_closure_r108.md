# r108 – Security Build Owner Rename Contract Preexisting Target Path Closure

## Zweck

r107 hat den geschlossenen Vertrag fuer die spaetere byteidentische Umbenennung des Cargo-Build-Owners eingefuehrt. Beim Aufbau des konkreten Renames zeigt sich eine rein vertragstechnische Randbedingung: aktive Security-Dateien koennen den kuenftigen Zielpfad bereits als zugelassenen Security-Pfad kennen. Eine rueckwaertige globale `new -> old`-Textsubstitution wuerde solche bereits vorhandenen Zielpfadreferenzen faelschlich als zusaetzliche Aenderung interpretieren.

r108 aendert ausschliesslich diese Referenzvergleichsprimitive. Der Vergleich laeuft jetzt positionsgebunden vom aktuellen zum eingehenden Text und erlaubt nur, dass konkrete Vorkommen des alten Build-Owner-Pfads durch den neuen Pfad ersetzt werden. Bereits vorhandene Zielpfadreferenzen duerfen bytegleich stehen bleiben. Jede andere Byteaenderung bleibt verboten.

## Sicherheitsgrenze

Der Rename-Vertrag bleibt unveraendert eng:

- nur `SecurityOnly`;
- nur der Cargo-`package.build`-Owner;
- alter und neuer Owner im selben Security-Parent;
- Build-Script-Bytes identisch;
- bijektive SecurityNegativeCheck-Menge;
- identische Signaturen, Bodies und Rejection-Surfaces;
- Execution-Anchor nur `alter_owner::main#1 -> neuer_owner::main#1`;
- Referenzquellen duerfen ausschliesslich einzelne alte Pfadvorkommen durch den neuen Pfad ersetzen;
- keine Musik-, Writer-, Renderer-, Pipeline-, Report-, Console-, Recovery- oder Kalibrierungssemantik.

Ein Regressionstest konserviert explizit den Fall, dass eine Referenzquelle den kuenftigen Zielpfad bereits vor dem Rename als zugelassenen Pfad enthaelt.

r108 fuehrt den Rename selbst noch nicht aus.
