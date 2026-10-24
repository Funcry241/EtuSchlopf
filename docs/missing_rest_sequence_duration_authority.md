# MissingRestSequenceDurationAuthority

Stand: v0.10.1663 r01

## Ausgangslage

Der Missing-Rest-Kandidatengenerator ersetzt eine bereits vorhandene
`forward`-Zeit durch sichtbare Pausen. Fuer laengere Dauern verwendet er
`split_duration_for_behind_bars_rest_compaction` und erzeugt bewusst eine Folge
aus mehreren `note`-Elementen. Ein Beispiel ist die gueltige Zerlegung
`120 -> 96 + 24`.

Die zentrale `RenderEditEffect::MissingRest`-Autoritaet verlangte dagegen mit
`exact_rest_note(replacement)`, dass der komplette Ersatz genau eine einzelne
Rest-Note ist. Anschliessend verglich sie die `forward`-Dauer mit der Dauer
dieser einen Note. Eine korrekt erzeugte Mehrpausensequenz wurde dadurch
abgelehnt, obwohl der direkte Aufrufer sie bereits als sicheren Kandidaten
ausgewaehlt hatte. Der folgende fail-fast-Aufruf konnte deshalb in einem bisher
nicht vom bestaetigten Korpus ausgeloesten Fall abbrechen.

## Aenderung

Die zentrale Autoritaet akzeptiert weiterhin den bisherigen Ein-Pausen-Fall und
zusaetzlich eine gepruefte Pausensequenz. Sie bindet keine neue Auswahl und
erzeugt selbst keine Musik. Der Ersatz wird als XML-Fragment unter einer
lokalen Wurzel geparst und muss folgende Bedingungen erfuellen:

- mindestens ein direktes `note`-Element,
- keine anderen direkten Nutzknoten,
- pro Note genau ein direktes `rest`-Element,
- kein `pitch`-Element in der Note,
- pro Note genau eine positive direkte `duration`,
- ueberlaufgeschuetzte Summenbildung,
- exakte Gleichheit der Gesamtdauer mit der gebundenen `forward`-Dauer.

Kommentare, sonstige direkte Elemente, leere Sequenzen, Null- oder ungueltige
Dauern, mehrere Dauerfelder, klingende Noten und falsche Gesamtdauern werden
fail-closed abgelehnt.

## Testvertrag

Die fokussierten Modultests pruefen:

- `forward 120 -> rest 120` bleibt erlaubt,
- `forward 120 -> rest 96 + rest 24` ist erlaubt,
- `96 + 12` wird wegen falscher Gesamtdauer abgelehnt,
- eine Sequenz mit `pitch` wird abgelehnt,
- ein leeres Ersatzfragment wird abgelehnt.

## Scope

Geaendert wird nur der bestehende Missing-Rest-Vertrag in
`render_measure_articulation_authority.rs`. Kandidatenauswahl,
Pausenzerlegung, Staff-/Voice-Entscheidung, Writer-Reihenfolge,
Schwellenwerte, MusicXML-Schema, Reports, Launcher und alle anderen
Deep-Clippy-Fundstellen bleiben unveraendert.

Da die Autoritaet in bisher abgelehnten Mehrpausenfaellen produktives
Render-MusicXML zulassen kann, wird der Take korrekt als `MusicMutation`
deklariert. Im bestaetigten v0.10.1662-Korpus wurde der Fehlerpfad nicht
ausgeloest; daher werden dort identische Artefakte erwartet.
