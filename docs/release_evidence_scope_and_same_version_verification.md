# Take, Strict, Ein-Befehl-Regel und Release-Evidenz

## 1. Geltung

Dieses Dokument ist die einzige vollständige Quelle für Take-Ablauf, Same-Version-Verhalten, Ein-Befehl-Regel und Evidenzumfang. README und Übergaben fassen nur zusammen.

## 2. Ein-Befehl-Regel

Der Benutzer beschreibt Installation und gewünschten Lauf in einem Aufruf:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Der Bootstrap entscheidet zuerst über den Take. Danach entfernt er intern `--take` und `--force` und reicht alle verbleibenden Argumente automatisch an `run.cmd` weiter.

Kein manueller Folgebefehl ist erforderlich.

## 3. Entscheidungstabelle

| Lage | Quelländerung | Fortsetzung | Exit |
|---|---:|---:|---|
| Neue Version oder zulässig geänderter Payload | transaktional | automatisch ohne `--take`/`--force` | Exit des Gesamtvorgangs |
| Identische Version und identischer Payload mit weiteren Argumenten | keine | automatisch ohne `--take`/`--force` | Exit des Folgelaufs |
| Identische Version und identischer Payload ohne weitere Argumente | keine | keine | 2 = reiner Schutz-Noop |
| Same-Version mit `--force` | absichtliche Neuinstallation nach Policy | automatisch, ohne Weitergabe von `--force` | Exit des Gesamtvorgangs |

## 4. Identischer Take mit Folgelauf

Für

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

muss bei bereits identischem Projektstand gelten:

```text
Payloadidentität bestätigen
keine Datei kopieren
keine Take-Transaktion eröffnen
--take und --force nicht weiterreichen
run.cmd automatisch mit --strict "Etüde Nr. 4" starten
Exitcode des Folgelaufs zurückgeben
```

Der Schutz verhindert nur die unnötige Kopie, nicht den ausdrücklich verlangten Lauf.

Die Fortsetzung besitzt kein take-spezifisches Buildrecht. Der Launcher erzeugt fuer jeden Einstiegspfad denselben `ValidationPlan` und `BuildPlan`. Ein Artefakt darf nur wiederverwendet werden, wenn Profil, aktuelle Quellsignatur und Binary-Fingerprint zu den erfolgreichen profilgebundenen Stempeln passen. Andernfalls wird gebaut.

Die bestehenden Negativnachweise bleiben verbindlich: Der abgeleitete Folgelauf entfernt `--take`, zitiert Projektnamen mit Leerzeichen korrekt und empfiehlt oder uebergibt niemals `--force`. Direkter Lauf, Ein-Befehl-Fortsetzung, `--all`, Visual Preflight und `--clippy-deep` duerfen keine voneinander abweichenden Buildrechte besitzen.

## 5. Reiner identischer Take

Für

```powershell
.\run.cmd --take
```

sind keine Folgelaufargumente vorhanden. Dann gilt:

```text
keine Kopie
kein Launcher
Exit 2
```

Dieser Exit 2 ist kein fehlgeschlagener Strict-Lauf und belegt weder Clippy noch Build oder Final.

## 6. Sicherheitsgrenzen

Unverändert bleiben Paketwahl, Mehrdeutigkeitsprüfung, Versions- und Payloadvergleich, Change-Class- und Pre-Copy-Gates, Trust Anchors, Public-Artifact-Recovery sowie transaktionales Backup, Apply, Verifikation, Commit und Rollback.

Die identische Fortsetzung veraendert keine Quelle und benoetigt daher keine Take-Transaktion. Ein Fehler im Folgelauf wird unveraendert als dessen Exitcode zurueckgegeben.

Buildfrische wird nicht aus einer globalen Versionssperre abgeleitet. Ein geaenderter Quellstand invalidiert die profilgebundene Signatur und erzwingt einen Build; ein unveraenderter Quellstand darf ein nachweislich passendes Artefakt wiederverwenden.

