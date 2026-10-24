# Transaktionsbackup-Korrektur vor dem #56-Child

r1446 wurde beim installierten Reexec abgelehnt und auf r1445 zurueckgerollt.
Ursache: verified_first_transition las ZIP_KIND.txt nur direkt im Parent,
waehrend die kontrollierte Transaktion Stammdateien unter root_files/ sichert.

r1447 ist ausschliesslich Kodex-Reparatur vor dem fachlichen Child r1448.
Nur wenn die direkte ZIP_KIND.txt mit NotFound fehlt, liest der exakte
Parent-Byteabgleich root_files/ZIP_KIND.txt. Andere Fehler, direkte falsche
Bytes und alle Quellpfade behalten ihre bisherigen Ablehnungen.
Alle erwarteten Inhalts-Hashes bleiben erhalten; nur die Paketidentitaeten
werden auf Parent r1447 und Child r1448 gebunden. Der Validator wird vom
Parent autorisiert und bytegleich an den Child vererbt.

Die fachlichen 23 Nachfolgerdateien des abgelehnten r1446 bleiben bytegleich.
Die fachlichen Vertraege mit r1445/r1446 im Dateinamen behalten ihre Funktion;
die neuen Revisionsnummern bezeichnen nur die Reparatur-Abfolge.
A4/r1444 bleibt die verbindliche native Referenz fuer alle 20 finalen XMLs.
Allgemeine Render-Custody und Boegen ueber Systemwechsel bleiben offen.

Der Paketname des Kodex-Takes bleibt gemaess dem bereits angenommenen
K2-Vertrag SscK2GovernanceRevisionKodex. Es gibt keine neue Namensausnahme.
Die alten Tabellen und Revisionsangaben des urspruenglichen Vertrags sind
historisch; fuer Paketidentitaet und Manifest ersetzt sie diese Reparatur.
Der frische native Child-Abnahmelauf ist r1448 statt r1446.

| Aktuelle Bindung | SHA-256 |
| --- | --- |
| Parent ZIP_KIND.txt | `33425ae9a23caef738cd4b804d23dc98a80b013935c997db79f912b601d8de8f` |
| Child ZIP_KIND.txt | `3f32028b99895d95dd61844169ce880bc1c85d106b21c60fa0d705af080187ec` |
| Child src/build_gate/take_change_class.txt | `9f902c083ec19782ae3f4182789abab66d125195ed7948a71ab8b4b9926f7f4f` |
