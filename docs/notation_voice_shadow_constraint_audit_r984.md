# r984 NotationVoiceShadowConstraintAudit

## Zweck

r984 untersucht die sichtbare Notationsstimme ausschliesslich als Shadow. Der akzeptierte Parent ist r982. r983 ist rejected Failure Evidence: der produktive Merge von Primary/Secondary reduzierte den Vollkorpus auf 8/10 und wurde zurueckgerollt.

## Shadow-Vertrag

Der Shadow veraendert keine MusicXML-Voice und besitzt keine Writer-, Repair-, Selection- oder Release-Authority.

Er klassifiziert finale sichtbare Cross-Voice-Strukturen in drei Klassen:

- `MERGE_SINGLE_VISIBLE_VOICE`: ein zusammenhaengender metrischer Run auf demselben Staff besitzt eine exakt passende primaere Balkenkette und im Run-Intervall existiert keine weitere sichtbare notierte Polyphonie.
- `PRESERVE_SEPARATE`: sichtbare pitched Events verschiedener Voices ueberdecken sich auf demselben Staff in positiver notierter Zeit.
- `UNRESOLVED`: Cross-Voice-Metriklauf ohne ausreichende positive Same-Line-Evidenz oder trotz Balkenkette mit externer notierter Polyphonie. Unresolved behaelt fail-closed die aktuelle Voice.

## Nicht-Autoritaeten

Technical Sounding Lane ist keine Shadow-Entscheidungsautoritaet. Projektname, Etuedennummer, Taktnummer, historische Auditzaehler und alte Passreihenfolgen duerfen keine Entscheidung selektieren. Balken sind hier nur positive finale visuelle Same-Line-Evidenz; sie schreiben selbst keine Voice.

## Erwartung

Der Vollkorpus soll beantworten, ob Etuede 6 Takt 1 in der konservativen Merge-Klasse liegt und welche Konstellationen in Etuede 3 und 10 bewusst `PRESERVE_SEPARATE` oder `UNRESOLVED` bleiben. Erst danach darf ein produktiver NotationVoice-Cut diskutiert werden.

Erwarteter Strict-Testbestand: 1.277.
