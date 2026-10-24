# Vor dem langen Diagnose-Build

1. Akzeptierte Basis r1463 und isolierte Scratch-Kopie festhalten. Vorabcheck
   auf genau dieser Kopie ausführen; Manifestfehler zuerst bewusst auflösen.
   Das Hilfsskript schreibt keine neuen Hashes und erteilt keine Take-Zulassung.
2. Den tatsächlichen Renderer-Aufruf verwenden. Bei einem Wrapper zusätzlich
   die wirkliche MuseScore-Binärdatei benennen. PDF- und SVG-Smoke müssen mit
   Exit 0 ohne Force gelingen. Danach bleibt der eigentliche Geometrie-Readback
   zu qualifizieren; ein erfolgreiches PDF allein reicht dafür nicht.
3. Vor dem Build alle gewünschten Felder und frühen Ausgänge festlegen.
   Diagnosen nur in der Scratch-Kopie; nicht gleichzeitig einen Reparatur-
   Selektor verändern. Source-/Manifest-Fingerprints zum Diagnosebefund sichern.
4. Erst die billigen Prüfungen, dann Build, dann der tatsächlich benötigte
   Etüdenlauf. Kein langer Lauf ohne funktionierenden Renderer. Ein kleiner
   Einzeltest darf einen später erforderlichen Gesamtlauf nicht ersetzen.

## Pro Gruppe mindestens erfassen

- Opaque Gruppenkennung, Part, Taktordinal und Staff als Beleg-Locatoren.
- Zahl ursprünglicher primärer Gruppen und ursprünglicher Mitglieder.
- Renderer-Geometrie vorhanden? Erwartete und beobachtete Komponentenzahl.
  **Fehlende Geometrie bleibt unbekannt und wird nicht zu null oder „intakt“.**
- Vorheriger Plan beansprucht die Gruppe? Aktueller abschließender Status:
  bereits behandelt / nachgewiesen nicht fragmentiert / vorgeschlagen /
  verworfen / Geometrie nicht verfügbar.
- Exakte erste scheiternde Bedingung und Diagnoseort. Falls zusätzliche
  Bedingungen geprüft werden: als solche kennzeichnen, keine Mehrfachzählung.
- Bei Mitgliedschaftsfehlern das betroffene Source-/Attack-/Fragment-Mitglied
  und die konkrete Abweichung; keine Identität aus Pitch/Voice rekonstruieren.
- Für Rollen-/Ankerfehler: beobachtete Rolle, erforderliche Rolle, vorhandener
  Anchor und dessen gebundene Identität; nicht nur „role failed“.
- Für Staff-/Voice-Konflikte: betroffene gebundene Vorkommen und deren Staff,
  Voice, Onset und Dauer. Für Geometrie-/Beam-Drift beide tatsächlichen Werte.

Die Tabelle `ablehnungen_vorlage.tsv` ist eine leere Feldvorlage, kein neues
produktives Reportschema. Fehlende Werte ausdrücklich als unbekannt benennen.
Pro Originalgruppe genau einen Endstatus zählen; bei 247 Eingangsgruppen müssen
auch 247 Endstatus-Zuordnungen vorliegen, nicht 247 frei kombinierte Meldungen.
„Vorgeschlagen“ ist keine Behauptung einer committed oder sichtbaren Reparatur.

## Vor dem Ableiten eines Takes

Erste scheiternde Bedingungen können spätere Hindernisse verdecken. Einen Check
probeweise zu entfernen beweist deshalb keine sichere Reparierbarkeit. Die
allgemeine Ursachenklasse, vollständige Originalbindung und unabhängigen
Safety-Prädikate müssen separat begründet werden. Bestehende Gates unverändert
anwenden; keine Umbenennung oder Verlagerung zum Umgehen einer Klassifikation.

Profilerzählungen sind keine XML-Hashes. Gleiche MuseScore-Version bedeutet
keine vollständige Gleichheit von Plattform, Fonts, Profil und Einstellungen.
Ein lokaler Smoke ist keine native Korpus- oder PDF-Abnahme.
