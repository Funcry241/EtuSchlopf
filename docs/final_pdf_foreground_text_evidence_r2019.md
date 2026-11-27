# r2019 · Gezeichnete Textglyphen in der PDF-Abstandsprüfung

Der Prüfer liest die sichtbaren Glyphen aus den Zeichenbefehlen und den
eingebetteten Schriftprogrammen. Eine unterstützte `ActualText`-Angabe
liefert Ersatztext für die Extraktion. Der Prüfer liest die Zeichenbefehle
innerhalb der Markierung weiter und bindet jede Glyphe an ihre eigene
Schriftressource, Position und Seitenkoordinaten.

Die Hülle einer fremden Textglyphe umfasst alle Kontrollpunkte der
eingebetteten Kontur. Die Hülle berücksichtigt beide horizontalen
Ursprünge aus Konturkoordinaten und seitlichem Schriftabstand. Kurven
und Füllflächen liegen innerhalb der so gebildeten Außenhülle. Eine
strukturell leere Glyphe hat keinen gefüllten Bereich. Unbekannte
Zeichenbefehle oder nicht belegte Konturen lassen den Nachweis offen.

Der Prüfer berechnet den Abstand zwischen dem tatsächlich gezeichneten
Halsrechteck und der um die Messreserve erweiterten Glyphenhülle. Dieser
Abstand ist eine Untergrenze für den tatsächlichen Abstand des Halses
zur Textglyphe. Der Nachweis verlangt mindestens eine normale
Halsstrichbreite zuzüglich der Rundungsreserve. Die Zuordnung verwendet
die Schriftressource und die jeweilige Zeichenoperation.

Die Probe für das gemeinsame PDF- und SVG-Profil läuft mit MuseScore.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf tied_stem_correction_renders_across_page_breaks -- --ignored --nocapture
```

`ETUSCHLOPF_PDF_RENDERER` verweist auf die lokale MuseScore-Programmdatei.
Die Probe prüft den Halsansatz und den Abstand zu Textglyphen. Eine
separate Probe liest ein vorhandenes PDF über `ETUSCHLOPF_PDF_FOREGROUND_INPUT`
und nennt die nachgewiesenen Textglyphen je Seite. Für die eingeschlossenen
Glyphen gibt sie die Hüllen in Seitenpunkten aus.

```powershell
cargo +1.98.0 test --locked --bin etuschlopf exported_pdf_foreground_text_is_observed -- --ignored --nocapture
```

Die gemeinsame Übernahme in die endgültige MusicXML- und PDF-Ausgabe
erfordert weiterhin eine eigene Zulassung an Checkpoint 58.
