# r1008 VisibleRhythmicOccupancyClosureAudit

r1008 baut direkt auf dem real akzeptierten und dunkelgruenen r1007 `NotationVoiceUpstreamLineagePreservationAudit` auf. r1007 zeigte, dass die gepruefte `pre_lane_voice` fuer 180/182 `MetricNotationLine`-Kandidaten im Wesentlichen nur die urspruengliche Staff-Seed-Voice fortsetzt und deshalb keine unabhaengige positive NotationVoice-Authority sein kann. Gleichzeitig bleibt der aktuelle PDF-Korpus bei Balken und Pausen sichtbar unzureichend: Balkenfragmentierung ist breit verteilt, und Rest-/Pitch- sowie Doppelrest-Konflikte muessen vor einem produktiven Voice-Writer aus derselben repraesentierten Zeitwahrheit pruefbar sein.

r1008 bleibt deshalb vollstaendig read-only. Der Take fuehrt innerhalb der bereits deklarierten `NOTATION-VOICE-PRE-SERIALIZATION-CONSTRAINT-CLOSURE-AUDIT`-Sektion eine gemeinsame voice-, beam- und SoundingLane-freie `VisibleRhythmicOccupancy`-Projektion ein. Es entsteht keine neue Reportsektion.

## Gemeinsame sichtbare Zeitwahrheit

Die bereits transaction-lokal gebundenen `PreSerializationVisibleOccupancyRow`-Zeilen werden nach `atomic_group_id` zu sichtbaren rhythmischen Atomen geschlossen:

- eine sichtbare einzelne Pitch-Note wird `PITCHED_NOTE`;
- ein sichtbarer Akkord aus genau einem normalen Anchor ohne `<chord/>` plus sichtbaren `<chord/>`-Mitgliedern mit identischem dargestelltem Intervall wird genau ein `PITCHED_CHORD`-Atom;
- eine sichtbare Pause wird `REST`;
- hidden-only Gruppen erzeugen keine sichtbare Occupancy;
- sichtbare Orphan-Chords, gemischte Gruppen, fehlende Pitch-Identitaet, uneinheitliche Intervalle oder andere nicht atomar schliessbare Gruppen bleiben `VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE` und erhalten keinerlei Autoritaet.

Aktuelle `<voice>`-Labels, Beam-Tags und `FrozenSoundingTemporalLanePlan` duerfen weder Atomgrenzen noch Konfliktklassifikation oder metrische Kontinuitaet waehlen.

## Konfliktklassen

Die Projektion unterscheidet drei Sachverhalte bewusst:

1. `DUPLICATE_REST_OCCUPANCY`: zwei sichtbare Rest-Atome besitzen auf demselben Staff dasselbe dargestellte Intervall. Das ist `REST_OWNERSHIP_CONFLICT_ANALYSIS_ONLY` und kein automatisches `HARD_DIFFERENT`.
2. `REST_PITCHED_OVERLAP`: sichtbare Rest- und Pitch-/Chord-Occupancy ueberlappen sich auf demselben Staff. Auch dies ist ein Rest-Ownership-Konflikt, nicht automatisch ein Stimmtrennungsbeweis.
3. `INDEPENDENT_PITCHED_OVERLAP`: zwei verschiedene sichtbare Pitch-/Chord-Atome mit disjunkter SourceAttack-Identitaet ueberlappen sich repraesentiert auf demselben Staff. Dies wird ausschliesslich als `HARD_DIFFERENT_CANDIDATE_ANALYSIS_ONLY` ausgewiesen; r1008 schreibt keine Constraint und keine Voice.

Damit werden Rest-Ownership und negative Pitched-Polyphonie erstmals aus derselben dargestellten Occupancy abgeleitet, ohne klingenden Nachhall mit sichtbarer Gleichzeitigkeit gleichzusetzen.

## Metrische Sicht

Auf Basis der MusicXML-`divisions` zaehlt r1008 pro `(measure, staff, beat)` sichtbare Occupancy-Segmente mit mehreren Atomen und trennt:

- lueckenlos kontiguierliche Scopes ohne sichtbare Ueberdeckung,
- Scopes mit Resten,
- Scopes mit sichtbarer Ueberdeckung.

Die bestehende r1006-`MetricNotationLine` bleibt unveraendert erhalten und wird nicht durch den neuen Audit ersetzt. r1008 schafft lediglich die gemeinsame atomare Grundlage, auf der spaeter NotationVoice, Rest-Ownership und Beam-Gruppierung gemeinsam geprueft werden koennen.

## Grenzen

- kein MusicXML-/PDF-/Musik-Writer;
- keine Repair-, Selection- oder Release-Authority;
- keine takt- oder etuedenspezifische Sonderlogik;
- der historische Astra-Katalog bleibt visuelle Referenz, aber kein Vollstaendigkeitsbeweis und kein Selektor;
- ein Restkonflikt ist nicht automatisch `HARD_DIFFERENT`;
- ein `HARD_DIFFERENT_CANDIDATE_ANALYSIS_ONLY` wird in r1008 noch nicht produktiv materialisiert.

Zwei generische Regressionen kommen hinzu. Die erste beweist, dass ein normaler sichtbarer MusicXML-Akkord als genau ein Pitch-Occupancy-Atom behandelt wird und keinen falschen Pitched-overlap erzeugt. Die zweite trennt deckungsgleiche Doppelpausen von Rest/Pitch-Ueberdeckung. Erwarteter Strict-Testbestand: **1320**.
