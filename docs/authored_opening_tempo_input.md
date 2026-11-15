# Übernahme einer gebundenen Anfangstempoangabe


Neben der ausgewählten MIDI liegt optional eine Datei mit der Endung
`.opening-tempo.txt`. Ihre Zeilen enthalten die Formatkennung
`etuschlopf-opening-tempo-v1`, die SHA-256 der MIDI, den Notenwert,
die positive ganzzahlige Schlagzahl und optional den Tempotext. Erlaubte
Notenwerte sind `half`, `quarter` und `eighth`, jeweils auch mit dem
Präfix `dotted-`. Die Datei verwendet UTF-8.

Eine fehlende Datei lässt die WAV-Schätzung wie bisher wirken. Eine
fehlerhafte oder fremde Datei bricht den Lauf ab. Ein bereits vorhandener
Metronom- oder Wiedergabetempo-Eintrag im Anfangstakt blockiert die
zusätzliche Angabe. Der additive Writer übernimmt den sichtbaren
Notenwert und den passenden Viertelwert für die Wiedergabe gemeinsam.
