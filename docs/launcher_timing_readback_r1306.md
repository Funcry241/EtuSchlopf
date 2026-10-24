# r1306 LauncherTimingReadback (KodexOnly)

Parent: accepted r1305, Version unverändert 0.10.1839.

Problem: TimingBaseline.TotalMs beginnt bei RunState-Initialisierung, obwohl der
Name die gesamte Startkette suggeriert. Parallel-Testvorbau ist kein additiver
Clippy-/Build-Aufwand. Polling liefert einen beobachteten Exit, keinen exakten
OS-Exitzeitpunkt. Receipt-Prüfungen benötigen Zeit, obwohl ChildMs bei Reuse 0 sind.

Neue Beobachtung: LauncherPrelude, StartupCleanup, BuildPlanAndArtifactVerification,
BuildControl, ClippyControlIncludingReceipt, CargoBuildControl, SourceBuildSignature,
AllTargetsSourceSignature, TestPrewarmResidualWait, IntegratedTestsControl und
PostProgramFinalizationBeforeExport besitzen monotone Start-/End-Offsets und WallUs.
Der Handoff projiziert diese vollständigen Messzeilen und subtrahiert die Union
vollständig enthaltener Messspannen für ExclusiveObservedUs; verschachtelte oder
überlappende Kinder werden nicht doppelt abgezogen. Diese Restzeit ist keine CPU-
oder rein eigene Funktionszeit, sondern enthält auch uninstrumentierte Arbeit.
Gleich begrenzte und kreuzende Intervalle haben keine erfundene Elternbeziehung.

Child-Prozesse behalten exakt dieselben Argumente, Umgebungen und Polls. Die
Zeitprobe erfolgt jetzt nach try_wait, vor Completion-UI. Spawnzeit, Completion-UI,
letzter lebender Pollstart und erste Exit-Rückkehr werden getrennt angegeben.
Die OS-Laufzeit ab spawn-call-start ist durch Unter-/Obergrenze beobachtet, mit
sichtbarer UncertaintyUs; Scheduler-/Pollzeit wird nicht als exakte Rechenzeit
behauptet. Prewarm-Lifetime am Join ist ausdrücklich nur eine Obergrenze;
ResidualWaitUs ist die zusätzliche serielle Wartezeit nach BuildControl.

Messdomäne: run()-Entry des aktuellen Launchers, vor Assistant-Handoff-Capture.
run.cmd-Kompilation, Take-Apply/Reexec und der spätere Handoff-/Outer-ZIP-/Exit-Tail
sind ausgeschlossen und werden nie als gemessene 0 ausgegeben. Nach Hashbindung
wird der exportierte Konsolenlog weiterhin nicht verändert. Timing-I/O ist best
effort, erzeugt kein Gate/Receipt/Release-Ergebnis und verändert keine Auswahl.

Der Handoff liefert zusätzlich die versionierte advisory Opus-Stimmenkollisions-
Übergabe. Sie ist historische r1305-Evidenz, kein neuer Implementierungsvertrag,
keine neue Musik-Freigabe und keine aktuelle Runtime-Wahrheit späterer Takes.

Launcher-Testbinaries schreiben keine Timing-Zeilen in den Konsolenlog ihres
aufrufenden Launchers; ihre Fixture-Uhren würden sonst die aktuelle Messdomäne
verunreinigen. Pure Format-/Intervalltests bleiben aktiv.

Tests prüfen Intervallunion statt Summieren, Parallelvorbau statt additiver
Prozesslaufzeit, fehlende/ungültige Messungen und Exit-Unschärfe. Bestehende
Launcher-Export-/Budget-/Validierungs-Tests bleiben verbindlich.
