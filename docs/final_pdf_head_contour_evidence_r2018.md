# r2018 · Eingebettete PDF-Notenköpfe und Halsansatz

Der Audit baut auf dem angenommenen r2017 auf. Die MuseScore-Probe bindet
Notenköpfe über die PDF-Schriftressource, den tatsächlich gezeichneten CID,
die eingebettete TrueType-Kontur und die Text- und Seitenkoordinaten. Die
Konturen kommen aus den tatsächlichen Glyphenpunkten. Informationen zur
Zeichenbreite und zur Glyphenhülle dienen zusätzlichen Konsistenzprüfungen.

Die Probe verwendet den bestehenden Halsplaner. Sie prüft den geteilten
Haltebogen in drei gerenderten Fassungen bei Systemwechsel, vor der Änderung
bei Seitenwechsel und nach der Änderung bei Seitenwechsel. Original und
markierter Export müssen dieselben Schriftressourcen und Zeichenoperationen
verwenden. Die markierte Ausgabe bindet die vorhandenen Quellmarken. Der
Prüfer weist eine abgeschnittene Glyphe, fehlende oder mehrdeutige Markierung
und einen abweichenden Ausgabestand zurück.

Die aus dem PDF gelesene Kontur muss zur unabhängigen SVG-Kontur passen.
Der Prüfer begrenzt Kurven- und Abtastfehler. Zur Prüfung des eigenen Halses
weist er eine kleine offene Kreisfläche nach, die gleichzeitig im
tatsächlich gefüllten Kopf und im gezeichneten Hals liegt. Die Berührung
liegt in der erwarteten Ansatzregion. Bei fremden Köpfen fordert die
Probe einen Abstand von mindestens einer normalen Halsstrichbreite
zwischen konservativen Außenhüllen. Sie behält die bestehenden Belege für
Haltebogenfragmente, Seitenzuordnung und betroffene Paare bei.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf tied_stem_correction_renders_across_page_breaks -- --ignored --nocapture
```

`ETUSCHLOPF_PDF_RENDERER` bezeichnet die lokale MuseScore-Programmdatei. Die
Probe läuft im Testpfad. Das qualifizierte Profil umfasst die geprüften
MuseScore-Glyphen und explizit unterstützten Zeichenoperationen. Der
reguläre PDF-Lauf braucht zusätzlich die vollständige Prüfung der durch
eine Änderung betroffenen fremden Tinte. Die gemeinsame Übernahme des
belegten MusicXML/PDF-Paars in die endgültige Ausgabe liegt hinter der
exakt gebundenen Writer-Grenze von Checkpoint 58.