`--strict` und `--clippy-deep` sind gleichberechtigte Pruefplaene, aber nicht identisch: Strict ist das blockierende Projekt-/Release-Gate; Deep Clippy ist ein diagnostischer All-Targets-Lauf mit nicht blockierenden Zusatzwarnungen und ohne Musikpipeline. Beide verwenden denselben BuildPlan. Ein ausdruecklicher Deep-Clippy-Aufruf fuehrt seine Diagnose immer real aus; nur der davon getrennte und nach der Diagnose erneut verifizierte Build darf bei passendem Beleg wiederverwendet werden.

Architekturpruefungen unterscheiden produktive Kontrollpfade von ausfuehrbaren
Retirement-Tests. Ein Test darf den Namen eines pensionierten Pfads tragen, um
dessen Rueckkehr zu verhindern. Verboten bleiben die konkreten produktiven
Bestandteile und Aufrufe; ein blosses Namensvorkommen im Test ist kein
produktiver Kontrollpfad.

`--force` ist nur fuer eine bewusst begruendete Neuinstallation oder einen absichtlich erzwungenen Neuaufbau vorgesehen. Es ist kein normaler Laufweg.

## 7. Evidenz

Drei Nachweise bleiben getrennt:

1. **Installation:** Ein neuer Quellstand wurde übernommen.
2. **Produktivlauf:** Launcher, Gates und Projektlauf wurden tatsächlich ausgeführt.
3. **Artefaktvergleich:** MusicXML, PDF und Berichte wurden gegen den Vorgänger geprüft.

Ein identischer Same-Version-Aufruf mit automatisch fortgesetztem Projektlauf liefert keinen neuen Installationsnachweis, aber einen Produktivnachweis. Wiederverwendete Clippy-/Build-Ergebnisse muessen dabei ausdruecklich als profil-, quell- und artefaktgebundene Wiederverwendung ausgewiesen werden; sie duerfen nicht als neu ausgefuehrte Kompilation dargestellt werden. Ein reiner Exit-2-Noop liefert nur den Schutz-Nachweis.

| Beleg | Installation | Produktivlauf | Schutz |
|---|---:|---:|---:|
| Versionswechsel mit grünem Folgelauf | ja | ja | nein |
| Identischer Take mit grünem automatischem Folgelauf | nein | ja | ja |
| Reiner identischer `--take`, Exit 2 | nein | nein | ja |
| Direkter grüner `--strict`-Lauf | vorausgesetzt | ja | nein |
| Grüner `--clippy-deep`-Lauf | vorausgesetzt | Diagnose, kein Musiklauf | nein |

Artefaktgleichheit gilt nur, wenn sie tatsächlich verglichen wurde.

## 8. Artefaktrollen

- **Source-Take:** `ZipKind: source_take`, `TakeCompatible: true`; installierbarer Kandidat.
- **Ergebnisexport:** `TakeCompatible: false`; dokumentiert einen Lauf und darf nie mit `--take` verwendet werden.
- **Übergabe:** fasst Stand, Evidenz und Risiken zusammen; ersetzt weder Take noch Ergebnisexport.

## 9. Abnahme

Eine Revision wird erst verbindlich, wenn Source-Take, geeigneter Folgelauf, erforderliche Gates und zugehöriger Ergebnisexport real bestätigt sind. Relevante Artefaktvergleiche müssen ausgeführt oder konkret entbehrlich begründet sein.

Standardbefehl:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Direkter Lauf einer installierten Version:

```powershell
.\run.cmd --strict "Etüde Nr. 4"
```

Gleichberechtigter diagnostischer Pruefplan:

```powershell
.\run.cmd --clippy-deep
```

## 10. Wartungsregel

Nicht zulaessig sind ein zweiter manueller Befehl trotz vorhandener Folgelaufargumente, ein versionsbasierter Buildabbruch trotz pruefbarem Buildinput, ein take-spezifisches Wiederverwendungsrecht, eine ungebundene Artefaktwiederverwendung, `--force` als Normalweg, ein Ergebnisexport als Take oder ein Strict-Nachweis aus einem reinen Exit-2-Noop.

Änderungen an dieser Regel werden ausschließlich hier vollständig gepflegt. Eine eigene Revision ist nur bei realer Bedien-, Sicherheits-, Evidenz- oder Wartungsverbesserung gerechtfertigt.
