# r1238 LegacyPhysicalRetirementDependencyClosure

## Autoritative Basis

Version 0.10.1839 bleibt unverändert. Accepted Parent: r1233 MaxiLegacyReductionWarningClosure, Acceptance SHA-256 0361355c1fd99703b80f996ad8a06364761fc77099009679af0c1a3ccc00a45d. Grundlage ist der gebundene restored-parent Source-Snapshot im Export assistant_export_latest(20261004-064631).zip. r1234–r1237 sind abgelehnte Versuche, keine Parents. r1237 wurde mit Exit 101 auf r1233 zurückgerollt; sein Strict-Clippy-Log enthält 28 dead_code-Fehler.

## Abgeschlossener Rückbau

- Der bereits test-only gewordene RenderXmlSnapshotReader samt seinem isolierten Legacy-Fallback-Test wird physisch gelöscht; der Console-Import bleibt produktiv.
- Bereits ausschließlich über Funktionswert-Keepalives erhaltene MI-A06/07/08/09/10-Körper und ihre Adapter werden entfernt. Reale IR7-NO-WRITE-/Readback-Tests bleiben erhalten.
- Der disconnected LowerRestLane-Planer wird einschließlich LowerStaffRestLaneSequence, LowerStaffGapWithRestLanePolishState, Sequenzrenderer und Plan-Typ entfernt. Die fünf weiterhin produktiv verwendeten Beobachtungs-/Suppression-Pfade behalten ihre gemeinsame Observation; nur deren nicht mehr gelesene start/end-Felder entfallen.
- Die fünf vom unveränderten Receipt-Kodex verlangten Macro-Aufrufe werden mit ihren Varianten, Match-Armen und der unveränderten Macro-Definition ausschließlich unter cfg(test) kompiliert. Der reale Source-/Range-Vertrag wird durch synthetische Empty-/Invalid-Range-Tests ausgeführt. Die aktiven drei Rest-Unterdrückungsvarianten behalten wortgleiche Bedingungen.
- PitchedDurationClear und CrossStaffBeam einschließlich ihrer ausschließlichen Helfer werden ausschließlich unter cfg(test) kompiliert. Der bisherige CrossStaffBeam-Test bleibt erhalten; ein realer ClearDuration-Test bindet Dauer, Voice-Reihenfolge und Tie-/Beam-Entfernung. Der Move-only-Test schneidet jetzt an seiner tatsächlichen Testmodulgrenze, damit neue test-only Items den Source-Test nicht versehentlich verkürzen.
- Unbenutzte produktive Reexports werden entfernt. Keine allow-/expect-Unterdrückung, Fake-Invocation oder neue Keepalive-Zeile.

## Bewusst erhaltene Vertragsfamilien

E031/Tenuto bleibt mit vollständigem Parent-Probe-Pfad, Mint-/Descriptor-/Enum-Oberfläche und echtem Receipt-Disposition-Test bytegleich. RA5/AccentReserve bleibt mit vollständigem NO-WRITE-Body, Sink-Signaturreferenz und echtem NoWrite-Test bytegleich. Der derzeitige Receipt-Kodex bindet E031 und die fünf Macro-Aufrufe ausdrücklich; eine vollständige physische Pensionierung dieser Oberflächen benötigt einen separat zuständigen Retirement-Kodex. r1238 behauptet weder SSC-K2-Abschluss noch N_A = leer.

Alle 61 include_bytes!-gebundenen Governance-/Trust-Quellen, Cargo.toml, Root-Cargo.lock und die gesamte Master-/Render-Identity-/Lineage-Implementierung sind bytegleich zum Parent. Root-Cargo.lock wird nicht in das eingehende Paket aufgenommen. Governance, Gates, Population-/Exclusion-Tabellen und Physical-Write-Budgets werden nicht verändert. RA3/MI-A02 bleibt der bestehende fail-closed Runtime-Guard.

## Zeilen- und Testbilanz

Source/Test umfasst hier src/**/*.rs und tests/**/*.rs, ohne Manifest, Dokumentation oder tools:

- +63 / -1048 = netto -985 physische Rust-Zeilen.
- Rust-Dateien: 293 -> 292; Zeilen: 236408 -> 235423.
- Physische #[test]-Inventarstellen: 1660 -> 1657. Entfernt: ein ausschließlich alter SnapshotReader-Test und vier Tests der physisch entfernten TieMerge-Legacy-Helfer. Hinzugefügt: zwei ausführbare Tests an den verbleibenden test-only Authority-Verträgen. Produktive Owner-/Receipt-/Readback-/NO-WRITE-Tests bleiben erhalten.
- Ausgeführte Main-Tests: 1131; All-Targets-Tests im lokalen Testprofil: 1676/1676 PASS.

Der Source-/Test-Patch docs/r1238_parent_source_delta.patch wurde auf einer isolierten Parent-Kopie mit git apply --check und git apply ausgeführt. Der reproduzierte komplette Source-/Test-Baum ist bytegleich zum finalen Candidate (ausgenommen separat generiertes Take-Manifest).

## Ausgeführte Validierung

Runtime: Linux x86_64, rustc/cargo/clippy 1.98.0. Der vorgeschriebene Toolchain-Vertrag bleibt unverändert.

- Parent Take-Change-Control dependency_preflight und vollständige incoming_take-Zulassung: PASS; RefactorOnly, keine gepinnten Architekturänderungen, keine Calibration-Deltas, keine Unknown-Capabilities.
- Cargo check --locked --all-targets: PASS.
- Cargo test --locked --all-targets --jobs 2 mit test.debug=0 und test.opt-level=0: PASS, 1676 Tests über 15 Targets; keine Compilerwarnung.
- Cargo clippy --locked --profile strict --jobs 2 --color never -- -D clippy::correctness -D clippy::suspicious -D clippy::perf: PASS, null Warnungen.
- Cargo --config profile.strict.package.etuschlopf.opt-level=0 test --locked --profile strict --all-targets --no-fail-fast --jobs 2: PASS, 1676/1676 Tests über 15 Targets; keine Compilerwarnung.
- Finale Manifest/FNV-/Tree-/ZIP- und Patch-Reproduktionsprüfung: PASS.

warning_debt_closure=closed_current_warning_debt_v1 bleibt ausdrücklich gebunden. Die aktuelle Null-Warnungsanforderung wird weder durch Suppression noch durch Entfernen der Manifestbindung umgangen.

## Ausführungsgrenze und Installation

Windows-Bootstrap/Reexec, optimierter Windows-Produktivbuild, 10/10-Korpus, MusicXML-Byteparität und PDF-Pixelparität sind hier NICHT AUSGEFÜHRT. Die erwartete Produktwirkung ist unverändert: die entfernten Körper waren bereits unerreichbare bzw. NO-WRITE-Altlasten. Der lokale Compiler-/Testnachweis ersetzt keine frische Korpus-Evaluation.

ZIP in transfer ablegen; andere nicht-abgewiesene Take-ZIPs aus transfer entfernen, damit die Auswahl eindeutig bleibt. Aus dem Projektordner genau ein Befehl: ./run.cmd --take --all. Kein --force. Erst Exit 0 und der erfolgreiche transaktionale Take-Commit machen r1238 accepted. Der normale Windows-Lauf führt alle bestehenden Gates aus; ein echter Fehler bleibt blockierend und transaktional rollbackfähig.
