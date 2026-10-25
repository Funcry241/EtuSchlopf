# Diagnose-Hilfsmittel

Installierter Zusatz des RefactorOnly-Takes r1464 auf akzeptiertem r1463.
Die Werkzeuge werden separat gestartet. Sie ändern keine Notationsentscheidung,
keine Produktivquelle, keine Gates und keinen akzeptierten Laufbeleg. Ein Take-
Gesamtlauf startet die Python-Hilfsmittel nicht automatisch.

## Voraussetzungen

Python 3.10 oder neuer. Der Vorabcheck braucht die bestehende Rust-1.98-Toolchain
und den vorgesehenen MuseScore-4.7.4-Aufruf. Der PDF-Exportvergleich benötigt
zusätzlich pypdfium2 5.14.0 (PDFium). Bei Bedarf separat installieren:

```powershell
python -m pip install -r tools\assistant_diagnostics\requirements.txt
```

Die PDF-Diagnose verwendet PDFium statt PyMuPDF. Der Bericht nennt die Versionen
von pypdfium2 und PDFium. Pixelgleichheit wird immer mit derselben installierten
Engine geprüft; frühere PyMuPDF-Raster sind kein Referenzbeleg für PDFium-Raster.
Die Bibliothek und ihre Binärdateien werden separat installiert. Ihre eigenen
Lizenzhinweise bleiben maßgeblich; siehe `THIRD_PARTY_NOTICES.md` im Projektroot.

Der Diagnosezusatz verändert weder Cargo-Abhängigkeiten noch Root-Cargo.lock und
installiert keine Python-Pakete automatisch.

## Vorabcheck aus der Projektwurzel

```powershell
$diagOut = Join-Path $env:TEMP ('etuschlopf-preflight-' + [guid]::NewGuid().ToString('N'))
tools\assistant_diagnostics\vorabcheck.cmd --tree . --out $diagOut --renderer 'C:\Program Files\MuseScore 4\bin\MuseScore4.exe'
```

Für eine Scratch-Diagnose `--tree` auf deren Projektwurzel setzen. Der
Ausgabeordner muss neu sein und außerhalb dieses Baums liegen. Standardbasis
ist die eingefrorene r1463-Dateireferenz. Für spätere Takes beziehungsweise
andere Diff-Basen zusätzlich `--reference-tree` mit einer separat überprüften
Kopie des tatsächlich angenommenen Elternstands angeben. Das Hilfsmittel kann
dessen Annahme selbst nicht beweisen. Diagnosebaum und Referenz dürfen nicht
derselbe Pfad sein. Die verwalteten Pfadrollen entsprechen dem r1463-Vertrag;
bei einem späteren Vertragswechsel ist auch diese Hilfsprojektion zu prüfen.

Bei einem Wrapper `--renderer` auf dessen Interpreter setzen, feste Argumente
mit `--renderer-arg` ergänzen und mit `--renderer-binary` die tatsächliche
MuseScore-Binärdatei benennen. Absolute Wrapperpfade verwenden. Die Wrapper-
Umgebung muss selbst Display und Bibliotheken korrekt setzen.

Geprüft werden Quellen-/Manifest-Hashes, undeclared Änderungen gegen die
gewählte Referenz, Versionen und ein tatsächlicher PDF-/SVG-Smoke mit Exit 0
ohne Force. Root-Cargo.lock ist lokaler Zustand; verwaltete verschachtelte
Locks sind eingeschlossen. Das Skript korrigiert keine Hashes. Fehlende oder
falsche Eingaben führen zum Abbruch und einem `preflight.json`, soweit der
Prozess regulär bis zum Schreiben dieses Berichts gelangt.

`PREFLIGHT_READY` ist nur die Bereitschaft für die nächsten Prüfungen, keine
kanonische Zulassung. ChangeClass, tatsächlicher parent-relativer Take-Diff,
Capability-/Pin-/Writer-/Receipt-Regeln und tatsächlicher Geometrie-Readback
bleiben separat zu prüfen. Kein Build oder Etüdenlauf wird gestartet.
Der optionale Timeout betrifft ausschließlich kleine Versions-/Smoke-Aufrufe.

## Zwei Assistant-Exporte vergleichen

```powershell
tools\assistant_diagnostics\exportvergleich.cmd 'C:\Pfad\alt.zip' 'C:\Pfad\neu.zip' --out 'C:\Pfad\neuer_vergleichsordner'
```

Ausgabe: `vergleich.md` und `vergleich.json`. Verwaltete Quellen, vollständiger
Source-Snapshot einschließlich lokaler Locks, XML-Bytes, PDF-Dateihashes und
PDF-Pixel werden getrennt ausgewiesen. Standard: 144 dpi. Testbelege erscheinen
als gemeldeter Ausführungslog, gemeldete Receipt-Wiederverwendung oder unbekannt.
Gemeldete Build-/Clippy-/Exit-Werte bleiben sichtbar. Die Autorität dieser
Belege oder eine native Take-Annahme werden nicht erneut geprüft.

Exit 0 bedeutet gleiche verwaltete Quellen, XMLs und PDF-Raster; 1 bedeutet
vollständig verglichene Änderungen, 2 einen unvollständigen Vergleich oder
Eingabefehler. Pixelgleichheit gilt nur für die aufgezeichnete Raster-Engine
und Auflösung, nicht für interaktive PDF-Funktionen oder jede höhere Auflösung.
Die ZIP-Eingaben werden nur gelesen und nicht entpackt oder verändert.

## Diagnose vorbereiten und Hilfsmittel testen

`DIAGNOSE_CHECKLISTE.md` und `ablehnungen_vorlage.tsv` legen die benötigten
Diagnosefelder fest. Insbesondere bleibt fehlende Geometrie unbekannt und wird
nicht als intakte Gruppe gezählt. Vorgeschlagene Reparatur ist kein Commit-Beleg.

```powershell
tools\assistant_diagnostics\tests.cmd
```

Unter Linux beziehungsweise ohne Starter:

```text
python -B -m unittest discover -s tools/assistant_diagnostics/tests -v
```

Immer `-B` verwenden, damit keine Python-Bytecode-Dateien im verwalteten Baum
entstehen. Tests sind synthetisch und unabhängig vom tatsächlichen Etüdenkorpus.
Der normale Strict-Cargo-Testvertrag wird nicht verändert. Die ergänzenden
Python-Tests ersetzen ihn nicht. Die Windows-Starter werden lokal in diesem
Linux-Arbeitslauf nur auf ihre Aufrufstruktur geprüft, nicht nativ ausgeführt.
