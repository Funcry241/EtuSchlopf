# SourceDuration Release-Boundary Essence Closure · r750

## Status und Scope

r750 ist ein reiner `AuditOnly`-Essenz-Take direkt auf dem weiterhin einzigen autoritativen Parent r745. Die Kandidaten r746, r747, r748 und r749 wurden abgelehnt beziehungsweise nach sicherem Rollback verworfen und besitzen keine Parent- oder Produkt-Authority.

Dieser Take aendert keine Musik, keinen Writer, keine SourceDuration-, Segmentation-, Tie-, Voice-, Renderer-, Release- oder Recovery-Semantik. Er bewahrt ausschliesslich die aus den Fehlversuchen generalisierbare Architekturkompetenz in der repository-residenten Essenz.

## Generalisierte Befundkette

Die vier verworfenen Kandidaten zeigen vier verschiedene Grenzen desselben Arbeitsblocks:

1. Ein Regressionstest darf keinen neuen produktiven Writer-Caller ausserhalb der StructuralWriteAuthority erzeugen. Decision-Tests muessen die pure Decision-Grenze pruefen.
2. Eine scheinbar harmlose Hilfsfunktion kann durch den Semantic-Capability-Diff als neue musikalische Threshold-/Calibration-Capability erkannt werden. Vor dem Packaging muss deshalb der finale Source-Diff durch dieselbe Capability-Klassifikation gedacht werden.
3. Das Take-Change-Manifest darf nicht aus dem Parent uebernommen werden. Seine `change=`-Zeilen muessen aus den finalen Kandidatenbytes gegen den autoritativen Parent erzeugt und rueckgeprueft werden.
4. `Programm/PDF OK` ist nicht Release. Der Public-Artifact-Commit verlangt einen formal gruenen Ergebnisbericht: 0 Blocker und 0 Warnungen. Ein musikalisch sinnvoller Zwischenzustand, der eine gekoppelte Restinvariante erst sichtbar macht, ist deshalb kein releasbarer MusicMutation-Zwischenstand.

## Neue Vorabfrage fuer gekoppelte Authority-Cuts

Vor einem gestuften mutierenden Cut muss beantwortet werden:

> Kann jeder einzelne geplante MusicMutation-Zwischenstand bereits mit den final persistierten Artefakten und einem formal gruenen Ergebnisbericht releasen?

Wenn nein, gilt unter dem aktuellen Release-Vertrag:

```text
read-only / audit evidence
        ↓
gekoppelte Decision bestimmen
        ↓
ein: Zwischenzustand nicht releasbar
        ↓
atomarer MusicMutation-Cut ueber alle fuer Formal-Green noetigen Invarianten
        ↓
persistierte Artefakte
        ↓
0 Blocker / 0 Warnungen
        ↓
Public-Artifact-Commit
```

Damit wird nicht "mehr auf einmal repariert", sondern exakt die kleinste Release-atomare Invariantengruppe geschnitten. Unabhaengige Fehlerklassen bleiben getrennt.

## Konsequenz fuer den aktuellen SourceDuration-/Tie-Block

Der bisherige Plan `SourceDuration mutieren -> spaeter Tie mutieren` darf nicht allein aus Architekturbequemlichkeit beibehalten werden. r749 beweist, dass der SourceDuration-Cut zwar den gesamten technischen Lauf bis `Programm/PDF OK` ueberstehen kann, aber der daraus erzeugte Bericht den formalen Public-Artifact-Green-Vertrag nicht erfuellt. Ohne den konkreten neuen Bericht ist noch nicht bewiesen, welche einzelne Warn-/Blockerklasse den Commit verhindert hat. Deshalb ist auch nicht bewiesen, dass Tie bereits die einzige Restursache ist.

Der naechste MusicMutation-Kandidat darf erst gebaut werden, nachdem aus den verfuegbaren Report-/Validator-Ownern die kleinste gekoppelte Invariantengruppe bestimmt wurde. Wenn SourceDuration-Korrektur notwendigerweise eine Tie-/Segmentation-Restklasse freilegt, muss diese Gruppe release-atomar geschlossen werden. Voice bleibt weiterhin ausserhalb dieses Cuts, solange kein Gegenbeweis seine gleichzeitige Uebernahme erzwingt.

## Authority-Grenze

Diese Dokumentation und die Essenz sind advisory only. Die aktuelle Wahrheit bleibt bei `formal_green_contract`, Public-Artifact-Recovery, Take-Change-Control, Semantic-Capability-Diff, StructuralWriteAuthority sowie den produktiven Ergebnisgates. Keine neue Berechtigung entsteht durch diese Closure.
