# r355 RetiredTonalSpanAuditScaffoldingCompaction

Parent: r354 `KeySignatureEnharmonicSpanWriteAuthorityRetirementStrictClippyRecovery` (real GREEN/released, 1092/1092, Master/Render byteidentisch zu r351, PDF 6/6 pixelidentisch).

r354 hat die spaete `KeySignatureEnharmonicSpan`-Write-Authority vollstaendig pensioniert. Damit sind mehrere historische Audit-Felder definitionsgemaess unveraenderlich geworden: `Attempted/Written/BehindBarsStructuralGateOpen` bleiben immer `false`, geplante Key-Signature-/Span-/Restore-Writes immer `0`, und Before/After-Paare sind bei vorhandener Beobachtung identisch. Diese Werte tragen keine fachliche Entscheidung und keine fail-closed Capability mehr.

r355 ist deshalb ein bewusst kleiner Besen-Take. Er entfernt nur dieses nachweislich tote Scaffolding:

- die einfeldrige `KeySignatureEnharmonicSpanRow`-Zwischenstruktur entfaellt; die bestehende Klassifikationsfunktion liefert direkt die konsumierte Decision-Klasse;
- `KeySignatureEnharmonicSpanRenderWriteAudit` behaelt nur noch tatsaechlich beobachtete oder zwischen "Quelle vorhanden" und "Quelle fehlt" variierende Daten;
- konstante Retirement-Zustaende (`Attempted/Written`, Gate, Plan-/Write-Zaehler, Before/After-Spiegelungen, DoubleBarlinesWritten) werden nicht mehr als zustandsbehaftete Felder transportiert;
- `Default` fuer die historische Writer-Audit-Struktur entfaellt; der No-Render-Source-Zweig wird explizit als observation-unavailable materialisiert;
- der bestehende Reporttext bleibt semantisch und fuer den normalen Render-Source-Pfad textuell gleich: die pensionierten Zustandswerte werden direkt als konstante Retirement-Fakten projiziert;
- die vier vorhandenen Regressionstests bleiben erhalten und pruefen weiterhin: kein Materialisierungsplan, keine Materialisierung, beobachtete Invarianten stabil, Owner-State pensioniert.

Nicht Bestandteil dieses Takes:

- keine neue musikalische DecisionAuthority;
- keine Aenderung an `KeySignatureEnharmonicSpelling`;
- keine Tonalregel, Schwelle, Passorder oder Kandidatenselektion;
- keine StructuralWriteAuthority-, Repair-, Recovery- oder Writer-Erweiterung;
- keine Security-/Gate-Lockerung;
- keine `#[allow(dead_code)]`-Ausnahme;
- keine breite projektweite Clippy-Bereinigung.

Produktive Zeilenbilanz gegen r354 in den zwei geaenderten Rust-Dateien: ca. **99 hinzu / 185 entfernt, netto -86**. Es werden keine neuen Funktionssymbole eingefuehrt.

Erwartung: 1092/1092 Tests, Clippy `architectural_budget: 0`, Master-/Render-MusicXML und PDF unveraendert. Nach GREEN ist dieser kleine Cleanup abgeschlossen; es folgt bewusst keine automatische naechste Authority-Insel, sondern eine Bestandsaufnahme.
