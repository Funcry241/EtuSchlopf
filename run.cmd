@echo off
setlocal EnableExtensions EnableDelayedExpansion
cd /d "%~dp0"
set "_ETUSCHLOPF_STARTUP_DATE=!DATE!"
set "_ETUSCHLOPF_STARTUP_CLOCK=!TIME!"
rem v0.10.1336: Ein offenes Public-Artifact-Journal wird immer vor Source-Recovery,
rem Launcher-Kompilation, Cleanup und musikalischem Lauf behandelt. Der bei einem Take
rem gesicherte alte Bootstrap ist der Restore-Carrier fuer den nachfolgenden Stand.
if exist "target\etuschlopf_public_artifact_transaction_pending.txt" (
    echo [EtuSchlopf] Offene Public-Artifact-Transaktion erkannt. Stelle den vorherigen freigegebenen Vierersatz wieder her ...
    if not exist "target\etuschlopf_take_recovery\etuschlopf_take_bootstrap.exe" (
        echo [EtuSchlopf] FEHLER: Recovery-Bootstrap fuer Public-Artefakte fehlt. Normaler Lauf wird zum Schutz verweigert. 1>&2
        exit /b 1
    )
    "target\etuschlopf_take_recovery\etuschlopf_take_bootstrap.exe" --recover-pending-public-artifacts
    if errorlevel 1 exit /b !ERRORLEVEL!
)
rem v0.10.1297: Ein abgebrochener --take darf niemals als gemischter Quellstand weiterlaufen.
rem Die vor der Transaktion gesicherte Bootstrap-EXE stellt den vorherigen Vollbaum wieder her,
rem bevor irgendeine neue Kompilation oder musikalische Verarbeitung beginnt.
if not defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE if exist "target\etuschlopf_take_transaction_pending.txt" (
    echo [EtuSchlopf] Unvollstaendige Take-Transaktion erkannt. Stelle den vorherigen Quellstand wieder her ...
    if not exist "target\etuschlopf_take_recovery\etuschlopf_take_bootstrap.exe" (
        echo [EtuSchlopf] FEHLER: Recovery-Bootstrap fehlt. Normaler Lauf wird zum Schutz verweigert. 1>&2
        exit /b 1
    )
    "target\etuschlopf_take_recovery\etuschlopf_take_bootstrap.exe" --recover-pending-take
    if errorlevel 1 exit /b !ERRORLEVEL!
)
rem r1307: Session-bound startup observations only; no validation authority.
set "_ETUSCHLOPF_TIMING_INVOCATION=%RANDOM%_%RANDOM%_%RANDOM%"
if not exist "target\etuschlopf_logs" mkdir "target\etuschlopf_logs" >nul 2>&1
if not defined ETUSCHLOPF_STARTUP_TIMING_SESSION (
    set "ETUSCHLOPF_STARTUP_TIMING_SESSION=!_ETUSCHLOPF_TIMING_INVOCATION!"
    >"target\etuschlopf_logs\startup_timing.txt" echo StartupTimingSession=!ETUSCHLOPF_STARTUP_TIMING_SESSION!
)
>>"target\etuschlopf_logs\startup_timing.txt" echo Session=!ETUSCHLOPF_STARTUP_TIMING_SESSION!; StartupCmdTiming: Phase=RunCmdBeforeChild; Invocation=!_ETUSCHLOPF_TIMING_INVOCATION!; Event=begin; Date=!_ETUSCHLOPF_STARTUP_DATE!; Clock=!_ETUSCHLOPF_STARTUP_CLOCK!
rem v0.10.1105: Bootstrap-Binaries werden einmal in einem gemeinsamen Cargo-Target gebaut.
rem Pro Start wird nur die kleine fertige EXE in einen eindeutigen Laufordner kopiert.
rem Dadurch bleibt die Windows-Dateisperren-Sicherheit erhalten, ohne pro Lauf ein komplettes Cargo-Target anzulegen.
set "_ETUSCHLOPF_HAS_TAKE="
for %%A in (%*) do (
    if /I "%%~A"=="--take" set "_ETUSCHLOPF_HAS_TAKE=1"
)
rem r1367: The incoming child captures pre-launcher failures even when the parent bootstrap is older.
if defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE if defined ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR (
    call :prepare_take_child_console
    if errorlevel 1 exit /b !ERRORLEVEL!
)
set "ETUSCHLOPF_BOOTSTRAP_JOBS=%NUMBER_OF_PROCESSORS%"
if not defined ETUSCHLOPF_BOOTSTRAP_JOBS set "ETUSCHLOPF_BOOTSTRAP_JOBS=1"
set /a "ETUSCHLOPF_BOOTSTRAP_JOBS=ETUSCHLOPF_BOOTSTRAP_JOBS - 1"
if !ETUSCHLOPF_BOOTSTRAP_JOBS! LSS 1 set "ETUSCHLOPF_BOOTSTRAP_JOBS=1"

