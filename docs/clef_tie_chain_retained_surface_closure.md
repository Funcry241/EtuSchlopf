# ClefTieChainRetainedSurfaceClosure

## v0.10.1792 r01

Der v0.10.1791-r02-Vollkorpus war formal 10/10 dunkelgruen, lieferte aber ein frisches strukturelles Gegenbeispiel zur neuen Tie-Chain-Regel: In Etuede 5, Takt 15, unteres System, blieb ein bereits upstream materialisierter G2-Clef-Surface zwischen einem laufenden Ais3-Tie-Start und dem abschliessenden Tie-Stop bestehen. Der ClefReadabilityPrimaryCourse hatte in 1791 zwar neue Transition-Candidates an den Tie-Ursprung rueckgebunden, behandelte bereits vorhandene sichtbare Clef-Surfaces jedoch weiterhin als frei retainbar.

1792 schliesst deshalb keine neue Repair-/Recovery-Schicht an. Der bestehende ClefReadabilityPrimaryCourse-Owner erweitert ausschliesslich seinen DecisionScope: Ein sichtbarer Clef-Surface, an dessen Position eine untere Tie-Chain offen ist, darf nicht als gueltige retained Transition ausgewaehlt werden. Ist der Surface im bestehenden Modell entfernbar, bleibt nur der ohnehin vorhandene Omit-Pfad; die bereits vorhandene tie-chain-gebundene Transition-Candidate-Logik kann den benoetigten Clef an der strukturell richtigen Ursprungsgrenze materialisieren. Ist ein solcher Surface nicht sicher entfernbar, plant der Owner fuer diesen Part fail-closed keine Course-Mutation.

Zusaetzlich besitzt derselbe Owner nun eine read-only Postcondition: Nach seiner Materialisierung darf kein sichtbarer unterer Clef-Surface innerhalb einer offenen Tie-Chain verbleiben. Diese Postcondition schreibt nichts und erzeugt keinen zweiten Writer; sie verhindert lediglich, dass die konkrete Fehlerklasse erneut formal gruen committed wird.

Der neue Regressionstest bildet bewusst den zweiten Fall ab, den 1791 nicht testete: Ein bereits vorhandener G2-Surface steht zwischen Tie-Start und Tie-Stop. Erwartet wird, dass derselbe Clef-Owner den Surface nicht retained, sondern den G2-Clef vor die Tie-Chain verankert.

Keine Kosten-, Schwellen-, Register-, Pitch-, Onset-, Dauer-, Staff-, Voice-, Security- oder Passorder-Aenderung. Keine Projekt-, Takt- oder Pitch-Identitaet wird produktiv verwendet.

ChangeClass: `MusicMutation`.

Erste Abnahme:

```powershell
.\run.cmd --take --strict --writer-trace=15 "Etüde Nr. 5"
```

Danach erneut Vollkorpus.
