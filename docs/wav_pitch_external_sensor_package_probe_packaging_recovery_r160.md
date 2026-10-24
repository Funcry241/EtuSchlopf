# r160 – WavPitchExternalSensorPackageProbePackagingRecovery

## ChangeClass

`AuditOnly`

## Parent

Autoritativer Parent ist der akzeptierte Stand `0.10.1839 r158` (`WavPitchExternalSensorRuntimeBootstrapRecovery`).

`r159 WavPitchExternalSensorPackageProbeControlFlowRecovery` wurde **vor Installation** vom Take-Tree-Integrity-Gate verworfen, weil das Paket irrtümlich einen unbekannten Top-Level-Pfad `Cargo.lock` enthielt. Diese Ablehnung ist ausschließlich Packaging-Evidence und enthält keine fachliche Sensor-Negativevidence.

## Recovery-Schnitt

r160 übernimmt ausschließlich den fachlichen Kontrollfluss-Fix des verworfenen r159 auf den grünen r158-Parent:

- der erwartete Zustand „TransKun/Mido noch nicht installiert“ wird als nativer Prozess-Exitcode behandelt;
- dadurch wird der bereits vorgesehene, gepinnte Installationszweig für `transkun==2.0.1` und `mido==1.3.3` erreichbar;
- nach der Installation werden die Paketversionen nochmals explizit geprüft;
- der bestehende r158-CPython-3.11.9-Bootstrap bleibt unverändert;
- Offline-Inferenz, Sensor-Freeze vor Oracle-Auswertung und alle bestehenden Authority-Grenzen bleiben unverändert.

## Packaging-Recovery

Das r160-Paket wird neu vom grünen r158-Parent aufgebaut. Es enthält **keinen Root-`Cargo.lock`**. Der legitime bestehende Lockfile unter `tools/etuschlopf_watch/Cargo.lock` bleibt unverändert.

## Keine Authority-Änderung

r160 führt insbesondere **nicht** ein:

- keine produktive Sensor-Authority;
- keine `CanonicalSourceEvents`;
- keine Staff-/Hand-/Voice-Authority;
- keine Phrase- oder Writer-Authority;
- keinen WAV-Source-Cutover;
- keinen FLP-Fallback-Umbau;
- keine Änderung an `run.cmd`, Launcher oder Rust-Produktivpfad.

Der Take bleibt innerhalb **WavPerception Take 2/4** und dient ausschließlich dazu, die bereits definierte externe Sensorqualifikation tatsächlich bis zur Messung ausführen zu können.
