# r265 – MusicMutation: abgeleitete Audit-WriterReachability

## Ausgangsbefund

r264 wurde vor Installation ausschliesslich wegen einer `modified WriterReachability` der byteunveraenderten Audit-Funktion `pipeline_orchestrator_report_compaction::real_main` abgewiesen. Die Musikmutation entfernt keinen Audit-Aufruf und fuegt keinen Audit-Writer hinzu. Sie trennt den unveraenderten alten Music-Writer-Root `octave_shift_measure_if_readability_passage` aus dem produktiven Musikpfad. Die daraus abgeleitete Writer-Erreichbarkeitsmenge des Audit-Orchestrators wird deshalb kleiner.

## Sicherheitsvertrag

Fuer `MusicMutation` ist eine abgeleitete Audit-`WriterReachability` nur autorisierbar, wenn alle Bedingungen gleichzeitig gelten:

1. Die betroffene Capability ist `WriterReachability` und nur `Modified`.
2. Der Audit-Pfad ist nicht als geaendert deklariert.
3. Aktuelle und eingehende Audit-Datei sind bytegenau identisch.
4. Die neue Writer-Root-Menge ist eine echte Teilmenge der alten.
5. Kein Writer-Root wird hinzugefuegt oder gegen einen anderen ausgetauscht.
6. Jeder weggefallene Root bleibt in beiden Baeumen als Music-Funktion vorhanden.
7. Signatur und Funktionskoerper jedes weggefallenen Roots bleiben identisch.
8. Alle anderen Audit-Capability-Deltas bleiben durch die bestehende `MusicMutation`-Policy verboten.

Damit autorisiert der Vertrag nur eine transitiv abgeleitete Verengung. Er kann keine neue Audit-Schreibbefugnis erzeugen und keine geaenderte Audit-Quelle unter einer Musikklasse verstecken.

## Tests

- byteunveraenderter Audit-Aufrufer + strenge Music-Writer-Verengung: erlaubt;
- Erweiterung der Writer-Root-Menge: abgelehnt;
- Aenderung der Audit-Quelle: abgelehnt;
- Austausch eines alten Writer-Roots gegen einen neuen: abgelehnt.

## Nicht Teil von r265

Keine MusicXML-Aenderung, keine Registernotationsentscheidung, keine Materialisierung, keine Schwellenwertaenderung, kein Writer-Umbau und kein SecurityCapabilityRetirement. r263 und r264 bleiben verworfene Kandidaten.
