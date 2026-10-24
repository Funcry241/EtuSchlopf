# r1010 VisibleRhythmicOccupancyPhaseDeltaAudit – Packaging Recovery

r1009 wurde vor Installation vom Take-Tree-Integrity-Gate wegen des versehentlich enthaltenen Root-`Cargo.lock` abgewiesen und ist keine Authority. r1010 baut deshalb weiterhin direkt auf dem akzeptierten r1008 `VisibleRhythmicOccupancyClosureAudit` auf. r1008 schloss erstmals die sichtbare rhythmische Occupancy unmittelbar vor Serialization/Presentation (`S0`) voice-, beam- und SoundingLane-frei. Der Korpusvergleich mit den bereits vorhandenen finalen Render-Audits zeigte danach zwei verschiedene Fehlerklassen: Konflikte, die schon in S0 existieren, und sichtbare Rest-/Visibility-Fehler, die erst im persistierten Renderzustand (`S1`) entstehen.

Der fachliche Rust-Payload von r1010 ist gegenueber dem verworfenen r1009-Kandidaten unveraendert; die Recovery betrifft nur erlaubte Paketstruktur und eindeutige Revisionsidentitaet. r1010 bleibt vollstaendig read-only. Es fuehrt keine neue NotationVoice-, Rest-, Beam-, Repair-, Selection- oder Release-Authority ein und schreibt weder MusicXML noch PDF. Die bestehende r1008-Closure bleibt die einzige atomare Definition; fuer S1 werden die persistierten Render-Noten lediglich in denselben `PreSerializationVisibleOccupancyRow`-Adapter ueberfuehrt und danach durch denselben `VisibleRhythmicOccupancy`-Closure-Code geschlossen. Fuer S1 wird SourceAttack-Identitaet nicht erfunden; Pitched-Overlap-HardDifferent bleibt deshalb S0-Provenienzdomäne.

## Phasenvertrag

- `S0`: kanonische FinalChord-Projektion nach Sounding-Freeze und vor dem serialisierten Voice-Channel-Plan.
- `S1`: tatsaechlich persistiertes finales Render-MusicXML.
- aktuelle `<voice>`-Labels, `<beam>`-Tags und SoundingLanes duerfen weder Atomgrenzen noch die S0/S1-Bindung waehlen.
- Voice und Primary-Beam werden erst nach einer voice-/beam-freien strukturellen Bindung als reine Presentation-Diagnose gelesen.

Der Audit klassifiziert unter anderem:

- `PRESERVED_REST_PITCH_CONFLICT`: derselbe strukturell gebundene Rest/Pitch-Konflikt ist in S0 und S1 vorhanden;
- `RENDER_INTRODUCED_DUPLICATE_REST`: eine deckungsgleiche Rest-Duplikation existiert erst in S1;
- `RENDER_INTRODUCED_REST_PITCH_CONFLICT`: ein S1-Rest/Pitch-Konflikt darf nur dann als downstream eingefuehrt gelten, wenn der Rest exakt an eine in S0 versteckte und in S1 sichtbare Rest-Occupancy gebunden ist;
- `RENDER_VISIBILITY_PROMOTION`: ein strukturell identisches S0-hidden Atom wird in S1 sichtbar, ohne dass dieselbe sichtbare Occupancy bereits in S0 existierte;
- `PRESERVED_PITCHED_OCCUPANCY`: strukturell identische sichtbare Pitch-/Chord-Atome bleiben ueber die Phase erhalten;
- `BLOCKED_INCOMPLETE_CHORD_BINDING`: r1008-incomplete Orphan-/nicht anchor-geschlossene Chordgruppen bleiben blockiert und werden nicht passend erfunden.

Ein wichtiger Fail-Closed-Punkt entstand beim Gegencheck von Etuede 6: Eine naive voice-freie S1-Rest/Pitch-Suche sieht neben dem bekannten Fall viele normale polyphone Presentation-Rest-Ueberdeckungen. Diese duerfen nicht automatisch zu `RENDER_INTRODUCED_REST_PITCH_CONFLICT` werden. r1010 zaehlt solche nicht phasenbindbaren S1-Ueberdeckungen deshalb separat als `S1UnboundRestPitchedConflictsDiagnosticOnly`; sie besitzen keine Fehler-, Selection- oder Writer-Authority.

## MetricNotationLine-Phasenbindung

Die bereits in r1008 vorhandenen `MetricNotationLine`-Kandidaten bleiben S0-Selektor. Fuer jeden Kandidaten sucht r1010 im finalen Render ausschliesslich nach derselben `(measure, staff, interval)`-Oberflaeche und derselben geordneten `(onset, duration)`-Folge. Erst nach erfolgreicher struktureller Bindung werden die finalen PresentationVoices und Primary-Beam-Fragmente gezaehlt. Voice oder Beam koennen daher niemals den Kandidaten auswaehlen.

Wenn die finale Struktur nicht exakt an den S0-Kandidaten gebunden werden kann, lautet der Status `UNRESOLVED_RENDER_STRUCTURAL_BINDING`; r1010 erfindet dann keine Voice-/Beam-Zuordnung.

## Grenzen

- kein Writer;
- keine Musikmutation;
- keine Regel- oder Gewichtungsaenderung;
- keine etuedenspezifische Sonderlogik;
- keine aktuelle Voice und kein Beam als Selektor;
- keine aus S1 erfundene SourceAttack-Provenienz;
- keine Forderung, dass die S0-Beweisstruktur in S1 strukturell fortbestehen muss;
- S0 begruendet die Phasenbindung, S1 darf durch Presentation/Serialization andere Voice-/Beam-Strukturen besitzen.

Acht generische Regressionstests sichern Duplicate-Rest-Delta, preserved versus introduced Rest/Pitch, Hidden-to-Visible-Bindung, ungebundene S1-Polyphonie, voice-/beam-freie MetricLine-Selektion und fail-closed Orphan-Chords. Erwarteter Strict-Testbestand nach r1008: **1328**.

## Naechstes Gate

Zuerst nur:

`./run.cmd --take "Etüde Nr. 6"`

Kein `--all`, kein `--inventory`. Danach gezielt Etuede 1 fuer downstream Doppelpausen und Etuede 4 fuer Orphan-Chord plus Visibility-Promotion. Erst wenn diese drei Phasenbilder stimmen, folgt einmal der Vollkorpus.
