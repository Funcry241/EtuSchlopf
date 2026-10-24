# r99 – Remaining Security Owner Partition A

## Ausgangspunkt

Parent ist der real gruen gelaufene Stand `0.10.1839 r98`
`SecurityBoundaryAdversarialOwnerExtraction`.

Nach r97 und r98 besitzen die Feedback-Structuralization-Security und die
adversariale Security-Matrix bereits eigene echte Child-Owner. Der verbliebene
Build-Policy-Monolith enthaelt jedoch weiterhin mehrere fachlich getrennte
Security-Verantwortungen direkt inline in `run()`.

## Ziel

Dieser Take bildet zwei weitere klare Owner-Grenzen, ohne vorhandene
SecurityNegativeCheck-Funktionen zu pensionieren oder umzubenennen:

1. `src/build_gate/source_integrity_security.rs`
   - Multi-Revision-/SHA-256-Owner-Bindung
   - Rust-Source-Scan-Primitive-Ownership
   - Productive-Write-Scanner-Pin
   - Thirty-Surface-Scanner-Pin
   - Security-Gate-Core-Ownership und path-stabile Negativchecks

2. `src/build_gate/recovery_transaction_order_security.rs`
   - Public-Artifact-Recovery-Fingerprint
   - Formal-Green-Vertrag
   - run.cmd-Recovery-Boundary
   - Take-/Launcher-/Public-Artifact-Transaktionsvertrag
   - Reihenfolge Public-Recovery vor Source-Rollback
   - Build-Cache-Retention-Reihenfolge

Beide Pruefgruppen waren in r98 bereits produktiv aktiv. r99 aendert die
Pruefbedingungen nicht, sondern materialisiert die zuvor inline liegenden
Bloecke als explizite Security-Owner, die weiterhin direkt aus `run()` aufgerufen
werden.

## Sicherheitsgrenzen

- ChangeClass: `SecurityOnly`.
- Kein `SecurityCapabilityRetirement`.
- Kein `security_negative_check_module_relocation`: Es wird keine bereits
  bestehende benannte Negativcheck-Funktion entfernt oder an einen anderen Pfad
  verschoben. Die bisherigen inline Checks werden in neue explizite Owner
  partitioniert; `run()` bleibt als Build-Security-Ausfuehrungsanker erhalten.
- Keine Musik-, Writer-, Renderer-, Pipeline-, Console- oder Calibration-Aenderung.
- Keine Schwellenwertaenderung.
- Keine Aenderung der Feedback-Observable-Sink-Allowlist.
- Die geschlossene `compile_time_security_policy`-Owner-Menge wird um die beiden
  neuen privaten Owner erweitert.
- Beide neuen Owner werden im Identity-Gate-Trust-Anchor bytegepinnt.
- Die adversariale Trust-Chain verlangt beide neuen Pins explizit.

## Architekturwirkung

`identity_debt_build.rs` sinkt statisch von 1836 auf 1457 Zeilen.

Die Reduktion ist kein Zeilen-Cleanup, sondern trennt zwei fachlich eigenstaendige
Security-Verantwortungen vom zentralen Orchestrator:

```text
identity_debt_build::run
  -> source_integrity_security::validate_source_integrity_security
  -> recovery_transaction_order_security::validate_recovery_transaction_order_security
```

Die produktive Entscheidung bleibt weiterhin fail-closed und zentral
orchestriert.

## Nichtziel

r99 ist noch nicht die Orchestrator-Closure. Die verbleibenden grossen
Fingerprint-/Feedback-/Take-/Inventory-Bloecke werden erst nach einem real
gruenen r99 neu bewertet. Dieser Take erzwingt keinen r100, falls die reale
Auswertung eine andere Grenze zeigt.
