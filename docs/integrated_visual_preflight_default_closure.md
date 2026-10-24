# Integrated Visual Preflight Default Closure

Version: v0.10.1745 r03  
Change class: SecurityOnly

## Ziel

Der Visual-Preflight ist kein Bedienmodus mehr, den der Benutzer an jeden normalen Strict-Lauf
anhaengen muss. `ValidationPlan::Strict` waehlt automatisch den bestehenden
`LauncherTestPlan::VisualPreflight`. Dadurch bleibt genau eine Validierungswahrheit im Launcher.

Legacy-Schreibweisen `--visual-preflight`, `--quick-visual` und `--fast-visual` werden weiterhin
vom Launcher konsumiert, damit alte Aufrufe nicht versehentlich an das Musikprogramm
weitergereicht werden. Bei einem normalen Projekt- oder Korpuslauf aendern sie die Planung nicht.

## Ein-Befehl-Vertrag

- Einzelprojekt: `.\run.cmd --strict "Etüde Nr. 5" --display`
- Korpus: `.\run.cmd --strict --all --display`

Beide laufen mit dem integrierten Launcher-Testgate.

## Unveraendert

- Reihenfolge Build -> integrierte Launcher-Tests -> Ziel
- Fail-Closed bei rotem Launcher-Test
- Musik-/Writer-/PDF-Semantik
- Watch-Aliase
- `--force`-Policy
- eigenstaendiger historischer Visual-Preflight-Zielpfad fuer Legacy-Kompatibilitaet

## Security-Capability-Identitaet

Die bestehenden negativen Launcher-Checks
`one_command_contract_runs_launcher_tests_only_for_strict_visual_preflight` und
`explicit_all_wins_over_visual_preflight_but_stays_multi_project` bleiben unter ihren
historischen Symbolnamen erhalten. Der neue Default-Vertrag wird durch zusaetzliche Tests
ergaenzt; kein bestehender SecurityNegativeCheck wird entfernt.
