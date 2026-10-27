# Barrierefreiheit

EtuSchlopf ist ein experimentelles, privat gepflegtes Projekt. Ziel ist, Hürden
bei Bedienung und Ergebniszugang sichtbar zu machen und im Rahmen der verfügbaren
Zeit zu verringern. Eine vollständige Barrierefreiheit oder Konformität mit einem
Barrierefreiheitsstandard wird derzeit nicht zugesagt.

## Vorhandene Zugänge

Der gepflegte Windows-Einstieg ist die Kommandozeile über `run.cmd` mit
Textausgaben und gespeicherten Laufberichten. Die optionale Watch-Webansicht
zeigt Zustand und Fortschritt. Ihr Quelltext enthält eine deutsche
Sprachkennzeichnung, native Links und aufklappbare Detailbereiche, textliche
Statusangaben zusätzlich zu Farben sowie eine Berücksichtigung von
`prefers-reduced-motion` für die definierten Animationen. Die Ansicht besitzt
auch eine Anpassung für schmale Browserfenster.

Diese Angaben beruhen auf dem
[aktuellen Quelltext der Watch-Webansicht](../tools/etuschlopf_watch/src/web_watch.rs).
Sie sind kein Nachweis einer vollständigen Tastatur-, Screenreader-, Kontrast-
oder Zoomprüfung. Die separat vorhandene native Watch-Ansicht wurde damit nicht
bewertet.

## Bekannte Grenzen und offene Prüfungen

- Die Webansicht verwendet JavaScript und aktualisiert Zustände dynamisch.
  Verlässliche Ansagen der Live-Änderungen durch Screenreader sind nicht belegt.
- Vollständige Tastaturnavigation, Fokuserhalt bei Aktualisierungen, Kontraste,
  starke Vergrößerung und konkrete assistive Technologien müssen noch praktisch
  geprüft werden. Einzelne native HTML-Elemente beweisen diese Eigenschaften nicht.
- Die Noten-PDFs stammen aus einem externen Renderer. Eine getaggte, mit
  Screenreadern zugängliche Notendarstellung oder eine gleichwertige nichtvisuelle
  Ausgabe ist nicht nachgewiesen. MusicXML und Textberichte liegen zusätzlich vor,
  sind aber nicht pauschal ein gleichwertiger Ersatz.
- Unterstützung anderer Betriebssysteme ist durch diese Erklärung nicht zugesagt.

## Eine Hürde melden

Melde sie über die [GitHub-Issues](https://github.com/Funcry241/EtuSchlopf/issues).
Beschreibe betroffene Oberfläche, Version, Betriebssystem, Browser und bei Bedarf
Hilfstechnologie, die konkrete Aufgabe sowie erwartetes und beobachtetes
Verhalten. Eine Diagnose oder andere persönliche Gesundheitsangaben sind nicht
nötig. Es gibt keine garantierte Bearbeitungsfrist.
