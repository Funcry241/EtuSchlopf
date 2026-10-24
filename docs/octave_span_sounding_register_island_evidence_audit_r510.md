# r510 – OctaveSpanSoundingRegisterIslandEvidenceAudit

## Ausgangspunkt

r509 `CanonicalMeasureRestMetricTruthCallGraphPreservationClosure` ist der installierte gruene Parent. Der naechste Sichtbefund betrifft die obere Hand von Etuede 4 in den Takten 22–24: materialisierte Oktavlinien enden bzw. beginnen so, dass einzelne oder kurze klingende Registerbereiche ohne Oktavlinie stehen und dadurch viele Hilfslinien entstehen.

Der aktuelle produktive `registernotation`-Owner besitzt bereits Grenzenevidenz, Segmentbildung, Mindest-Elegibilitaet und Lesbarkeitskosten. Dieser Audit veraendert diese Authority nicht und dupliziert ihre Kostenfunktion nicht.

## Vorhandener Baustein

`src/audit_only/oktavspannentopologie_pruefung.rs` ist bereits eine ausschliesslich lesende MusicXML-Topologiepruefung fuer Haupt- und Darstellungsdatei. Sie besitzt weder Writer-, Repair-, Auswahl- noch Musikmutationsrecht.

r510 erweitert genau diesen vorhandenen Audit im bereits bestehenden XML-Lesedurchlauf um eine weitere Beobachtungsdimension; es entsteht kein zweiter Vollscan und keine parallele Registerwelt:

- vollstaendige, bereits materialisierte Oktavspannen werden nach Teil und System geordnet,
- zwischen zwei aufeinanderfolgenden nicht ueberlappenden Oktavspannen wird das offene Intervall betrachtet,
- nur wenn dort klingende Tonereignisse liegen, entsteht eine `KlingendeZwischeninsel`-Evidenz,
- berichtet werden Angriffszahl, Tonobjektzahl, MIDI-Minimum/-Maximum sowie Richtung und Groesse der benachbarten bereits materialisierten Oktavspannen,
- die Beispiele werden nach kleinster Angriffszahl priorisiert.

## Harte Grenze

Der Audit entscheidet nicht:

- ob eine bestehende Registernotationsgrenze tragend oder schwach sein soll,
- ob eine Oktavspanne verlaengert, fusioniert oder neu geplant werden soll,
- ob 8va, 15ma, 8vb oder 15mb gewinnt,
- ob die produktive Mindestanzahl von Tonobjekten geaendert werden soll,
- welche Kosten oder Verbesserungsschwellen gelten.

Insbesondere wird die produktive `lesbarkeitswert`-/`entscheide_oktavdarstellung`-Logik nicht kopiert. r510 liefert nur Artefakt-Evidenz fuer die naechste Architekturentscheidung.

## Ziel des Laufs

Der erste Abnahmelauf ist `Etüde Nr. 4`. Relevant ist, ob die Darstellungsdatei unter den kuerzesten `KlingendeZwischeninsel`-Beispielen die beobachteten Registerunterbrechungen im Bereich Takt 22–24 sichtbar macht. Danach wird korpusweit geprueft, ob dieselbe Evidenzklasse weitere plausible oder unplausible Faelle zeigt.

Erst nach diesem Audit darf entschieden werden, ob eine produktive Register-Closure sinnvoll ist. Eine moegliche spaetere Richtung `MandatoryStop -> ReEvaluationBoundary` ist ausdruecklich **nicht** Bestandteil von r510.

## Unveraendert

- keine MusicXML-Mutation,
- keine Master-/Render-Mutation,
- keine neue StructuralWriteAuthority,
- keine Writer-Aufrufkante,
- keine Repair-/Recovery-Schicht,
- keine neue Register- oder Layout-Authority,
- keine Etueden-/Takt-/Staff-/Voice-/Pitch-Identitaetsregel,
- keine Schwellenwert- oder Passordnungsveraenderung,
- kein Wiederbeleben des pensionierten `LowerStaffRegisterIslandPreviewAudit`.