rem r1076: ShortPath-Closure fuer Windows-Kopien; derselbe dedizierte abgeleitete Cargo-Cache lebt unter target\bc.
set "ETUSCHLOPF_BOOTSTRAP_CACHE=target\bc"
rem r926: Der fruehere globale Incremental-Off-Schalter und der package-only Cargo-Clean
rem nach jedem Take erzeugten drei kalte EtuSchlopf-Kompilationen (Clippy, Produkt, Tests).
rem Cargo-Zwischenartefakte sind abgeleiteter Cache, niemals Authority. Source-Signatur,
rem Cargo-Fingerprints und der gespeicherte Executable-FNV entscheiden weiterhin fail-closed,
rem ob ein Binary wiederverwendet werden darf. Geaenderte Quellen erzwingen weiterhin einen
rem Build; Cargo darf dabei nur seine eigenen passenden inkrementellen Units wiederverwenden.
set "CARGO_INCREMENTAL=1"
rem r926: Nach transaktionaler Installation bleibt der abgeleitete Cargo-Paketcache erhalten.
rem Der incoming Vollbaum ist vorher bytegenau verifiziert; der Launcher akzeptiert ein
rem bestehendes Produktbinary nur bei passender Version + Source-Signatur + Binary-FNV.
rem Damit wird kein altes Binary legitimiert, sondern ausschliesslich kaltes Re-Codegen vermieden.
if defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE (
    if not exist "target\etuschlopf_logs" mkdir "target\etuschlopf_logs" >nul 2>&1
    set "ETUSCHLOPF_CACHE_CLEANUP_LOG=target\etuschlopf_logs\cache_cleanup.log"
    >"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo ETUSCHLOPF TAKE BUILD CACHE RETENTION
    >>"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo [EtuSchlopf] Scope=take_transaction_incremental_reuse
    >>"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo [EtuSchlopf] StrictProfilePackageClean=retired_incremental_reuse
    >>"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo [EtuSchlopf] BootstrapPackageClean=retired_incremental_reuse
    >>"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo [EtuSchlopf] ReuseAuthority=source_signature+cargo_fingerprint+binary_fnv
    >>"!ETUSCHLOPF_CACHE_CLEANUP_LOG!" echo [EtuSchlopf] CacheRetentionResult=passed
    echo [EtuSchlopf] Build-Cache nach Take bleibt als abgeleiteter Incremental-Cache erhalten.
)
rem r1392: Take-Bootstrap, Launcher (dev-Profil unter target\bc) und das Build-Script mit den
rem Kodex-Gates werden mit opt-level 1 gebaut (r1294: 2, r1390: 0). Mittelweg: Grundoptimierung ohne die
rem hohe Compiler- und Speicherlast von opt-level 2. debug-assertions und overflow-checks bleiben aktiv.
rem Die Werte gelten nur fuer diese beiden Builds und werden vor dem Start der EXE wieder entfernt.
set "CARGO_PROFILE_DEV_OPT_LEVEL=1"
set "CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL=1"
set "ETUSCHLOPF_BOOTSTRAP_TOKEN=%RANDOM%_%RANDOM%_%RANDOM%"
if not exist "target\etuschlopf_logs" mkdir "target\etuschlopf_logs" >nul 2>&1

