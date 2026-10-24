# FinalReportLocatorSanitizerOrderingClosure

Stand: v0.10.1626 r01

Der reale v0.10.1625-Bericht zeigte, dass die neuen Locatorregeln selbst korrekt
arbeiten, aber mehrere spaet erneut projizierte Sektionen den Sanitizer umgingen.
Dadurch blieben `target M22`, `Ziel M22`, `CompleteBeatMeasures 4,9,...` und
`Measures 4,9,...` sichtbar, waehrend frueher projizierte Felder bereits korrekt
anonymisiert waren.

Dieser RefactorOnly-Take verschiebt die beiden bestehenden finalen
Locator-Sanitizer hinter saemtliche Reportprojektionen und unmittelbar vor
`finalize()`. Die Erkennungsregeln selbst werden nicht erweitert. Damit gilt ein
einziger, finaler Berichtvertrag fuer alte und spaet wiederangehaengte Sektionen.

Ein struktureller Regressionstest bindet:

- genau einen Aufruf jedes finalen Sanitizers,
- beide Aufrufe nach der optionalen Writer-Trace-Projektion,
- beide Aufrufe vor der finalen Berichtserzeugung.

Unveraendert bleiben Musik, MusicXML, PDF, Writer, Renderer, Solver, Kandidaten,
Gewichte, Freigabeentscheidungen und produktive Takt-/Etuedenauswahl.
