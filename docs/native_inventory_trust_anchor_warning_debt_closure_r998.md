# r998 NativeInventoryTrustAnchorWarningDebtClosure

r998 reappliziert die enge Warning-Debt-Closure direkt auf den real ausgefuehrten r996 `InventoryReferenceEvidenceClosure`. Der Kandidat r997 wurde nach erfolgreichem Pre-Take-Diff-Gate installiert, scheiterte jedoch im anschliessenden Strict-Clippy-Build compile-time an einem stale `LAUNCHER_INVENTORY_EXPORT_FNV1A64`-Sollwert in `src/identity_gate_trust_anchor.rs` und ist deshalb rejected Failure Evidence, niemals Parent.

Die fachliche Warning-Ursache aus r996 bleibt unveraendert: `closed_unreachable_removal_components` in `src/bin/etuschlopf_launcher/inventory_export.rs` ist seit ARCH-4 nur noch Regressionstest-Helper und wird deshalb explizit `#[cfg(test)]` gebunden. Es wird keine Warning-Suppression eingefuehrt.

r998 schliesst zusaetzlich ausschliesslich die von r997 offengelassene Trust-Anchor-Koordination: der Compile-Time-Pin `LAUNCHER_INVENTORY_EXPORT_FNV1A64` wird auf den aus den finalen `inventory_export.rs`-Bytes berechneten Wert `fnv64:8d7fcf79a6f0717a` neu gebunden. Keine weitere Trust-, Gate- oder Runtime-Semantik wird veraendert.

ESI-3, ARCH-4, CMS-1 und XLSX-3 bleiben semantisch unveraendert. Reference-/Reachability-Aufloesung, A-/B-Evidence, A/B/C/D/OFFEN-Regeln, JSON/XLSX-Schema, Table-AutoFilter, Assistant-Export-Bindung sowie Musik-, MusicXML-, PDF-, Writer-, Repair-, Selection- und Release-Authorities bleiben unveraendert.

Der Take ist `KodexOnly` mit `warning_debt_closure=closed_current_warning_debt_v1`. Erwarteter Strict-Testbestand bleibt 1.303. Ziel ist ausschliesslich ein kompilierbarer Trust-Anker bei weiterhin `TotalWarnings: 0` und damit eine dunkelgruene ESI-3/ARCH-4/XLSX-3-Inventurbaseline.
