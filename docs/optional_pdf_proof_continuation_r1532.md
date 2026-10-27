# r1532 – N1 bei ausgewählter Render-XML

Voraussetzung: r1531 nativ angenommen. Dieser Refactor korrigiert nur die
PDF-Commit-Fortschreibung bei nicht angefordertem oder vorher blockiertem PDF.
Die XML-Quelle wird nicht umgewählt. Beide Pflicht-XML und deren strenge
Prüfungen bleiben erhalten. Kein musikalischer Eingriff, kein Zeit-/Taktimport.

Die produktiven Rust-Bytes entsprechen exakt dem lokal geprüften N1-Stand:
Build/Clippy grün, 2564 Tests bestanden, 0 fehlgeschlagen, 1 ignoriert.
Master- und Render-Quellenfall mit --no-pdf bestehen Erstlauf und Wiederholung.
Lokal wurde kein erfolgreicher PDF-Export und kein Windows-Lauf nachgewiesen.
Alle lokalen Laufbelege liegen im separat gelieferten Prüfstand
EtuSchlopf_Pruefstand_N1_BImport_2026-10-27.zip.

Native Annahme und Korpusvergleich dieses Takes stehen aus. Erwartet werden
unveränderte musikalische XML/PDF im üblichen nativen Lauf mit PDF.
