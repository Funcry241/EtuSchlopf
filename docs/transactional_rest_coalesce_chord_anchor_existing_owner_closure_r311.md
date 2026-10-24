# TransactionalRestCoalesceChordAnchorExistingOwnerClosure

Stand: v0.10.1839 r311  
Parent: r309 `AuditDeepClippyRiskBoundaryClosure`  
ChangeClass: `MusicMutation`

## Ausgangslage

r309 ist der letzte real installierte und Vollkorpus-GREEN bestaetigte Stand. Der Kandidat r310 `TransactionalRestCoalesceChordAnchorSafetyClosure` wurde vor Installation vom Security-Gate abgewiesen und ist **kein Parent**.

Der fachliche Befund hinter r310 bleibt bestehen: Beim generischen transaktionalen Rest-Coalescing kann das Entfernen einer sichtbaren Pause den MusicXML-Anker eines direkt folgenden gepitchten `<chord/>`-Events entfernen. In Etuede 5, Takt 48 werden dadurch drei hidden Chord-Events B3/D4/F#4 von Onset 264 auf Onset 240 vorgezogen, obwohl Pitch, Dauer, Staff und Voice unveraendert bleiben.

## Warum r310 abgewiesen wurde

r310 fuegte drei neue `#[test]`-Funktionen direkt im produktiven Writer-Modul `render_polish_hidden_rest_readability_tail.rs` hinzu. Der semantische Call-Graph erkannte diese neuen Tests korrekt als neue Caller bestehender Writer-Wurzeln ausserhalb `StructuralWriteAuthority` und blockierte sie mit `NO-NEW-REPAIR-AUTHORITY SECURITY GATE`.

Zusaetzlich erzeugte eine neu eingefuehrte Hilfsfunktion im MultiRevision-Semantic-Capability-Vergleich eine semantiknahe Funktionskoerper-Kollision mit einer frueheren `calibration-threshold`-Capability. Der Kandidat war damit architektonisch unnoetig breit, obwohl seine Musikidee eng war.

## r311-Strategie

r311 startet wieder direkt von r309 und fuehrt **keine neue Rust-Funktion und keinen neuen Test-Symbol-Caller** ein.

Die bestehende Funktion `apply_structural_transactional_rest_coalesce_render_write()` erhaelt innerhalb ihres bereits autorisierten Owners zwei Guards:

1. Wenn ein Restfragment des Kandidaten direkter Host eines folgenden `<chord/>`-Events ist, wird die Transaktion mit `rejected_chord_anchor_dependency` verworfen.
2. Fuer jeden ansonsten zulaessigen Trial wird inline das sortierte Multiset aller gepitchten Events aus Scope, Staff, Voice, Onset, Dauer, MIDI, Hidden und Chord vor/nach verglichen. Nur bei vollstaendiger Timeline-Invarianz darf adoptiert werden.

Es gibt bewusst keine neue Helper-Funktion fuer diese Signatur.

## Regression ohne neue Writer-Caller

Die bereits vorhandene Regression `chord_anchored_sounding_events_block_initial_rest_coalescing()` ist schon in r309 writer-erreichbar und belegt die aeltere `ChordAnchoredRestSpanSafetyClosure`. r311 erweitert **diesen bestehenden Test** um zwei Subfaelle:

- der generische transaktionale Coalescer lehnt Rest-Rest-HiddenChord als `rejected_chord_anchor_dependency` bytegleich ab;
- ein echtes Rest-Rest-Paar ohne Chord-Host wird weiterhin zu Dauer 48 konsolidiert.

Damit bleibt die Zahl registrierter Tests unveraendert; erwartet werden weiterhin 1066/1066 Tests.

## Scope

- Parent ausschliesslich r309;
- r310 bleibt verworfener Kandidat;
- keine neue Writer-/Repair-/Recovery-/Shadow-Authority;
- keine neue Rust-Funktion;
- keine neue Schwelle oder Gewichtung;
- keine Etueden-/Takt-/Pitch-Sonderregel;
- Kandidatenfinder unveraendert;
- bestehender layer-balanced Restpfad unveraendert;
- Master-MusicXML unveraendert;
- Render-MusicXML/PDF duerfen sich nur dadurch aendern, dass eine bisher unsichere Restkonsolidierung nicht mehr committed wird.
