# r971 – LauncherDominatedKodexBasementRetirementCompileClosure

## Parent und Rejection-Historie

Parent ist der real installierte r969 `TestOnlyHorizontalLayoutParallelWriterRetirement`. r970 `LauncherDominatedKodexBasementRetirement` wurde beim Reexec wegen eines unvollstaendig nachgezogenen API-Signaturcuts (`E0061`) plus zwei daraus folgenden `unused variable`-Warnungen korrekt abgewiesen und automatisch auf r969 zurueckgerollt. r970 bleibt Failure Evidence und ist niemals Parent.

## Fachlicher Keller-Harvest

Der fachliche Cut ist derselbe geschlossene Kodex-Harvest wie im abgewiesenen r970-Kandidaten:

1. Der Firebreak-Console-Focus ist compile-time deaktiviert; Schalter, vier exklusive Konstanten und der unerreichbare Ausgabeblock entfallen.
2. `GLOBAL_OUTPUT_PRECISION_ENABLED` ist dauerhaft wahr und waehlt ausschliesslich `compact_global`; Schalter und tote `verbose`-Alternative entfallen.
3. Der Controlled-Acceptance-PDF-Review-Schalter ist dauerhaft wahr. Die dominierten reinen Praedikate fuer PDF-Inclusion/Review entfallen; `auto_visual_shape_proxy_stable` bleibt wegen der lebenden `auto_visual_changed_label`-Berichterstattung erhalten.

## Compile-Closure gegen den abgewiesenen r970

- alle sieben Call-Sites von `assistant_export_includes_pdf` verwenden die neue Ein-Parameter-Signatur;
- die beiden Zaehlschleifen binden kein unbenutztes `ledger` mehr;
- `assistant_export_zip_contract_audit` traegt keinen funktionslosen `final_exit`-Parameter mehr;
- die bestehenden Trust-Pins fuer Launcher und Assistant-Export sind auf die finalen korrigierten Bytes synchronisiert.

## Semantik und Bilanz

MusicXML-, Renderer-, Writer-, Repair-, Recovery-, Outcome- und Korpus-Semantik bleiben unveraendert. Es entsteht keine neue Abstraktion, kein neues Gate, kein neuer Pin und keine neue ChangeClass. Die vier eigentlichen Kodex-Dateien schrumpfen netto um 58 Rust-Zeilen; `identity_gate_trust_anchor.rs` ist zeilenneutral. Root-`Cargo.lock` bleibt LocalTransactional und ist kein Take-Payload.
