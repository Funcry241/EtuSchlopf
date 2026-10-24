# r161 – WavPitchExternalSensorNativeExitCodeCaptureRecovery

## ChangeClass

`AuditOnly`

## Parent

Autoritativer Parent ist der akzeptierte Stand `0.10.1839 r160` (`WavPitchExternalSensorPackageProbePackagingRecovery`).

r160 wurde installiert und der normale EtuSchlopf-Lauf blieb `dark_green` / `released`. Der externe Sensorbenchmark erreichte jedoch noch keine Sensorinferenz. Die lokal bereitgestellte CPython-3.11.9-Runtime und die venv waren vorhanden; auch `pip` selbst aktualisierte sich erfolgreich. Der Audit-Wrapper wertete diesen erfolgreichen nativen Prozess anschließend fälschlich als Fehlschlag.

## Ursache

`Invoke-NativeExitCode` ließ bei nicht-leisen nativen Prozessen deren stdout/stderr direkt auf den PowerShell-Success-Stream schreiben und gab danach zusätzlich `$LASTEXITCODE` zurück. Eine Funktionszuweisung erhielt dadurch nicht nur den skalaren Exitcode, sondern eine Sammlung aus Prozessausgabe plus Exitcode. Der Vergleich mit `0` konnte daher trotz erfolgreichem nativen Prozess in den Fehlerzweig laufen.

Dies ist reine Infrastruktur-Evidence. Es wurde weiterhin kein TransKun-Sensoroutput beobachtet und daraus folgt keine fachliche Negativevidence für WAV-only Source Parity.

## Recovery-Schnitt

r161 ändert ausschließlich die native Prozess-Hilfsfunktion:

- stdout/stderr eines nicht-leisen nativen Prozesses wird lokal gesammelt;
- `$LASTEXITCODE` wird unmittelbar danach separat als Integer eingefroren;
- die gesammelte Ausgabe wird mit `Write-Host` sichtbar projiziert und kontaminiert damit nicht den Funktionsrückgabewert;
- die Funktion liefert exakt einen skalaren Integer-Exitcode;
- der leise Probe-Pfad bleibt semantisch unverändert.

Damit kann ein erfolgreicher `pip`-Lauf nicht mehr durch seine eigene Konsolenausgabe als Fehler fehlklassifiziert werden.

## Keine Authority-Änderung

r161 führt insbesondere **nicht** ein:

- keine produktive Sensor-Authority;
- keine `CanonicalSourceEvents`;
- keine Staff-/Hand-/Voice-Authority;
- keine Phrase- oder Writer-Authority;
- keinen WAV-Source-Cutover;
- keine Änderung am FLP-Produktivpfad;
- keine Änderung an `run.cmd`, Rust oder Launcher/Security;
- keine neue Sensorversion oder neue Modellwahl.

Der Take bleibt `AuditOnly` innerhalb von **WavPerception Take 2/4**. Ziel bleibt ausschließlich, die bereits definierte TransKun-2.0.1-Qualifikation erstmals bis zu echter Sensor-Evidence auszuführen.
