# r318 – ProductiveXmlSemanticMutationSurfaceAudit

> Revisionshistorie: r316 wurde vom Take-Change-Control vor Installation abgewiesen. r317 passierte diesen Gate, scheiterte danach jedoch im Strict-Clippy-Compile mit `E0432`: der Audit importierte `rust_source_scan_primitives` ueber ein Modul, das im produktiven etuschlopf-Binary nur unter `#[cfg(test)]` existiert. Beide Kandidaten sind nicht installiert und werden nicht wiederverwendet. r318 bleibt Parent r315.

## Ausgangspunkt

Parent ist der real installierte und im Vollkorpus bestaetigte Stand `v0.10.1839 r315 StructuralRestBeatMaterializationChordAnchorOwnerClosure`: 10/10 Projekte GREEN, `FinalExit 0`, `ReleaseDecision released`, `ActiveRepairTarget none`.

r311 bis r315 haben zugleich eine Architekturgrenze sichtbar gemacht: Der bestehende `ProductiveWritePath`-Security-Scanner ist fuer seine bewusst enge physische Write-Klasse korrekt, aber seine Nullzahl ausserhalb der `StructuralWriteAuthority` ist kein vollstaendiger Beweis dafuer, dass ausserhalb dieser Authority keinerlei produktive MusicXML-Semantik transformiert wird. Insbesondere koennen XML-Rekonstruktionen ueber `String::with_capacity`/`push_str`, anschliessende Rueckgabe oder direkte Rebindings wie `*xml = out` sowie `layout_xml`-Adoptionen ausserhalb dieser engeren syntaktischen Surface-Klasse liegen.

## Compile-Closure gegen r317

r318 aendert gegenueber dem verworfenen r317 nur die Abhaengigkeit des neuen Audits: Die Source-Maskierung und der FNV-Inventarfingerprint werden audit-lokal berechnet. Es wird kein test-only Build-/Security-Modul in den produktiven Modulgraph exportiert und kein Security-Pin geaendert. Die beobachteten Signalklassen, Reportprojektion, Domaenenzuordnung und Truth Boundary bleiben gleich.

## Ziel

r318 fuegt deshalb ausschliesslich einen read-only Architektur-Audit hinzu. Der gepinnte Security-Scanner und sein Build-Vertrag werden nicht veraendert. Der neue Audit beantwortet eine andere Frage:

> Welche produktiven Rust-Funktionen zeigen syntaktische Evidenz fuer MusicXML-Rekonstruktion, XML-Buffer-Rebinding oder XML-Zustandsadoption, auch wenn sie nicht Teil des engeren physischen Write-Inventars sind?

Die Ergebnisse werden in die bereits bestehende `MUTATION-OWNERSHIP-EVIDENCE-BOUNDARY` projiziert. Es entsteht keine zweite Release-Wahrheit und kein neuer Gate-Owner.

## Beobachtete Signalklassen

Der Audit inventarisiert pro produktiver Funktion:

- direkte XML-Rebindings wie `*xml = ...`;
- Adoptionen in produktive XML-Zustaende wie `candidate.layout_xml = ...` oder `best.layout_xml = ...`;
- XML-nahe String-Rekonstruktionen aus `String::with_capacity`/`String::new` plus `push_str`, wenn die Funktion einen MusicXML-/Notationskontext besitzt;
- vorhandene Abdeckung durch das bestehende `PRODUCTIVE_WRITE_PATH_SURFACES`-Inventar;
- interne Lage in `src/structural_write_authority/`;
- delegierte Aufrufe in die `StructuralWriteAuthority`;
- grobe Domaenenzuordnung fuer Rest, Beam, Clef/Register/Octave, Staff/Voice, HiddenMaterial, Layout, Tonal, Articulation und Other.

Offensichtliche Report-/Summary-/Proof-/Gate-Stringbuilder werden aus der XML-Builder-Klasse ausgeschlossen. Die Klassifikation bleibt bewusst diagnostisch: Ein Treffer ist weder Defektbeweis noch Bypass-Beweis und erteilt kein Schreibrecht.

## Truth Boundary

r318 aendert die Semantik der bestehenden Zeile `SingleMandatoryWriteBoundary` nicht und weitet ihren Security-Beweis nicht stillschweigend aus. Im Bericht steht deshalb explizit:

- bestehender `ProductiveWritePath`-Security-Scanner unveraendert;
- neuer `SemanticMutationSurfaceAudit` ist reine Source-Observation;
- `candidate surface != defect`;
- `candidate surface != bypass`;
- `candidate surface != write authorization`.

Damit koennen beide Inventare nebeneinander korrekt gelesen werden: das bestehende Gate fuer die gepinnte physische Write-Klasse und der breitere Architektur-Audit fuer die spaetere DomainOwnership-Konsolidierung.

## Keine Mutation

`ChangeClass=AuditOnly`.

Keine Aenderung an:

- MusicXML, Render-MusicXML oder PDF;
- Pass-Reihenfolge;
- Kandidatenauswahl;
- Rest-/Beam-/Clef-/Staff-/Voice-Entscheidungen;
- StructuralWriteAuthority;
- Security-/Take-Gates;
- Schwellen oder musikalischen Regeln;
- Repair-/Recovery-/Writer-Reichweite.

Der Audit liest ausschliesslich den installierten Rust-Quellbaum und das bereits build-time gebundene physische Write-Inventar.

## Regressionen

Vier allgemeine Audit-Regressionen binden:

1. XML-Rekonstruktion plus `*xml = out` wird als nicht vom physischen Inventar abgeleitete Semantikflaeche erkannt.
2. Eine explizit autorisierte Authority-Adoption wird beobachtet, aber nicht als hochkonfidenter externer Bypass klassifiziert.
3. Ein reiner Report-Stringbuilder ohne MusicXML-Semantik wird nicht als Kandidat inventarisiert.
4. Audit- und Security-Quellwurzeln werden aus dem produktiven Musikscan ausgeschlossen.

Ausgehend von r315 mit 1067 integrierten Tests sind nach r318 **1071/1071** Tests erwartet. Der reale Windows-Lauf bleibt autoritativ.

## Folgeschritt

r318 autorisiert noch kein Rest-Refactoring. Erst der reale r318-Bericht soll zeigen, welche Domaenen tatsaechlich mehrere hochkonfidente, nicht vom engeren physischen Scanner abgedeckte Semantikflaechen besitzen. Danach kann ein eigener `DomainOwnershipGraph`-Take geplant werden; Rest ist aufgrund r311-r315 ein starker Kandidat, aber nicht vorweggenommen.
