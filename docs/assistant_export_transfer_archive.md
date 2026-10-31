# AssistantExportUtcZChronologyContract

> **Status:** Historischer Vertrag. Die Assistant-Export-Archivkopie nach
> `EtuSchlopfTransfer` ist seit v0.10.1839 r61
> `PersistedResultVocabularyTransferDomainSecurityClosure` pensioniert. Der
> `assistant_export_latest.zip` bleibt nur im Projekt-Root; das Transferverzeichnis
> ist fuer EtuSchlopf-Machine-Evidence reserviert.

Version: 0.10.1807 r01
ChangeClass: SecurityOnly

## Historischer Zielvertrag (pensioniert)

Die folgenden Abschnitte dokumentieren ausschliesslich den frueheren Vertrag.
Sie sind keine aktuelle Runtime-Anweisung.

Der bestehende `assistant_export_latest.zip`-Vertrag bleibt unveraendert erhalten.
Nach erfolgreicher Erzeugung dieses Latest-Exports publiziert der Rust-Launcher
zusaetzlich eine zweite, chronologische Archivkopie in ein konfigurierbares
Transferverzeichnis.

Standard:

```text
%USERPROFILE%\Documents\Programmieren\EtuSchlopfTransfer
```

Konfigurierbar ueber:

```text
ETUSCHLOPF_ASSISTANT_EXPORT_DIR
```

Ein relativer Wert wird gegen den aktuellen EtuSchlopf-Projekt-Root aufgeloest.

## Doppelvertrag

1. `assistant_export_latest.zip` im Projekt-Root bleibt der sofortige aktuelle
   Handover und wird weiterhin exakt wie bisher erzeugt.
2. Danach wird best-effort eine zweite Kopie in das Transferarchiv geschrieben.

Ein Fehler im Transferarchiv darf den bereits erfolgreich geschriebenen
`assistant_export_latest.zip` nicht invalidieren.

## UTC-Zeitvertrag

Die Zeitbasis der Archivnamen ist verbindlich `UTC_Z`. Sie wird ausschliesslich
aus `SystemTime` relativ zur Unix-Epoch abgeleitet. Lokale Windows-Zeit,
Zeitzonenoffsets sowie Sommer-/Winterzeit werden fuer den Dateinamen nicht
verwendet.

Das abschliessende `Z` ist Teil des verwalteten Dateinamenschemas und bezeichnet
UTC. Dadurch bleiben Dateinamen auch im Rueckstellfenster der Sommerzeit
eindeutig und lexikographisch chronologisch sortierbar. Der Launcher-Test deckt
das Rueckstellfenster am 25.10.2026 explizit mit zwei aufeinanderfolgenden
UTC-Zeitpunkten ab.

Der Archiv-Diagnoselog weist die Zeitbasis zusaetzlich als
`TimeStandard=UTC_Z` aus.

## Dateiname

Archivdateien verwenden einen UTC-Zeitstempel mit Nanosekunden sowie Version
und Revision aus dem aktuellen Projektstand:

```text
assistant_export_20260815T114604_123456789Z_v0_10_1807_r01.zip
```

Damit enthaelt jeder Archivname Datum, Uhrzeit, Version und Revision. Die
Revision wird aus der autoritativen `ZIP_KIND.txt` gelesen; es wird keine zweite
Revisionswahrheit eingefuehrt.

## Retention

Das Transferarchiv behaelt maximal 50 von EtuSchlopf selbst verwaltete
Archivdateien. Nach erfolgreicher Publikation werden die aeltesten passenden
Archivdateien entfernt.

Fremde Dateien werden nicht angefasst. Als verwaltet gelten nur Namen im
exakten Schema:

```text
assistant_export_YYYYMMDDTHHMMSS_NNNNNNNNNZ_v<version>_r<digits>.zip
```

Die Kopie wird zuerst unter einem temporaeren Namen im Zielverzeichnis
geschrieben und danach per Rename publiziert. Erst dann erfolgt die Retention.

## Diagnose

Best-effort Statusmeldungen werden in

```text
target/etuschlopf_logs/assistant_export_archive.log
```

protokolliert. Ein Transferfehler wird dort als WARNING mit
`LatestPreserved=yes` dokumentiert.

## Architekturgrenzen

- Rust-owned; keine PowerShell- oder Batch-Retention.
- Keine Musik-, Writer-, Solver-, Passorder-, Repair-, Recovery- oder
  Korpussemantik.
- Kein Ersatz des bisherigen Latest-Exports.
- Keine neue Runtime-Abhaengigkeit.
- Die gepinnte Assistant-Export-Security-Flaeche und ihre Identity-Fingerprints
  werden synchron aktualisiert.

## Aktuelle Retirement-Abnahme

Seit r61 gilt statt der historischen Publikationsabnahme:

- `assistant_export_latest.zip` bleibt im Projekt-Root.
- Es wird keine datierte Assistant-Export-Archivkopie in ein Transferverzeichnis geschrieben.
- Seit r66 sind auch die zuvor nur noch als `#[cfg(test)]` erhaltenen Archive-Symbole entfernt.
  Ihre sieben bisherigen `SecurityNegativeCheck`-Identitaeten sind append-only und
  fingerprint-gebunden im `SecurityCapabilityRetirement`-Ledger pensioniert.
- Machine Evidence wird weiterhin getrennt ueber den Neuro-Evidence-Producer publiziert.

Historischer damaliger Abnahmebefehl war:

```powershell
.\run.cmd --take --strict "Etüde Nr. 1"
```

Die anschliessenden historischen Erwartungen (zusaetzliche Archiv-ZIP, Retention) sind seit r61
explizit pensioniert und duerfen nicht mehr als aktuelle Abnahme verwendet werden.
