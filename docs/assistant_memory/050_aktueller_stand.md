# 050 – Aktueller Stand

Stand der vorliegenden Prüfbelege vom 13. November 2026. Maßgeblich für die
Annahme eines Takes sind der Windows-Export und der darin gebundene
Annahmebeleg.

## Windows-Prüfstand r1792

Windows hat r1792 für Chopin op. 10 Nr. 3, p01 angenommen. Der Lauf endete
mit Exitcode 0, ohne Warnungen und mit 100 % Notenerhalt. Build, Strict
Clippy, 3.066 Tests und der PDF-Export bestanden. Vier Tests waren ignoriert.

Der MIDI-Seed-Owner berücksichtigt Sprungbelastung und Tastenbelegung
beider Hände sowie die bestehende Akkordaufteilung. r1792 prüft die
Spannweite bereits bei der ersten Aufteilung. Im lokalen Chopin-Vergleich
zu r1791 entfielen damit drei späte Spannweitenkorrekturen. Für Chopin
und BWV 847 waren die beiden MusicXML-Dateien bytegleich und die
gerenderten Seiten pixelgleich.

Die Chopin-Zeitzuordnung zwischen MIDI und WAV bleibt wegen zu geringer
gemeinsamer Anschlagsdeckung offen. Der r1792-Prüfumfang umfasst den
Windows-Einzellauf für Chopin und den lokalen Vergleich mit BWV 847.
Auf GitHub liegen Quellstand und Windows-Testdownload auf r1749.

## BWV 847 und technische Pflege

| Revision | Änderung | Beleg |
| --- | --- | --- |
| r1745 | Halbe mit gebundenem Achtel in Takt 24. Der Notenwert-Abgleich für BWV 847 ist abgeschlossen. | Windows hat r1745 angenommen. Im Referenzvergleich stimmen 754 Notenansätze, geschriebene Notenwerte und Haltebögen überein. |
| r1746 | E057 überspringt den frühen Cursor-Austausch ohne aktuellen gebundenen Plan. | Windows hat r1746 angenommen. |
| r1747 | Anfangspausen verwenden die Position und Metrik des tatsächlichen Takts. Gleichlautende numerische Taktnummern können unabhängige Entscheidungen nicht mehr zusammenlegen. | Windows hat r1747 angenommen. Build, Clippy und MuseScore waren erfolgreich. 3.041 Tests bestanden, vier wurden ignoriert. |
| r1748 | Der Pausen-Owner steuert die Folgerunden der Konsolidierung. Eine Runde ohne Änderung beendet die Wiederholung. | Lokal sind Zulassung, Clippy und 2.247 Hauptprogrammtests bestanden. Beide MusicXML-Dateien und die vier gerenderten PDF-Seiten stimmen mit r1747 überein. Ein Windows-Annahmebeleg für r1748 liegt hier noch nicht vor. |

Der bestätigte r1747-Export bindet den Take an die SHA-256
`31e6b0caff51bfcd108271dcb2dccb9a90a11c1a5c3ad278316d1f46803eaedb`.

## Geltungsbereich der Nachweise

Die früheren Windows-Exporte zu BWV 847 prüfen das Einzelprojekt. Die
[Mathe-Seite](../MATHEMATIK.md#historischer-vollkorpuslauf-r1683) dokumentiert
den Vollkorpuslauf r1683 mit seinem damaligen Stand. Ein aktueller
Vollkorpuslauf ist in den vorliegenden Exporten nicht belegt.

Die Schlöpfchen-Formel bleibt ein Entwurf für die Seitenfüllung.
Ein Versuch zur Seitenverdichtung setzt begründete sichtbare Mindestabstände
und einen frischen PDF-Vergleich voraus.
