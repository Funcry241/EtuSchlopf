# r85 – Feedback Compile-Time Security Owner Identity Decoupling

## Parent

EtuSchlopf 0.10.1839 r83 `SecurityNegativeCheckExactRelocationContractFoundationFromR80`.

## Ziel

Der erste reale Ownership-Cut aus `identity_debt_build.rs` darf den in r83 eingeführten exakten `SecurityNegativeCheck`-Relocation-Vertrag nicht dadurch umgehen, dass die verschobene Funktion gleichzeitig ihren eigenen Pfadvertrag ändern muss.

Die Feedback-Sicherheitsprüfung besitzt in r83 noch zwei historische Selbstidentitätskopplungen:

1. das erwartete `feedback_observable_sink_allowlist` wird direkt im Validator mit `src/identity_debt_build.rs` aufgebaut;
2. die Confinement-Prüfung nimmt `identity_debt_build.rs` über einen Dateinamen-Sonderfall von der Selbstprüfung aus.

Damit wäre ein direkter Owner-Wechsel nicht bytegleich möglich.

## Änderung

r85 verändert noch **keinen Security-Owner-Pfad** und verschiebt noch **keinen `SecurityNegativeCheck`**. Stattdessen wird der bestehende Vertrag owner-neutral vorbereitet:

- das erwartete Feedback-Sink-Register besitzt eine einzige deklarative Konstante;
- der Validator materialisiert daraus weiterhin exakt denselben geschlossenen `BTreeMap`-Vertrag;
- Compile-Time-Security-Owner werden aus der registrierten Rolle `compile_time_security_policy` abgeleitet;
- die Confinement-Ausnahme hängt nicht länger am Dateinamen `identity_debt_build.rs`;
- der aktuelle registrierte Owner bleibt in r85 weiterhin exakt `src/identity_debt_build.rs`;
- die reale beobachtbare Feedback-Surface und alle produktiven Regeln bleiben unverändert.

## Sicherheitswirkung

Der Sink-Vertrag wird nicht geöffnet: unbekannte, fehlende oder umklassifizierte Sink-Pfade bleiben fail-closed. Die neue Ableitung ändert ausschließlich die Quelle der Owner-Identität von einer lokalen Dateinamen-Heuristik auf den bereits geschlossenen expliziten Sink-Register-Vertrag.

Damit kann ein Folgetake den Feedback-Security-Validator unter dem r83-Relocation-Vertrag mit unverändertem Funktionskörper verschieben und ausschließlich den externen Owner-Vertrag aktualisieren.

## Nicht geändert

Keine Musikmutation, keine Writer-/Renderer-/Pipeline-Änderung, keine Console-Änderung, keine Threshold-Änderung, kein Security-Retirement, keine SecurityNegativeCheck-Relocation und keine Lockerung des Take-Change-Control.
