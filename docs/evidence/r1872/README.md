# Native Windows-Abnahme r1872

Die Windows-Abnahme vom 15. November 2026 dokumentiert den Schritt von r1871
auf **r1872 / 0.10.1839** anhand von Schuberts Deutschem Tanz As-Dur D. 783
Nr. 15, Aufführung p18.

| Prüfung | Ergebnis |
| --- | --- |
| Schubert-Einzellauf | Exitcode 0, keine Blocker, Warnungen oder Hinweise |
| Build und Strict Clippy | Bestanden |
| Rust-Tests | 3.097 bestanden, vier ignoriert |
| PDF-Ausgabe | Renderer-Freigabe für zwei Seiten mit sieben Systemen |

## Notenbild

Im ersten vollen Takt steht das E wieder als Viertel im linken Begleitakkord.

## Notenerhalt und Quellauswahl

Der Abschlussbericht weist **100 % Notenerhalt nach dem Erhaltsmaß des Laufs**
aus. Die Quellauswahl zählt die Ereignisse an einem eigenen Prüfpunkt. Ihr mit
diesem Lauf verknüpfter Messbeleg (CMS-1) erfasst 330 kanonische Quellereignisse,
von denen der Auswahlplan 323 übernimmt und sieben ausschließt. Die 100 % sind
eine Erhaltskennzahl mit eigener Bezugsmenge. Wie das Verfahren und die
Prüfgrenzen zusammenhängen, erläutert die [Mathematik-Seite in Kapitel 9](../../MATHEMATIK.md#kapitel-9).

## Belege

- [Annahmebeleg](native_acceptance.txt)
- [Laufergebnis](run_outcome_audit.txt)
- [Prüfsummen des nativen Quellstands](accepted_source_sha256.tsv)
- [Prüfsummen der GitHub-Dokumentation](github_projection_sha256.tsv)

Der vollständige Windows-Quell-Snapshot hat den SHA-256-Wert
`e82644eb5b1e0278b5520cb49349a44ed51fbfd5df446ca795565399d44c1398`.
Der separate [GitHub-Paketlauf](https://github.com/Funcry241/EtuSchlopf/actions/runs/43755526495)
prüft die eigenständige Windows-EXE und das synthetische Startbeispiel.
[Windows-Testversion r1872](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1872-windows-preview)
