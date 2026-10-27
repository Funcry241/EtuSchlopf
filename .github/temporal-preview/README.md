# Entscheidung B und naechste verbindliche Grenzen

Otter hat am 27.10.2026 B gewaehlt: allgemeine Takt-/Zeitunterstuetzung vor der
naechsten EXE-Veroeffentlichung. Der echte Fremdstueck-Positivtest bleibt
unveraendert verpflichtend. Er wird weder entfernt noch als erwarteter Fehler
in einen erfolgreichen Musiktest umgedeutet.

Die r1530-Ergaenzungen entfernen die unzutreffende Komponistenzuordnung,
geben dem Benutzer eine neutrale Eingabefehlermeldung und freundlich erklaerte
MuseScore-Hinweise. Die privaten Renderer-Zwischenprodukte liegen beim EXE-Lauf
ausserhalb des sichtbaren Ergebnisordners. Die vier r1529-Bindungen bleiben
unveraendert; die Annahmepruefung laesst die Ergaenzungen weiterhin als denselben
RefactorOnly-Nachfolger zu. Eine native Annahme wird damit nicht behauptet.

Der unabhaengige B-Prototyp enthaelt ausschliesslich den Eingangsteil:

- Exakte ganzzahlige Zeitdomaene statt Rundung: minimale noetige Erweiterung
  des historischen 96er-Masses aus allen Note-On/Off-Grenzen und Taktmetadaten.
- Chronologische Taktwechsel mit expliziter exakter Teilspanne bei einem
  Wechsel innerhalb eines Takts; keine stille Verschiebung von Grenzen.
- Kompakte zeitliche Segmente statt einer Speicherallokation pro leerem Takt.
- Unveraendertes MIDI-Pairing und ein Kausalbeweis fuer alle Originalpaare in
  der tatsaechlich verwendeten Zeiteinheit. Die Originaldatei wird nicht geaendert.
- 16 isolierte Tests mit verweigerten Compilerwarnungen und ein realer
  Importbeleg des unveraenderten Menuetts: 204 Noten, 408 Rohereignisse,
  384 Divisions, 32 Takte in 3/4, jede Position/Dauer exakt rueckgeprueft.

Der Prototyp ist nicht in die produktive EXE eingebunden. Insbesondere sind
Notationsdauer, Pausen, Tuplet-Schreibweise, Hotspot-/Solver-Zeitgrenzen,
eingefrorene Klang-/Stimmenobligationen und ihre Einheiten noch nicht auf den
neuen Verlauf umgestellt. Ein erfolgreicher Import ist kein bestandener
Fremdstueck-Musiklauf und ersetzt keinen Master-/Render-/PDF-/Berichtsbeweis.

Fuer die produktive Fortsetzung gilt diese Reihenfolge: Quellzeit- und
Taktverlauf als getragener Kontext; danach Seed und Notenwerte einschliesslich
Tuplets; dann alle konsumierenden Zeit-/Eigentuemerpruefungen; anschliessend
Fremdstueck mit/ohne PDF, Korpusparitaet und EXE-Ergebnisschutz. Dachs prueft
die konkreten Pakete, bevor Otter einen nativen Lauf investiert. Keine
vorweggenommene Anzahl weiterer Takes und keine Freigabe nach Weg A.
