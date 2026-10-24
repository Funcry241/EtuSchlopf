# r209 WavLegacyOracleMidiSourceRetirementDeadCodeClosure

r207 ist der autoritativ gruene Ausgangsstand. r208 sollte ausschliesslich den verbliebenen FLP-Dateizugriff des audit-only WAV/FrozenLegacySourceOracle durch den bereits produktiv qualifizierten fail-closed Standard-MIDI-Decoder ersetzen. Der PRE-TAKE-DIFF-GATE war gruen; der anschliessende Strict-Clippy-Lauf wurde jedoch durch genau einen `dead_code`-Fehler gestoppt und der Take automatisch zurueckgerollt.

Der konkrete Befund lautet: `src/source_event.rs::scale_ticks` ist nach dem Oracle-Cutover nicht mehr benutzt. Die Funktion war ein gemeinsamer PPQ-Helfer der Uebergangs-/Auditphase und hat nach dem vollzogenen MIDI-Cutover keinen lebenden Leser mehr.

r209 uebernimmt den r208-Oracle-Schnitt und pensioniert zusaetzlich genau diese tote Funktion. Es gibt kein `allow(dead_code)`, keinen Keepalive und keinen Ersatzpfad. Die produktive MIDI-Decodierung besitzt ihre eigene fail-closed PPQ-Normalisierung; deren Verhalten wird durch diesen Schnitt nicht veraendert.

Der WAV/FrozenLegacySourceOracle bleibt reine Audit-Evidenz und erhaelt keine neue musikalische oder Writer-Autoritaet. `src/flp.rs`, externes Sensorwerkzeug, Launcher, Security, Solver und Writer bleiben unveraendert.