if defined _ETUSCHLOPF_HAS_TAKE (
    set "ETUSCHLOPF_TAKE_BOOTSTRAP_RUN=target\take_bootstrap_runs\bootstrap_!ETUSCHLOPF_BOOTSTRAP_TOKEN!"
    set "ETUSCHLOPF_TAKE_BOOTSTRAP_EXE=!ETUSCHLOPF_TAKE_BOOTSTRAP_RUN!\etuschlopf_take_bootstrap.exe"
    set "_ETUSCHLOPF_LAST_BUILD="
    if exist "target\etuschlopf_logs\take_bootstrap_build_last_s.txt" set /p _ETUSCHLOPF_LAST_BUILD=<"target\etuschlopf_logs\take_bootstrap_build_last_s.txt"
    call :last_build_hint
    echo [EtuSchlopf] Rust-Take-Bootstrap wird gebaut - Bibliothek + Kodex-Gates, Cargo ohne Zwischenausgabe!_ETUSCHLOPF_LAST_BUILD_TEXT! ... Jobs !ETUSCHLOPF_BOOTSTRAP_JOBS!
    set "ETUSCHLOPF_TAKE_BOOTSTRAP_BUILD_LOG=target\etuschlopf_logs\take_bootstrap_build.log"
    rem r253: Auch die Take-Bootstrap-Kompilation durchlaeuft das verpflichtende Architektur-Gate.
    call :startup_timing TakeBootstrapCargoBuild begin
    cargo build --quiet --locked --jobs "!ETUSCHLOPF_BOOTSTRAP_JOBS!" --target-dir "!ETUSCHLOPF_BOOTSTRAP_CACHE!" --bin etuschlopf_take_bootstrap >"!ETUSCHLOPF_TAKE_BOOTSTRAP_BUILD_LOG!" 2>&1
    set "_ETUSCHLOPF_CARGO_EXIT=!ERRORLEVEL!"
    call :startup_timing TakeBootstrapCargoBuild end
    if exist "!ETUSCHLOPF_TAKE_BOOTSTRAP_BUILD_LOG!" type "!ETUSCHLOPF_TAKE_BOOTSTRAP_BUILD_LOG!"
    if not "!_ETUSCHLOPF_CARGO_EXIT!"=="0" (
        exit /b !_ETUSCHLOPF_CARGO_EXIT!
    )
    if exist "!ETUSCHLOPF_TAKE_BOOTSTRAP_RUN!" rmdir /s /q "!ETUSCHLOPF_TAKE_BOOTSTRAP_RUN!" >nul 2>&1
    mkdir "!ETUSCHLOPF_TAKE_BOOTSTRAP_RUN!" >nul 2>&1
    copy /Y "!ETUSCHLOPF_BOOTSTRAP_CACHE!\debug\etuschlopf_take_bootstrap.exe" "!ETUSCHLOPF_TAKE_BOOTSTRAP_EXE!" >nul
    if errorlevel 1 exit /b !ERRORLEVEL!
    set "CARGO_PROFILE_DEV_OPT_LEVEL="
    set "CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL="
    call :startup_timing RunCmdBeforeChild end
    "!ETUSCHLOPF_TAKE_BOOTSTRAP_EXE!" %*
    set "_ETUSCHLOPF_EXIT=!ERRORLEVEL!"
    rmdir /s /q "!ETUSCHLOPF_TAKE_BOOTSTRAP_RUN!" >nul 2>&1
    exit /b !_ETUSCHLOPF_EXIT!
)

set "ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN=target\run_launcher_bootstrap_runs\run_!ETUSCHLOPF_BOOTSTRAP_TOKEN!"
set "ETUSCHLOPF_LAUNCHER_EXE=!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN!\etuschlopf_launcher.exe"
set "_ETUSCHLOPF_LAST_BUILD="
if exist "target\etuschlopf_logs\launcher_build_last_s.txt" set /p _ETUSCHLOPF_LAST_BUILD=<"target\etuschlopf_logs\launcher_build_last_s.txt"
call :last_build_hint
echo [EtuSchlopf] Rust-Launcher wird gebaut - Bibliothek + Kodex-Gates, Cargo ohne Zwischenausgabe!_ETUSCHLOPF_LAST_BUILD_TEXT! ... Jobs !ETUSCHLOPF_BOOTSTRAP_JOBS!
set "ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG=target\etuschlopf_logs\launcher_bootstrap_build.log"
call :take_child_stage launcher_bootstrap_build
call :startup_timing LauncherBootstrapCargoBuild begin
cargo build --quiet --locked --jobs "!ETUSCHLOPF_BOOTSTRAP_JOBS!" --target-dir "!ETUSCHLOPF_BOOTSTRAP_CACHE!" --bin etuschlopf_launcher >"!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG!" 2>&1
set "_ETUSCHLOPF_CARGO_EXIT=!ERRORLEVEL!"
call :startup_timing LauncherBootstrapCargoBuild end
call :capture_take_launcher_build !_ETUSCHLOPF_CARGO_EXIT!
if exist "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG!" type "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG!"
if not "!_ETUSCHLOPF_CARGO_EXIT!"=="0" (
    exit /b !_ETUSCHLOPF_CARGO_EXIT!
)
if exist "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN!" rmdir /s /q "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN!" >nul 2>&1
mkdir "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN!" >nul 2>&1
copy /Y "!ETUSCHLOPF_BOOTSTRAP_CACHE!\debug\etuschlopf_launcher.exe" "!ETUSCHLOPF_LAUNCHER_EXE!" >nul
if errorlevel 1 exit /b !ERRORLEVEL!
set "CARGO_PROFILE_DEV_OPT_LEVEL="
set "CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL="
call :startup_timing RunCmdBeforeChild end
"!ETUSCHLOPF_LAUNCHER_EXE!" %*
set "_ETUSCHLOPF_EXIT=!ERRORLEVEL!"
rmdir /s /q "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_RUN!" >nul 2>&1
exit /b !_ETUSCHLOPF_EXIT!

