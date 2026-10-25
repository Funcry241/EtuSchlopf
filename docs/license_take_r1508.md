# r1508 – LicenseNoticeAndPdfiumDiagnostics

Version 0.10.1839, ChangeClass RefactorOnly. Parent r1507 ist durch den
mitgelieferten nativen Export als angenommen belegt, Take-SHA-256:
`2a2ce714d783101956ace0753c6ff2f9b3a9a8603c143081dc23a5288a286a66`.
Der Parent meldet 2465 bestandene Tests, 0 Fehler, 1 ignorierten Test,
Build/Clippy Exit 0 und 10/10 Korpusprojekte.

## Inhalt und Paketgrenze

Die Lizenzbeigaben liegen vollständig unter `docs/licenses`, die Erläuterung
unter `docs/THIRD_PARTY_NOTICES.md`. Damit sind sie Teil des bereits erlaubten
verwalteten Baums und werden auch von den bestehenden Source-Snapshots erfasst.
Die MIT-Lizenz des eigenen Codes ist dort ebenfalls enthalten. Der öffentliche
Git-Baum behält zusätzlich seine bestehende Root-Datei `LICENSE`.

Die 189 geprüften Registry-Paketversionen stammen aus zwei unveränderten
Parent-Lockfiles und zwei separat erzeugten Prüf-Auflösungen unter
`docs/license_lock_snapshots`. Letztere sind reine Audit-Belege. Sie werden nicht
in die qualifizierte Schemaschlopf-Komponente oder den Sensor-Targetbaum eingebaut
und fixieren keine native Laufzeitauflösung. Ein abweichender Sensor-Build braucht
ein Inventar seiner tatsächlich aufgelösten Pakete.

Der frühere GitHub-Entwurf wurde deshalb für den Take korrigiert: keine neue
Top-Level-Lizenzstruktur, keine Root-Cargo.lock-Nutzlast, keine neue Datei im
eingefrorenen qualifizierten Komponentenbaum und keine Sensor-Cargo-Änderung.
Die bestehenden Gates werden nicht erweitert oder abgeschwächt.

Die separaten Python-Diagnosen verwenden pypdfium2 5.14.0 statt PyMuPDF. Beide
PDF-Vergleichseinstiege nutzen dieselbe Engine. Metadatenänderungen bleiben von
sichtbaren Änderungen unterscheidbar; Seitenzahl und Seitengröße werden geprüft.
Fehlende Bibliothek oder ungültige PDF-Daten ergeben einen unvollständigen Befund.
Der Bericht hält pypdfium2- und PDFium-Version fest. Alte PyMuPDF-Raster sind kein
PDFium-Referenzbeleg. Die normale Rust-Pipeline startet diese Python-Werkzeuge
nicht; Python-Pakete werden durch den Take nicht automatisch installiert.

## Lokal geprüfte Evidenz

- 20/20 synthetische Python-Diagnosetests bestanden.
- Beide PDF-Einstiegspunkte auf gleiche, geänderte und zusätzliche Seiten geprüft.
- Alle zehn PDFs des r1507-Exports, zusammen 50 Seiten, beim Selbstvergleich
  mit PDFium bei 144 dpi pixelgleich. Dies ist kein neuer Korpuslauf.
- Originalarchive der 189 Paketversionen gegen ihre Lock-Prüfsummen geprüft;
  Lizenztexte und deren Zuordnung vollständig vorhanden. Identische Texte sind
  bytegenau dedupliziert.
- Basic-Pitch-Primärdownload entspricht der vorhandenen Modell-Prüfsumme.
- Sämtliche `.rs`-Dateien, qualifizierte Komponentenbytes, sechs Original-XSDs,
  Root-Cargo.toml, Root-Starter und Root-Cargo.lock bleiben gegenüber r1507
  bytegleich. Nur der vorgeschriebene Change-Manifest-Text unter `src/` ändert sich.

Der unveränderte Parent-Prüfer wird separat gegen den vollständigen eingehenden
Baum ausgeführt. Der native Windows-Gesamtlauf und die Take-Annahme bleiben bis
zum Benutzerlauf ausstehend. Erwartung für RefactorOnly: alle 20 XML-Ausgaben
bytegleich zum angenommenen r1507, unveränderte PDF-Notenbilder.

## Anwendung und GitHub

1. Das ZIP `EtuSchlopf_v0_10_1839_r1508_take.zip` im üblichen Download- oder
   Projektordner bereitstellen. Nicht über einen neueren angenommenen Stand
   einspielen; Parent dieses Takes ist ausschließlich r1507.
2. Im bestehenden r1507-Projekt den normalen Befehl ausführen:

   ```cmd
   .\run.cmd --take --all
   ```

3. Erst nach erfolgreicher nativer Annahme den aktualisierten
   [GitHub-Entwurf PR #1](https://github.com/Funcry241/EtuSchlopf/pull/1) öffnen,
   `Ready for review`, dann `Merge pull request` und `Confirm merge` wählen.
   Erneutes manuelles Hochladen der Dateien ist nicht erforderlich.

Git-Commit, Take-ZIP-Hash und native Annahme bleiben unterschiedliche Identitäten.
Der PR enthält zusätzlich die bestehenden Git-Dateien, während das native ZIP
nur den vom Parent erlaubten Take-Vollbaum enthält.
