# Headless Watch Launch Closure

Version: 0.10.1742 r03

## Ziel

Diese Same-Version-Closure entfernt ausschliesslich das separate Windows-Konsolenfenster der bereits bestehenden Watch. Die Browser-Watch bleibt dieselbe Watch und dieselbe read-only Observation-Projektion aus r01.

Der vorher vorbereitete r02-Candidate wurde vor dem Kopieren korrekt abgelehnt, weil der gepinnte Launcher ohne veraenderte Pin-Eigentuemer deklariert war. r03 synchronisiert deshalb die bereits bestehende FNV-Bindung des Launchers in `src/identity_debt_build.rs` und `src/identity_gate_trust_anchor.rs`; der dadurch geaenderte Build-Owner-Fingerprint wird ebenfalls im Trust Anchor nachgezogen. Das ist eine Fortschreibung der vorhandenen Trust-Kette, keine Lockerung und kein neuer Owner.

## Windows-Start

Der Rust-Launcher startet `etuschlopf-watch.exe` unter Windows mit `CREATE_NO_WINDOW` statt `CREATE_NEW_CONSOLE`. Standard-Ein-/Ausgabe der Watch wird fuer diesen Launcherpfad auf `null` gelegt. Der Launcher setzt ausserdem `ETUSCHLOPF_WATCH_WEB_ONLY=1`.

Damit gilt fuer den Launcherpfad:

```text
Web-Watch startet
  -> Browseroberflaeche, kein separates Watch-CMD

Web-Watch kann nicht starten
  -> Watch-Prozess beendet sich
  -> Launcher erkennt den fruehen Exit best effort
  -> musikalischer Lauf laeuft unveraendert weiter
```

Der Browserstarter selbst verwendet bereits seit r01 ebenfalls `CREATE_NO_WINDOW`, damit `cmd /C start` keinen Konsolenblitz als Ersatz fuer das entfernte Watch-Fenster erzeugt.

## Nicht geaendert

- keine Watch-/Show-/Display-Aliassemantik
- keine Observation-/MQTT-/UDP-Wahrheit
- keine PDF-Route oder PDF-Erzeugung
- keine Musik-, Writer-, Render-, Recovery- oder Manifest-Mutation
- keine Change-Control-Lockerung
- nicht-Windows-Launcherpfade bleiben unveraendert

## Abnahme

```powershell
.\run.cmd --take --strict --all --display --visual-preflight
```

Erwartung unter Windows: Browser-Watch sichtbar, kein separates Watch-CMD-Fenster.
