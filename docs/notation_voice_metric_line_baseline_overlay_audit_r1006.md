# r1006 NotationVoiceMetricLineBaselineOverlayAudit

r1006 baut direkt auf dem real akzeptierten und dunkelgruenen r1005 `NotationVoiceCounterfactualOccurrenceMultiplicityIsolationAudit` auf. r1005 hat den lokalen Materialisierungs-Counterfactual fuer 130 eindeutig gebundene Kandidaten geschlossen und zugleich mit Etuede 7 gezeigt, dass eine gemeinsame Neuberechnung des globalen greedy Channel-Planners trotz lokal gruener Kandidaten die Partition von Nicht-Kandidaten veraendern kann. Ausserdem ist der bestehende Beam ein starker sichtbarer Witness, aber keine unabhaengige SAME-Line-Authority: der unmittelbar relevante Beam-Owner gruppiert bereits nach Staff, bestehender Voice und Beat.

r1006 bleibt deshalb vollstaendig read-only und fuehrt zwei neue, voneinander getrennte Auditprojektionen ein.

## Beam-/Voice-freie MetricNotationLine

Die positive Kandidatenbildung benutzt weder aktuelle MusicXML-Voice-Labels noch `<beam>`-Zustaende. Sie arbeitet ausschliesslich mit:

- MusicXML-`divisions` als metrischer Beat-Grenze,
- Measure und Staff,
- sichtbarer atomarer gepitchter Occupancy,
- lueckenloser kurzer Darstellung innerhalb genau eines Beats,
- kanonischer `(measure, pitched_ordinal) -> (SourceEventId, SoundingAttackId)`-Bindung,
- semantischem Onset und semantischer Dauer,
- kausal rekonstruiertem self-induced LanePressure,
- vollstaendiger sichtbarer Same-Staff-Occupancy-Pruefung im Kandidatenintervall.

Mehrgliedrige sichtbare Akkordatome sind fuer den ersten Writer-Domain bewusst noch nicht positive Line-Kandidaten. Sie bleiben erhalten und koennen einen Bereich blockieren; hidden technische Carrier erzeugen weiterhin keine sichtbare Occupancy-Authority. `voice` und `beam` werden nur nachtraeglich als Diagnose verglichen und selektieren keinen Kandidaten.

## Baseline-erhaltender Channel-Overlay-Counterfactual

Der zweite neue Audit berechnet keinen neuen globalen `FrozenSerializationVoiceChannelPlan`. Fuer jeden beam-/voice-frei geschlossenen MetricNotationLine-Kandidaten wird ausschliesslich hypothetisch geprueft, ob dessen SourceEvents auf einen bereits im akzeptierten Baseline-Plan vorhandenen Primary-Channel desselben Staffs gelegt werden koennen.

Dabei gilt fail-closed:

- jedes Candidate-SourceAttack muss genau eine physische Occurrence besitzen, bevor ein source-level Channel-Overlay betrachtet wird;
- der Kandidat muss genau einen bereits vorhandenen passenden Primary-Channel besitzen;
- Primary/Secondary desselben Staffs sind die einzigen zulaessigen Ausgangsrollen;
- alle Nicht-Kandidaten behalten exakt ihre bestehende Source->Channel-Bindung; Channel-Rollen werden nicht neu verteilt;
- der Overlay darf keinen neuen sichtbaren gepitchten Channel-Konflikt erzeugen;
- ein Batch umfasst nur lokal gruene, source-disjunkte MetricNotationLines und prueft dieselben Invarianten gemeinsam.

Damit ist die in r1005 sichtbare E7-Nichtkompositionalitaet des globalen Replannings architektonisch aus dem Counterfactual entfernt: r1006 misst eine minimale Baseline-Overlay-Transformation statt einer erneuten greedy Gesamtallokation.

Der bestehende beam-basierte S0-Audit und der r1005 Global-Replan-Counterfactual bleiben unveraendert als Vergleichsevidenz erhalten. Der neue MetricNotationLine-Pfad bekommt keine Writer-, Repair-, Selection- oder Release-Authority. Es wird weder MusicXML noch PDF noch Musik geschrieben. Der visuelle Fehlerkatalog bleibt ausschliesslich Abnahme-Oracle und ist kein Selektor.

Zwei generische Regressionen kommen hinzu: eine entfernt alle Beam-Tags und variiert die Voice-Labels, waehrend dieselbe MetricNotationLine weiterhin erkannt werden muss; die zweite prueft, dass der Baseline-Overlay-Counterfactual nur Kandidatenquellen auf den vorhandenen Primary-Channel legt und den Batch ohne globale Neuplanung schliesst. Erwarteter Strict-Testbestand: **1316**.
