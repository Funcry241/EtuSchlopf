# r219 PerformanceClosureInstrumentationRetirement


Historischer Abschlussbericht der Serie r213 bis r219. Die Laufzeiten unten
sind damalige Messwerte, keine aktuelle Benchmark. Die ueberholten Zwischenberichte
zu der entfernten Instrumentierung und den verworfenen Experimenten sind bereinigt;
[der erhaltene Batch-Reuse](performance_solver_exact_seed_finalization_reuse_r215.md)
bleibt dokumentiert. Aktuelle Messquellen sind die
[Phasenzeiten](phase_timing_console_audit_r1291.md) und
[Launcher-Zeitgrenzen](launcher_timing_readback_r1306.md).

## Zweck

Abschluss des Performance-Zyklus nach der Messreihe r213 bis r218.

## Gemessener Verlauf

- r214 Messbasis: etwa 03:41 Korpus-Wall-Time; ArrangementSolver rund 253.2 s.
- r215 ExactSeedFinalizationReuse: etwa 03:24 Wall-Time; ArrangementSolver rund 219.8 s; 327 von 1736 vorbereiteten Seeds mussten nicht erneut vollstaendig finalisiert werden.
- r216 projektweite Erweiterung: nur 9 zusaetzliche Reuse-Treffer; kein belastbarer Zusatzgewinn.
- r217 RendererConfirmedArtifactReuse: nur 1/10 Treffer; kein Korpusgewinn.
- r218 Rueckbau auf den r215-Performancekern: 03:25 Wall-Time / 06:33 ProjectRuntimeSum; 1736 SeedPrepare / 1409 CandidateFinish. Der r215-Gewinn ist damit erneut bestaetigt.

## Abschlussentscheidung

r219 behaelt ausschliesslich den real belegten r215-Mechanismus und pensioniert die temporaere r213/r214 Instrumentierung.

Die Messinstrumentierung war Audit-/Beobachtungsinfrastruktur und kein Teil der musikalischen Entscheidung. Ihre Entfernung veraendert daher keine Kandidaten, keine Scores, keine Passreihenfolge, keine Writer-Reachability, keine Renderer-Feasibility-Regel und keine MusicXML-Entscheidung.

## Verbleibender Performancevertrag

Innerhalb eines Candidate-Batches werden alle Seeds weiterhin erzeugt. Nur wenn zwei Seed-MusicXML-Strings vollstaendig byteidentisch sind, wird die teure Candidate-Finalisierung einmal berechnet und fuer die logisch getrennten Kandidaten wiederverwendet. Der FNV64-Wert ist lediglich ein Bucket; der volle Bytevergleich bleibt die Gleichheitsautoritaet.

Keine Wiederverwendung zwischen Projekten oder Laeufen. Keine Persistenz. Keine Suchverkuerzung.

## Schlusslinie

Nach einem gruenen r219-Vollkorpus gilt Performance fuer diese Serie als abgeschlossen. Weitere Optimierung benoetigt einen neuen expliziten Auftrag und neue aktuelle Mess-Evidenz.
