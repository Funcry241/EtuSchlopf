# r611 – PianisticHandConfigurationReferenceBracketAudit

## Zweck

r607 bis r610 haben die Score-Seite der Spielbarkeitsfrage getrennt materialisiert:

- statische Zwei-Hand-Positionsfrontier ohne feste menschliche Spanngrenze,
- exakte Zeitfenster und Same-Key-Re-Attack-Demands,
- assignment-freie Tastenzustandsgeometrie über die reale TempoMap.

r611 bindet erstmals wieder eine **explizite Hand-/Finger-Referenzannahme** gegen diese Demand-Seite, ohne aus dem Score selbst Capability-Antworten abzuleiten.

## Zwei getrennte Referenzmodelle

Das bereits vorhandene r299-Modell bleibt unverändert:

- Standard-88-Tasten-Klaviatur,
- höchstens fünf verschiedene Tasten pro Hand,
- monotone simultane Fingerordnung,
- äußere Spannweite je Hand höchstens 12 Halbtöne,
- zeitliche Übergänge in r299 noch auf 0 µs relaxed,
- `REFERENCE_ASSUMPTION`,
- `MODEL_RELATIVE_ONLY`.

r611 ergänzt ein zweites, separat SHA-256-gebundenes **Structural-Ceiling-Referenzmodell**:

- Standard-88-Tasten-Klaviatur,
- höchstens fünf verschiedene Tasten pro Hand,
- monotone simultane Fingerordnung,
- **keine** äußere Spanngrenze,
- keine Transition-Capability,
- keine empirische Human-Capability,
- `REFERENCE_ASSUMPTION`,
- `MODEL_RELATIVE_ONLY`.

Dieses zweite Modell ist absichtlich zu permissiv, um als menschliche Wahrheit zu gelten. Es bildet nur die maximal lockere statische Oberklammer innerhalb der vorhandenen v1-Fünf-Finger-Ontologie.

## Bracket-Frage

Für jede simultane Grenze werden alle beschrifteten Links/Rechts-Partitionen der **verschiedenen** aktiven physischen Tasten erschöpfend geprüft.

Gezählt werden getrennt:

- Partitionen, die das Structural-Ceiling-Modell unterstützt,
- Partitionen, die zusätzlich das konservative r299-Modell unterstützt.

Daraus entstehen drei wichtige Klassen:

1. **beide Referenzen unterstützen mindestens eine Partition**  
   Der Zustand ist bereits unter der konservativen r299-Spanannahme statisch modellrealisierbar.

2. **nur Structural Ceiling unterstützt**  
   Die Blockade hängt spezifisch an der r299-Spanannahme. Das ist ein Kalibrierungsbedarf, aber noch kein menschlicher Capability-Beweis.

3. **selbst Structural Ceiling unterstützt nicht**  
   Dann liegt innerhalb der v1-Fünf-Finger-Ontologie ein struktureller statischer Gegenfall vor, etwa mehr als zehn verschiedene simultane Tasten.

Same-Key-Overlaps bleiben absichtlich getrennt. Sie werden gezählt, aber nicht durch eine statische Mehrfingerbelegung derselben Taste „gelöst“. Ihre Semantik bleibt r608-Technik-/Zeitbedarf.

## Nicht-Ziele

r611 behauptet ausdrücklich nicht:

- dass ein realer Mensch unbegrenzte Handspanne besitzt,
- dass die Zielpianistin eine bestimmte Spanne besitzt,
- dass ein Structural-Ceiling-Witness menschlich spielbar ist,
- dass Hand- oder Fingerbewegungszeiten bereits bekannt sind,
- dass Same-Key-Re-Attacks ohne Pedal-/Release-/Repetitionstechnik lösbar sind,
- dass Tempo-Feasibility bewertet ist.

`HumanPlayabilityClaim` bleibt `NOT_EVALUATED`.

## Sicherheitsgrenzen

- `ChangeClass: AuditOnly`
- `WriterAuthority: none`
- `MusicMutation: no`
- `MasterApply: no`
- `RenderXmlWrite: no`
- `EvidenceBasis: REFERENCE_ASSUMPTION`
- `ClaimBoundary: MODEL_RELATIVE_ONLY`
- `EmpiricalCalibration: NOT_BOUND`
- `HandTransitionCapability: NOT_BOUND`
- `FingerTransitionCapability: NOT_BOUND`

## Erwarteter Erkenntnisgewinn

Nach r611 ist erstmals sauber unterscheidbar, ob ein statischer r299-Gegenfall

- wirklich schon an der Fünf-Finger-Struktur scheitert,
- oder nur an der konservativen 12-Halbton-Annahme.

Erst danach ist ein empirisches per-Hand-Capability-Profil sinnvoll: Es kann zwischen diesen beiden Modellklammern unabhängig gebunden werden, ohne zirkulär aus dem Score abgeleitet zu sein.
