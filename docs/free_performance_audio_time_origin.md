# WAV-Zeitursprung beim Import freier MIDI-Einspielungen

Der Import vergleicht akustische Anschläge mit den Einsatzgruppen der MIDI.
Er prüft zuerst das bestehende Zeitfenster um den gemeinsamen Zeitnullpunkt.
Liegt der erste gespielte MIDI-Einsatz später als dieses Suchfenster und
liefert die erste Suche keine gültige Zuordnung, berücksichtigt der Import
ein zweites Fenster um den möglichen Anfang einer separat zugeschnittenen WAV.

Die abgeleitete Verschiebung muss die erste und letzte Einsatzgruppe innerhalb
der Audiozeit abdecken. Die akustische Evidenz muss sich von den übrigen
geprüften Verschiebungen abheben und dieselbe Zuordnung in jedem Drittel
bestätigen. Diese lokale Kontrolle toleriert kleine Abweichungen zwischen
den Anschlagstransienten. Sie erzeugt keine abschnittsweise Zeitverschiebung.
Die Notationszeitableitung erhält die ursprünglichen MIDI-Ereignisse.

Die Freigabe dieser Zuordnung belegt nur die akustische Zeitreferenz.
Die Ableitung der Taktart, der Taktphase und der geschriebenen Dauern
muss ihre eigenen Bedingungen weiterhin erfüllen.
