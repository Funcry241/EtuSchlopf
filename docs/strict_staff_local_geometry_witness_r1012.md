# r1012 StrictStaffLocalGeometryWitness

r1011 ist der akzeptierte Parent. Etuede 6 lief dort dunkelgruen mit 1329 Tests. r1012 fuehrt keine weitere globale Audit-Schicht und keine Reparatur ein. Der Take beobachtet ausschliesslich die bereits vorhandene StrictStaff-Pausenmaterialisierung an der kleinsten zusammenhaengenden Vorher-/Nachher-Grenze, an der Entscheidung und Kandidatenmaterialisierung gemeinsam sichtbar sind.

## Zielgeometrie

Der bekannte E6-Befund wird lokal als voice- und beam-freie Konfliktgeometrie gemessen:

- Measure 40;
- Staff 2;
- sichtbare Pause `[192,288)`;
- sichtbares Pitched-Atom Fis4 / MIDI 66 `[192,216)`;
- erwartete geerbte Metrik fuer den aktuellen E6-Fall: `divisions=96`, `time=4/4`.

Der Probe-Operator verwendet dieselbe bestehende Render-Phase-Parse- und VisibleRhythmicOccupancy-Closure wie r1011. Er parst den vollstaendigen XML-Kontext und waehlt den Zielbereich erst danach aus. Voice und Beam sind kein Bestandteil von `G`.

## Dreistufiges G

`G` besitzt genau drei Zustaende:

- `0`: beide Zielintervalle liegen in einer vollstaendig erfassten und metrisch vergleichbaren Measure-Basis, die konkrete Rest/Pitch-Konfliktgeometrie ist nicht vorhanden;
- `1`: dieselbe vollstaendige Basis enthaelt die konkrete Rest/Pitch-Konfliktgeometrie;
- `UNDETERMINED`: Parse, geerbte `divisions`/Taktart, Intervallgrenzen oder lokale atomare Vollstaendigkeit reichen fuer eine Negativ- oder Positivaussage nicht aus.

Eine lokal unvollstaendige Akkordgruppe darf deshalb niemals zu `0` kollabieren.

## Beobachtete Funktionsgrenze

Die Messung liegt in `build_strict_staff_local_release_completeness_with_transient_wav_gate_trial` unmittelbar um

`apply_strict_staff_local_release_completeness_materialization(xml, 32)`.

Dabei gilt:

- A = vollstaendiges eingehendes `xml`;
- B = vollstaendiges `candidate_xml` unmittelbar nach der StrictStaff-Materialisierung;
- `candidate_transition` wird nur ausgegeben, wenn A und B bestimmt sind und dieselbe Metrikbasis besitzen;
- `gap_trace` fuer `scope 40 staff 2` wird rein diagnostisch mitgefuehrt.

Die Probe beeinflusst weder `selected` noch `invariants_ok`, Writer-Freigabe, Kandidatenauswahl oder Release-Entscheidung.

## Uebernommener Write

Wenn StrictStaff tatsaechlich schreibt, wird `output_path` unmittelbar danach mit `std::fs::read_to_string` gelesen. Es gibt absichtlich kein `read_or_fallback`. Ein `adopted_transition` ist nur bestimmt, wenn

1. der direkte Readback erfolgreich ist,
2. der Readback bytegleich zu `candidate_xml` ist,
3. Vorher- und Readback-Probe bestimmt sind und dieselbe Metrikbasis besitzen.

Damit bedeutet `adopted_transition 0->1` nur: Dieser konkrete StrictStaff-Write hat in diesem gebundenen Lauf die untersuchte Geometrie zwischen seinen direkt beobachteten Grenzen erzeugt. Es ist keine Behauptung, dass dies das historisch erste oder einzige Auftreten der Geometrie in der Gesamtpipeline ist.

## Regressionen

r1012 fuegt drei lokale Tests hinzu:

1. vollstaendige geerbte Metrikbasis und synthetisches `0->1`;
2. fehlende Taktbasis liefert `UNDETERMINED`;
3. unvollstaendige Ziel-Akkordgruppe liefert `UNDETERMINED` statt `0`.

Erwarteter Strict-Testbestand: **1332**.

## Authority

`RefactorOnly`; keine Musikmutation, keine Regel- oder Gewichtungsaenderung, kein neuer Writer, keine Reparatur, kein Selektor und keine Release-Authority. Die bestehende StrictStaff-Entscheidung wird ausschliesslich beobachtet.

Naechstes Gate: nur `./run.cmd --take "Etüde Nr. 6"`; kein `--all`, kein `--inventory`.
