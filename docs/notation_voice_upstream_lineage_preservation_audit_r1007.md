# r1007 NotationVoiceUpstreamLineagePreservationAudit

r1007 baut direkt auf dem real akzeptierten und dunkelgruenen r1006 `NotationVoiceMetricLineBaselineOverlayAudit` auf. r1006 hat beam- und aktuelle-Voice-label-frei 182 `MetricNotationLine`-Hypothesen konstruiert und fuer 147 lokal eindeutige Kandidaten einen baseline-erhaltenden Channel-Overlay ohne globales Greedy-Replanning geschlossen. Der Audit zeigte zugleich die verbleibende logische Grenze: metrische Kontinuitaet, self-induced LanePressure und konfliktfreie sichtbare Occupancy beweisen eine konsistente gemeinsame Linie, aber fuer sich allein noch nicht, dass absichtlich alternierende Notationsstimmen zusammengelegt werden duerfen.

r1007 bleibt deshalb vollstaendig read-only und prueft fuer jeden bereits in r1006 konstruierten `MetricNotationLine`-Kandidaten die vor der technischen Sounding-Lane-Zuweisung vorhandene Representation-Voice-Lineage.

## Upstream-Voice-Carrier

`CandidateMaterialAttackState.voice` ist die zu pruefende Upstream-Voice. Diese Voice gehoert zur bereits gebundenen CandidateMaterial-Lineage. Der `FrozenSoundingTemporalLanePlan` wird erst danach aus dem gefrorenen CandidateMaterial-Zustand erzeugt; der `FrozenSerializationVoiceChannelPlan` folgt wiederum auf diesen technischen Lane-Plan. r1007 kopiert deshalb die CandidateMaterial-Voice zusammen mit SourceEventId und SoundingAttackId in die bestehende read-only `PreSerializationSourceAttackRow`.

Die Voice wird nicht zur Bildung einer `MetricNotationLine` verwendet. Die r1006-Selektion bleibt beam- und current-voice-label-frei. Erst nach der Kandidatenbildung fragt r1007, ob alle exakt gebundenen Mitglieder derselben MetricLine dieselbe bereits vorhandene Pre-Lane-Voice tragen.

## Fail-closed Klassifikation

Pro MetricLine gilt:

- fehlt die eindeutige SourceAttack-Bindung oder eine Pre-Lane-Voice, lautet der Status `BLOCKED_INCOMPLETE_UPSTREAM_VOICE_BINDING`;
- tragen die Mitglieder mehr als eine Pre-Lane-Voice, lautet der Status `BLOCKED_UPSTREAM_VOICE_AMBIGUITY`;
- nur bei exakt einer gemeinsamen Pre-Lane-Voice lautet der Status `UPSTREAM_LINEAGE_PRESERVATION_PROVEN_ANALYSIS_ONLY`.

Damit bekommt insbesondere ein adversariales metrisch lueckenloses A-B-A-B-Hocket mit intern erklaertem LanePressure kein Preservation-Recht, sobald dessen Mitglieder bereits upstream auf verschiedene Stimmen verteilt waren. Der Audit erfindet keine neue Stimme, sondern prueft nur, ob die spaetere SoundingLane-/Serialization-Schicht eine bereits einheitliche Representation-Voice aufgespalten hat.

Die r1006-Baseline-Overlay-Pruefung bleibt ein separater notwendiger Begleitbeweis. r1007 fuehrt weiterhin keinen Writer ein und verleiht weder Repair-, Selection- noch Release-Authority. Es wird weder MusicXML noch PDF noch Musik geschrieben. Der visuelle Fehlerkatalog bleibt ausschliesslich Abnahme-Oracle und ist kein Selektor.

Zwei generische Regressionen kommen hinzu: eine beweist, dass unterschiedliche aktuelle XML-Voice-Labels eine upstream einheitliche MetricLine nicht blockieren; die zweite erzeugt gezielt alternierende Pre-Lane-Voices und verlangt trotz weiterhin vorhandener MetricLine `BLOCKED_UPSTREAM_VOICE_AMBIGUITY`. Erwarteter Strict-Testbestand: **1318**.
