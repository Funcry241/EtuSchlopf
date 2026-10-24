# r939 TransportPathBudgetClosure

## Ziel

Der Transportpfad wird Windows-kopiersicher, ohne historische Dokumente massenhaft umzubenennen oder Musik-/Solver-Semantik anzufassen.

## Schnitt

- Eingebettete Source-Snapshots verwenden nur noch `EtuSchlopf_v<version>_<revision>_unbuilt_source.zip`; der semantische Take-Name bleibt in `SOURCE_SNAPSHOT.txt`/`ZIP_KIND.txt` erhalten.
- Der Source-Snapshot prueft vor dem Schreiben fail-closed ein UTF-16-Pfadbudget: Archivname <= 64, einzelner Dateiname <= 120, relativer Repository-Pfad <= 140, automatisches Extract-Envelope `archive_stem/relative_path` <= 180.
- Der aktuelle historisch laengste Repository-Pfad bleibt zulaessig; neue Pfadexplosionen werden beim Assistant-Export sichtbar blockiert.
- Keine Musik-, Regel-, Writer-, Repair-, Renderer-, Release- oder Public-Artifact-Semantik wird geaendert.

## Motivation

Der bisherige Source-Snapshot uebernahm den vollstaendigen langen Take-Namen in seinen Archivnamen. Beim Entpacken/Kopieren addierte sich dieser Ordnername zu bereits langen historischen `docs/`-Pfaden und konnte dadurch Windows-Pfadgrenzen erreichen. Semantik bleibt deshalb in Metadaten; physische Transportnamen bleiben kurz.
