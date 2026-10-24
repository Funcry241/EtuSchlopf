# r90 – Identity Locator Cleanup and Corpus Security Decoupling Recovery

## Ausgangspunkt

Autoritativer Parent ist weiterhin `0.10.1839 r87`. Die zuvor erzeugten r88/r89-Kandidaten sind **nicht als Parent zu verwenden**: Im r88-Trust-Anchor blieben zwei Testassertionen auf den alten FNV-Werten fuer `identity_debt_build.rs` und `launcher/assistant_export.rs`. Ein realer Testlauf muss das fail-closed ablehnen bzw. zurueckrollen.

## Zweck

Dieser SecurityOnly-Take materialisiert den beabsichtigten r88-Security-/Launcher-Schnitt erneut, diesmal mit vollstaendig synchronisierten Compile-Time- **und Test-Pins**.

- historische `do_not_repair_m005_again`-Instruktion wird durch eine fresh-evidence-generische Regel ersetzt;
- Launcher-Visual-Audit bindet Korpusvollstaendigkeit an `expected_project_count` statt an die Literalzahl 10;
- `identity_debt_build.rs` pinnt den geaenderten Assistant-Export;
- `identity_gate_trust_anchor.rs` bindet sowohl die Compile-Time-Assertions als auch die entsprechenden Regressionstest-Assertions an die finalen Bytes.

Keine Musikmutation, keine Writer-/Renderer-/Pipeline-Semantik, keine Kalibrierung und keine Korpus-Mitgliedschaftsregel wird geaendert.

ChangeClass: `SecurityOnly`.
