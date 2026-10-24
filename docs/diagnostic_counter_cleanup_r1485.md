# r1485: Diagnosezähler der Pausenbrücke bereinigt

Parent: r1484. Class: GovernedChange, music_effect none. Autor: Dachs.
Keine Musik-, XML- oder PDF-Änderung; nur Bericht- und Konsolentext.

1. `LowerRestBridgeCells` entfällt. Der späte RestBridge-Teil von
   `close_structural_beam_gestures` (E048) ist seit r1478 stillgelegt; der Zähler war
   seitdem in jedem Lauf 0. Entfernt aus Statistik, Audit und den drei Berichtzeilen
   von `ExternalFeedbackClosureRenderWrite` sowie aus drei Test-Asserts, die nur
   diesen immer-0-Zähler prüften. Den Schutz vor einer Wiederkehr liefern die
   Verhaltenstests (`late_closure_does_not_restore_a_withheld_bridge` u. a.).
2. `split_groups` in `BridgeFrozenVoiceCompatibility` zählt nur noch gesperrte
   Gruppen, deren Schreiben einen Balkenzustand ändern würde. Gruppen ohne
   Änderung erscheinen als `unchanged_split_groups`. Bisher meldete der Korpus
   z. B. E7 `split_groups=19` ohne jede tatsächliche Sperrwirkung.

Abnahme: 20 XMLs bytegleich zu r1484, PDFs pixelgleich, MuseScore ohne `-f` mit
erster Variante; Summe `split_groups + unchanged_split_groups` je Etüde gleich dem
bisherigen `split_groups` aus r1484, der größte Teil davon `unchanged_split_groups`.
