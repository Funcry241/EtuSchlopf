# r204 MidiLauncherSourceIdentitySecurityClosure

## Zweck

Der r203-Vollkorpus beweist den produktiven MIDI-Cutover fuer alle zehn Projekte. Vor dem Entfernen der FLP-Dateien muss die Launcher-Wiederverwendungsidentitaet ebenfalls von WAV/FLP auf WAV/MIDI wechseln.

## Security-Schnitt

- `project_run_input_signature` bindet WAV und MIDI ein.
- Eine geaenderte MIDI-Datei invalidiert damit einen alten Renderer-Reuse-Stempel.
- Root-Hygiene erkennt Projektpaare ueber WAV/MIDI.
- Assistant-Export behandelt `.mid` wie `.wav` als rohen Projekteingang und exportiert sie nicht.
- Legacy `.flp` bleibt in r204 noch geschuetzt und exportseitig ausgeschlossen; die eigentliche Runtime-Pensionierung folgt getrennt.
- Keine Musik-, Writer-, Solver- oder Repair-Autoritaet wird geaendert.