:ensure_owned_cargo_cache_tag_if_missing
set "_ETUSCHLOPF_CACHE_TAG_ROOT=%~1"
if /I not "!_ETUSCHLOPF_CACHE_TAG_ROOT!"=="target" if /I not "!_ETUSCHLOPF_CACHE_TAG_ROOT!"=="target\bc" (
    echo [EtuSchlopf] FEHLER: CACHEDIR.TAG darf nur fuer die beiden fest gebundenen Cargo-Cache-Wurzeln hergestellt werden: !_ETUSCHLOPF_CACHE_TAG_ROOT! 1>&2
    exit /b 31
)
if not exist "!_ETUSCHLOPF_CACHE_TAG_ROOT!\" (
    echo [EtuSchlopf] FEHLER: Cargo-Cache-Wurzel fehlt vor CACHEDIR.TAG-Pruefung: !_ETUSCHLOPF_CACHE_TAG_ROOT! 1>&2
    exit /b 31
)
set "_ETUSCHLOPF_CACHE_TAG_FILE=!_ETUSCHLOPF_CACHE_TAG_ROOT!\CACHEDIR.TAG"
set "_ETUSCHLOPF_CACHE_TAG_SIGNATURE=Signature: 8a477f597d28d172789f06886806bc55"
if exist "!_ETUSCHLOPF_CACHE_TAG_FILE!" exit /b 0
>"!_ETUSCHLOPF_CACHE_TAG_FILE!" echo !_ETUSCHLOPF_CACHE_TAG_SIGNATURE!
if errorlevel 1 (
    echo [EtuSchlopf] FEHLER: fehlendes CACHEDIR.TAG konnte nicht fuer Cargo hergestellt werden: !_ETUSCHLOPF_CACHE_TAG_FILE! 1>&2
    exit /b 31
)
>>"!_ETUSCHLOPF_CACHE_TAG_FILE!" echo # This file is a cache directory tag created for Cargo compatibility by EtuSchlopf.
if errorlevel 1 exit /b 31
>>"!_ETUSCHLOPF_CACHE_TAG_FILE!" echo # For information about cache directory tags see https://bford.info/cachedir/
if errorlevel 1 exit /b 31
if not exist "!_ETUSCHLOPF_CACHE_TAG_FILE!" (
    echo [EtuSchlopf] FEHLER: CACHEDIR.TAG fehlt nach Herstellung: !_ETUSCHLOPF_CACHE_TAG_FILE! 1>&2
    exit /b 31
)
exit /b 0

:last_build_hint
rem K8: advisory "zuletzt" hint for the silent Cargo builds; digits only, otherwise no hint.
set "_ETUSCHLOPF_LAST_BUILD_TEXT="
if not defined _ETUSCHLOPF_LAST_BUILD exit /b 0
for /f "delims=0123456789" %%D in ("!_ETUSCHLOPF_LAST_BUILD!") do set "_ETUSCHLOPF_LAST_BUILD="
if defined _ETUSCHLOPF_LAST_BUILD set "_ETUSCHLOPF_LAST_BUILD_TEXT=, zuletzt !_ETUSCHLOPF_LAST_BUILD! s"
exit /b 0

:startup_timing
>>"target\etuschlopf_logs\startup_timing.txt" echo Session=!ETUSCHLOPF_STARTUP_TIMING_SESSION!; StartupCmdTiming: Phase=%~1; Invocation=!_ETUSCHLOPF_TIMING_INVOCATION!; Event=%~2; Date=!DATE!; Clock=!TIME!
exit /b 0

:prepare_take_child_console
set "_ETUSCHLOPF_EARLY_CHILD_CONSOLE=target\etuschlopf_logs\etuschlopf.console.txt"
>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo ETUSCHLOPF-EARLY-CHILD-LOG
if errorlevel 1 exit /b 1
>>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo TakeAttemptEvidenceDir=!ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR!
>>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo StartupInvocation=!_ETUSCHLOPF_TIMING_INVOCATION!
call :take_child_stage run_cmd_before_launcher
exit /b !ERRORLEVEL!

:take_child_stage
if not defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE exit /b 0
if not defined ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR exit /b 0
>"!ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR!\CHILD_STAGE.txt" echo schema=1
if errorlevel 1 exit /b 1
>>"!ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR!\CHILD_STAGE.txt" echo authority=none
>>"!ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR!\CHILD_STAGE.txt" echo launcher_stage=%~1
>>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo EarlyChildStage=%~1
exit /b !ERRORLEVEL!

:capture_take_launcher_build
if not defined ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE exit /b 0
if not defined ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR exit /b 0
>>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo LauncherBootstrapCargoBuild
if exist "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG!" type "!ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG!" >>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!"
>>"!_ETUSCHLOPF_EARLY_CHILD_CONSOLE!" echo LauncherBootstrapCargoExit=%~1
exit /b 0
