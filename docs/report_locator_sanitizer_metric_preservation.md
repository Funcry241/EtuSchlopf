# Report Locator Sanitizer Metric Preservation

Stand: v0.10.1624 r01

Der finale Ergebnisbericht anonymisiert konkrete Takt- und Bereichskoordinaten,
damit Fundstellen keine produktive Auswahlautoritaet erhalten. Die bisherige
Slash-Nachmaskierung war jedoch zeilenweit: Sobald eine Zeile ein Locatorwort
wie `TargetMeasure`, `Takt` oder `Scope` enthielt, wurde jede spaetere
Zahlenfolge hinter `/` ebenfalls als Locator behandelt.

Dadurch wurden unabhaengige Beweiswerte beschaedigt, beispielsweise:

- `NoteCountBeforeAfter 795/795` wurde zu `795/<locator>`,
- `AccentCountBeforeAfter 12/11` wurde zu `12/<locator>`,
- Offset-/Dauerfolgen wie `192:24/216:24/240:24` verloren ihre zweiten
  Koordinaten.

Die Closure erkennt Locatorlisten und -bereiche vor der Einzelwertmaskierung
und begrenzt die Slash-Nachmaskierung auf eine unmittelbar an einen bereits
maskierten `<locator>` anschliessende Zahlenkette. Konkrete Taktfolgen wie
`Takt 18/22/24` sowie Kommalisten wie `Target measure 4,10,11` bleiben
vollstaendig maskiert, waehrend Notenzahlen, Akzentzahlen,
Offset-/Dauerpaare und andere metrische Invarianten unveraendert lesbar
bleiben.

Der Take aendert nur die Ergebnisbericht-Sanitizerlogik und fuegt zwei
Regressionstests hinzu. Musik, MusicXML, PDF, Writer, Renderer, Solver,
Freigabe und Takt-/Etuedenauswahl bleiben unveraendert.
