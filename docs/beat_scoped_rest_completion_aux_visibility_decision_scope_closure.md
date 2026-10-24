# BeatScopedRestCompletionAuxVisibilityDecisionScopeClosure

## Version

`v0.10.1795 r01`

## Frische Evidenz

Der v0.10.1794-r01-Vollkorpus ist nicht releasefaehig, obwohl alle zehn Projektberichte `OK` melden: Etuede 8 wird vor MuseScore fail-closed gestoppt, weil `CommittedShadowRestCandidates = 1` im final erneut gelesenen Render-MusicXML steht. Die konkrete Domain ist Takt 45, Staff 2, Voice 2, Offset 0: eine sichtbare Viertelpause ueberlappt klingendes Voice-1-Material desselben Staffs. Der committed ShadowRest-Detector wird nicht gelockert.

Der `--writer-trace=45`-Lauf lokalisiert die Reihenfolge genauer. `visible_rest_role` ([011]) hinterlaesst Takt 45 ohne diese Voice-2-Viertelpausen. Der spaete `final_notation`-Checkpoint ([021]) materialisiert vier Voice-2-Viertelpausen aus dem Masterzustand erneut; nur die erste ist sichtbar. Der Mastervergleich v0.10.1792 -> v0.10.1794 zeigt dieselbe Regression bereits vor der spaeten Rendergrenze: in 1792 ist die erste Voice-2-Viertelpause `print-object="no"`, in 1794 sichtbar.

## Ursache

Die 1794-Closure korrigiert zu Recht die falsche staff-lokale Stilleannahme der BeatScopedRestCompletion: Wenn dieselbe Voice innerhalb eines lokalen Gaps auf einem anderen Staff (oder hidden/chord) klingendes Material traegt, wird kein cursor-advancing Rest eingefuegt.

Der bestehende Owner koppelte jedoch eine zweite Entscheidung an `inserted_rest_staffs`: vorhandene rest-only Aux-/Shadow-Pausen wurden nur dann verborgen, wenn auf demselben Staff zuvor tatsaechlich ein neuer Completion-Rest eingefuegt worden war. Seit 1794 kann ein Staff wegen der korrekten Cross-Staff-Suppression aus `inserted_rest_staffs` verschwinden. Dadurch verliert die bereits vorhandene Aux-Rest-Sichtbarkeitsentscheidung ihren DecisionScope, obwohl der Staff weiterhin Gegenstand einer Rest-Completion-Entscheidung war.

Das ist keine neue ShadowRest-Domain und kein Grund fuer eine Recovery. Es ist eine DecisionScope-Kopplung innerhalb des bestehenden BeatScopedRestCompletion-Owners.

## Closure

`plan_visible_voice_rest_completion_gaps()` liefert neben den tatsaechlichen Restplaenen nun read-only die Staffs, auf denen eine lokale Completion ausschliesslich wegen klingenden Materials derselben Voice cross-staff unterdrueckt wurde.

Die vorhandene `rest_only_aux_rests_hidden`-Entscheidung verwendet fuer ihren bisherigen Staff-Kontext danach:

```text
inserted_rest_staffs
UNION
cross_staff_suppressed_completion_staffs
```

Wichtig: Ein suppressierter Staff erzeugt weiterhin **keinen** Rest. Er bleibt lediglich innerhalb des bereits vorhandenen Aux-Rest-Sichtbarkeits-DecisionScopes. Dadurch bleibt die 1794-Regel fuer Takt 19 unveraendert, waehrend das versehentliche Sichtbarwerden einer Hilfspause nicht mehr von der Existenz eines neu eingefuegten Rests abhaengt.

## Invarianten

- keine neue Repair-/Recovery-Authority,
- kein neuer Writer und kein neuer Checkpoint,
- keine Aenderung an `FinalVisibleRestRole` oder am committed ShadowRest-Detector,
- kein Pitch-/Onset-/Duration-/Staff-/Voice-/Beam-/Clef-Target-Write,
- Cross-Staff-Gap bleibt nicht materialisierbar,
- PitchedTimeline bleibt unveraendert,
- zuvor gruener RawCursor bleibt gruen,
- Aux-Rest-Visibility wird nur auf Staffs erweitert, deren Completion wegen derselben cross-staff klingenden Voice suppressiert wurde.

Ein synthetischer Regressionstest bildet genau diese Kopplung ab: alternierende Voice-1-Pitches auf Staff 1/2 blockieren beide staff-lokalen Completion-Gaps; eine sichtbare rest-only Voice-2-Viertelpause auf dem betroffenen Staff wird dennoch wie bisher hidden materialisiert, ohne einen Completion-Rest einzufuegen und ohne Pitch-/Cursor-Aenderung.

## Erste Abnahme

```powershell
.\run.cmd --take --strict --writer-trace=45 "Etüde Nr. 8"
```

Erwartung: `CommittedShadowRestCandidates = 0`, MuseScore wird wieder zugelassen und Etuede 8.pdf entsteht. Danach muss Etuede 3/Takt 19 weiterhin die 1794-Verbesserung behalten; erst dann Vollkorpus.
