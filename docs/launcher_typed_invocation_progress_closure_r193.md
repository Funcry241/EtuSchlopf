# Launcher Typed Invocation Progress Closure r193

## Ausgangspunkt

r193 basiert auf dem autoritativ gruenen r192-Vollkorpus:

- 10/10 Projekte OK
- 0 Blockerprojekte
- 0 Warnprojekte
- 10/10 final-clean
- 10/10 Ausgaben vollstaendig
- 10/10 Renderer belegt
- normales Clippy: 101 = 3 Architektur + 98 mechanisch

Die drei Architekturmeldungen lagen ausschliesslich in `src/bin/etuschlopf_launcher.rs`.

## Ursachen

Die drei Meldungen bilden zwei gemeinsame fachliche Transportformen ab.

### Fortschrittsabschluss

`write_progress_done_segments` erhielt Rolle, Bezeichnung, Laufzeit, Loggroesse und Exitcode als lose Parameter. Diese Werte beschreiben gemeinsam genau einen abgeschlossenen Fortschrittsschritt.

r193 fuehrt dafuer den internen Typ `ProgressCompletion` ein und transportiert ihn durch `Launcher`, `ConsoleSession`, Plaintext-Projektion und Segmentausgabe.

### Protokollierter Prozessaufruf

`invoke_logged_observed_target` und `invoke_logged_program_with_env` erhielten Programm, Argumente, Logziel, Ueberschrift, Darstellungsrolle und Fortschrittsbezeichnung als lose Parameter. Diese Werte beschreiben gemeinsam einen protokollierten Kindprozess.

r193 fuehrt dafuer `LoggedProcessSpec` ein. `ProcessEnvMode` bleibt separat und unveraendert, weil er die einzige echte Verhaltensvariante darstellt.

## Aufrufer-Closure

Vor Paketierung wurden alle Verwendungen repository-weit inventarisiert:

- `invoke_logged_observed_target`: zwei produktive Aufrufe, beide umgestellt.
- `invoke_logged_program_with_env`: interner Wrapper, beobachteter Wrapper und ein Build-Control-Aufruf, alle umgestellt.
- `invoke_logged_program`: bleibt als siebenparametriger Rand bestehen und baut intern `LoggedProcessSpec`; dadurch bleibt der Visual-Preflight-Aufrufer unveraendert.
- `write_progress_done_segments`: genau ein produktiver Aufruf, auf `ProgressCompletion` umgestellt.
- `progress_done_plain`: beide Regressionstest-Aufrufe auf denselben Completion-Typ umgestellt.

Es bleibt keine alte achtparametrige Launcher-Signatur der drei Clippy-Fundstellen.

## Sicherheitsgrenze

`src/bin/etuschlopf_launcher.rs` ist eine Security-Domaene und architektur-gepinnt. Die geaenderten drei Funktionen besitzen keine inventarisierte Negative-Check-Rejection-Surface. Der Take bleibt deshalb `SecurityOnly` und synchronisiert die vorhandenen Pin-Eigentuemer vollstaendig.

Nicht geaendert werden:

- SecurityNegativeCheck-Vertraege
- Relocation-/Retirement-Vertraege
- Writer- oder Musikautoritaet
- Prozessprogramm oder Argumente
- Logpfade und Logheader
- Observation-Session-Umgebungsvariablen
- Cargo-Target-Umgebungsbereinigung
- Polling-Intervall
- Fortschritts- und Exit-Semantik
- MusicXML/PDF oder musikalische Entscheidungen

## Erwartung

Nach erfolgreichem Strict-Lauf soll das normale Architektur-Clippy-Budget von 3 auf 0 sinken. Das mechanische Budget 98 soll unveraendert bleiben.
