# r511 – Registernotation: klingende Zwischeninsel als Re-Evaluation statt erzwungener Sichtstopp

## Ausgangspunkt

r510 `OctaveSpanSoundingRegisterIslandEvidenceAudit` ist installiert und dunkelgruen. Der Audit blieb rein lesend und beobachtete in Etuede 4 zwischen bereits materialisierten Oktavspannen 13 klingende Zwischeninseln. Alle 13 lagen zwischen gleichgerichteten Oktavzuständen; 11 lagen sogar zwischen gleicher Oktavgroesse.

Der konkrete Sichtbefund in den oberen Takten 22–24 wird dadurch als Planungsproblem erklaert:

- am Ende von Takt 22 bleibt ein einzelner hoher Angriff zwischen einer 8va- und einer 15ma-Spanne loco;
- zwischen dem Ende der 15ma-Spanne in Takt 23 und ihrem erneuten Beginn in Takt 24 liegt eine laengere, vollstaendig klingende loco-Insel.

Die bereits vorhandene Lesbarkeitsfunktion unterscheidet diese Faelle klar von normalen loco-Luecken. Fuer den Einzelangriff MIDI 95 ergibt sie im oberen System `loco 132`, `8va 0`, `15ma 24`. Fuer die klingende Insel zwischen Takt 23 und Takt 24 ergibt die unveraenderte Formel `loco 1962`, `15ma 68`.

## Architekturentscheidung

r511 aendert weder die Grenzbeobachtung noch die bestehende harte Standalone-Elegibilitaet `MINDEST_TONOBJEKTE = 4`.

Eine tragende Grenze bleibt weiterhin fachliche Evidenz und damit eine Re-Evaluation-Schnittstelle. Sie erzwingt aber nicht mehr automatisch eine sichtbare loco-Insel, wenn zwischen zwei bereits begruendeten Oktavplaenen eine vollstaendig klingende Passage liegt.

Die neue Continuity-Closure arbeitet ausschliesslich innerhalb des bestehenden `RegisterNotationPlan`-Owners:

1. Zwei bereits vorhandene benachbarte Registerplaene muessen dieselbe Darstellungsrichtung besitzen.
2. Zwischen ihrem Ende und Beginn muss die gedruckte Tonspur ohne zeitliche Stille durchgehend klingend sein.
3. Die klingende Insel wird mit der bereits vorhandenen `entscheide_oktavdarstellung`-Entscheidung unveraendert neu bewertet; die benachbarten Oktavplaene liefern nur die sonst wegen `MINDEST_TONOBJEKTE` fehlende Kontinuitaets-Elegibilitaet.
4. Akzeptiert wird ausschliesslich eine Darstellung, die links oder rechts bereits fachlich begruendet ist. Eine dritte Oktavdarstellung darf die Closure nicht neu einfuehren.
5. `lesbarkeitswert`, Doppeloktav-Bezeichnungsstrafe, Unterstaff-Regel, Kandidatenreihenfolge, Tie-Break und `VERBESSERUNGSSCHWELLE` bleiben dadurch bytegleich in der vorhandenen Entscheidungscapability.
6. Nur wenn diese unveraenderte Entscheidung eine der beiden Nachbardarstellungen liefert, entsteht genau ein zusaetzlicher semantischer Kontinuitaetsplan fuer die klingende Insel.
7. Reine Stille wird niemals durch diese klingende Closure ueberbrueckt.

Damit wird keine neue 8va/15ma-Formel eingefuehrt, die bestehende Calibration-Capability nicht verschoben oder umformuliert und keine historische `LowerStaffRegisterIslandPreviewAudit`-Parallelarchitektur reaktiviert.

## Geltung und Sicht

Die zusaetzlichen Teilplaene tragen den Grund `KlingendeZwischeninselKontinuitaet`. Sie sind Teil der semantischen Planwahrheit und besitzen selbst weiterhin keinerlei MusicXML-Schreibrecht.

Der vorhandene Sichtzustandspfad bleibt unveraendert:

```text
RegisterNotationPlan
        ↓
Sichtmaterialisierungsauftrag
        ↓
bestehende Kontaktfusion / bestehende stille Bruecken
        ↓
StructuralWriteAuthority
```

Wenn ein Kontinuitaetsplan dieselbe Darstellung wie sein Nachbar besitzt und zeitlich direkt anliegt, fusioniert die bereits bestehende Kontaktfusion ihn wie jeden anderen semantischen Plan. Die Sichtschicht trifft dadurch keine neue Registerentscheidung.

## Unveraenderte Grenzen

r511 aendert nicht:

- Pitch, Onset, Dauer, Voice oder Staff;
- `MINDEST_TONOBJEKTE`;
- `VERBESSERUNGSSCHWELLE`;
- die Doppeloktav-Bezeichnungsstrafe;
- die Lower-Staff-Clef-Schutzregel;
- die Grenzklassifikation Schwach/Tragend;
- den MusicXML-Writer oder dessen produktiven Aufrufgraphen;
- den r510-Audit;
- Layout-, Beam-, Rest- oder Clef-Owner;
- Kodex, Bootstrap oder Take-Gates.

Insbesondere gibt es keine Etueden-, Takt-, Staff-, Voice- oder Pitch-Sonderregel.

## Falsifikation

Der Einzelprojektlauf Etuede 4 muss zuerst zeigen:

- PRE-TAKE-Gate, Build und Strict Clippy gruen;
- Registerplan topologisch weiterhin geschlossen;
- keine widerspruechlichen Planueberlappungen;
- die neuen Kontinuitaetsplaene werden im Bericht explizit gezaehlt;
- Takte 22–24 besitzen die erwartete ruhigere Oktavlinienfuehrung;
- keine neue sichtbare Fehloktavierung an anderen Stellen des Projekts.

Erst nach positiver Sichtpruefung darf der Vollkorpus die Breite der Regel beweisen.
