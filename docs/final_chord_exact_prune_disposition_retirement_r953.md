# r953 FinalChordExactPruneDispositionRetirement

AuthorityParent ist r945 `BasementLegacyObservationPhysicalStartRecovery`.
Die verworfenen Kandidaten r946-r952 bleiben ausschliesslich Failure Evidence und werden nicht als Parent verwendet.

## Ziel

Die r946-r952-Serie zeigte zwei duale FinalChord-Bindefehler bei render-only geprunten, strukturell identischen Chord-Occurrences. Der bisherige r945-Pfad verwirft an der Prune-Grenze die bereits bekannte Source-Occurrence-Identitaet und rekonstruiert spaeter eine Omission heuristisch aus:

`measure + onset + pitch + staff + tie`

Das ist bei identischen Chord-Mitgliedern nicht owner-eindeutig.

r953 entfernt diese heuristische Rekonstruktion ersatzlos. Die bereits vorhandene und validierte `RetentionFirstP2SplitOccurrenceReceipt` enthaelt alle ueberlebenden pitched Source-Occurrences mit ihrem urspruenglichen `(scope, raw_note_index)`. Im InitialRender-Stage existiert gleichzeitig die eindeutige `(measure, raw_note_index) -> canonical FragmentId`-Abbildung. Nur wenn die Render-Prune-Projektion wirklich `pdf_source_selected` ist, wird daraus die exakte Differenzmenge der geprunten canonical FragmentIds gebildet. P2-generierte Occurrences mit Writer-Witness werden aus dieser Omission-Menge wieder entfernt.

Der spaetere FinalChord-Binder konsumiert damit nur noch exakte canonical FragmentIds als Render-Prune-Omission. Ein zertifiziert geprunter Owner wird vor `raw_note_index`-/semantischer Ruecksuche als omitted disponiert; er kann deshalb nicht mehr als strukturell identischer Survivor wiederauferstehen oder dessen physische Occurrence als zweiter Owner beanspruchen.

## Fail-closed Grenzen

- Kein Omission-Recht, wenn die Render-Prune-Projektion nicht committed/selected wurde.
- Split-Survivor-Receipt muss cardinality- und identity-stabil gegen CandidateMaterial sein.
- Die Differenzmenge muss exakt der akzeptierten Zahl `removed_safe_hidden_inner_chord_tones` entsprechen und darf nur hidden chord pitched fragments enthalten.
- P2-generierte canonical Owner mit Writer-Witness werden nicht als omitted behandelt.
- TiedAnchorChain-Omission bleibt nur erlaubt, wenn die komplette generierte Chain physisch absent ist; partielle Chain bleibt blockierend.
- Master/normal binding bekommt weiterhin keine Render-Omission-Authority.
- Unbekannte/fehlende Omission-Certificates bleiben blockierend.
- Keine Writer-, MusicMutation-, Repair-/Recovery-, Renderer- oder Gate-Authority wird erweitert.

## Physischer Abbau

Entfernt werden:

- `safe_render_pruned_final_chord_omission_keys()`
- die strukturelle 6-Tupel-Rekonstruktion `(measure,onset,pitch,staff,tie_start,tie_stop)`
- die nachtraegliche physische Aehnlichkeitspruefung als Omission-Owner-Beweis
- exception-driven `find -> error -> maybe omission` fuer die drei FinalChord-Omission-Pfade
- ein inzwischen schwaecherer Same-Pitch-NonChord-Test zugunsten eines strengeren identischen-Chord-Owner-Regressionsfalls

Source-Diff gegen r945 vor Take-Metadaten: `+200 / -242`, netto `-42` Zeilen.

Keine neue Identity-/Receipt-Klasse, kein neuer Fingerprint und keine neue produktive Writer-Root.
