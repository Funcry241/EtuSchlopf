# RestExistenceGapMaterializationEvidence r340

## Ausgangspunkt

Autoritative Basis ist der real vollstaendig gruene r339-Vollkorpus (10/10, FinalExit 0). Dieser Take ist AuditOnly. Er aendert keine Musik, keine Passorder, keine Schwelle, keinen Rest-Selektor und keine Materialisierung.

## Kontrollbefund Etuede 5, Takt 5

Der auffaellige untere Notenbereich wurde nicht nur als Staff-2-Bild, sondern als komplette Voice-Timeline gelesen.

- Voice 1 ist ueber beide Klavierstaffs von 0 bis 384 divisions lueckenlos gepitcht belegt.
- Voice-1-Silence im Takt: 0 divisions.
- Die sichtbare Staff-2-Projektion hat dagegen Luecken 96..240 und 264..384.
- Staff 2 / Voice 2 enthaelt vier versteckte Viertelpausen zu je 96 divisions. Diese bilden eine vollstaendige Hilfs-/Buchhaltungs-Lane, keine bewiesene Voice-1-Stille.

Damit gilt fuer diesen Kontrollfall: Staff-Gap ist nicht gleich Voice-Silence. Ein RestExistence-Cut, der die Staff-2-Luecken direkt materialisiert, waere fachlich nicht bewiesen.

## Korpus-Gegenproben

Im r339-Renderkorpus gibt es drei Takte mit derselben Kombination aus cross-staff sichtbarer Voice 1 und einer versteckten 4x96-Restlane:

- Etuede 5 / Takt 5: Voice-1-All-Staff-Coverage 384/384, keine Silence.
- Etuede 7 / Takt 5: Voice-1-All-Staff-Coverage 384/384, keine Silence.
- Etuede 8 / Takt 45: Voice-1-All-Staff-Coverage 360/384; nur 0..24 ist echte Voice-Silence.

Etuede 8 / Takt 45 ist zugleich historischer Gegenbeweis gegen ein pauschales Sichtbarmachen der Aux-Restlane: genau diese Klasse hatte bereits eine Shadow-Rest-Regression erzeugt.

## Source-Ownership-Befund

Der aktuelle r339-Quellstand besitzt bereits mehrere Rest-/Gap-Materialisierungsflaechen, unter anderem:

- VisibleRhythmLedger,
- BeatScopedRestCompletion,
- StrictStaffGapMaterialization,
- MissingStaffEdgeRest.

BeatScopedRestCompletion unterscheidet dabei bereits explizit Staff-Projektionsgap und Same-Voice-Pitched-Coverage ueber beide Staffs. Bei cross-staff Belegung wird kein Completion-Rest erzeugt. Die bestehende Aux-Rest-Sichtbarkeit kann dennoch Resttraeger verstecken.

Der spaetere FinalVisibleRestRole-Owner klassifiziert dagegen nur bereits sichtbare Restereignisse. Ein frueh versteckter Aux-Rest gelangt damit nicht erneut in diesen sichtbaren Role-Entscheidungsraum.

## Urteil Take A

Der konkrete Kontrollfall beweist **keinen RestExistence-Fehler**. RestExistence / GapMaterialization ist fuer Etuede 5 Takt 5 nicht die primaere Obstruction.

Offen ist stattdessen die Frage, wie eine lueckenlos klingende Voice bei Staff-Wechseln als Hand-/Staff-Projektion lesbar bleibt, ohne eine falsche Silence zu erfinden und ohne den historischen Shadow-Rest-Gegenfall wieder zu oeffnen.

Arbeitsname der offenen Dimension:

`CrossStaffHandProjectionReadability`

Dieser Take gibt dafuer ausdruecklich kein produktives Schreibrecht frei.

## Invarianten

Unveraendert bleiben:

- Pitch, Onset, Duration, Staff und Voice,
- RestExistence und RestVisibility,
- Beam-, Clef- und RestSpan-Owner,
- Master- und Render-MusicXML,
- PDF,
- Passorder, Thresholds und Security-Gates.
