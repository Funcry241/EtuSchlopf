# r565 – PhraseNotationIntrinsicSpanSignatureAudit

## Autoritativer Parent

r565 basiert direkt auf EtuSchlopf v0.10.1839 r564
`PhraseNotationHigherLevelUnityAuditTreeClosure`.

r564 ist der im Vollkorpus bestaetigte 10/10-Green-Stand. Die verworfenen
Kandidaten r561, r562 und r563 bleiben ausserhalb der erfolgreichen Linie.

## Zweck

r564 hat fuer die bestehenden `allow_structural_pending_visual`-Projektionen
exakten Whole-Span-Parallelismus aus Folgen vorhandener
`JointParallelSignature`-Deskriptoren gemessen. Die anschliessende Analyse hat
eine methodische Grenze dieses Deskriptors sichtbar gemacht: Eine lokale
`JointParallelSignature` benoetigt neben den beiden Boundary-Attacks auch einen
vorherigen und einen folgenden Attack. Eine Folge solcher Joint-Signaturen ist
deshalb kein intrinsischer PhraseSpan-Deskriptor; an der linken und rechten
Span-Grenze fliesst Musik ausserhalb der eigentlichen Projektion ein.

r565 fuegt ausschliesslich einen zweiten, intrinsischen Exact-Audit hinzu. Der
bestehende r564-Higher-Level-Unity-Audit bleibt unveraendert erhalten und wird
nicht umgedeutet.

`ChangeClass=AuditOnly`.

## Intrinsische exakte Span-Signatur

Fuer eine bestehende Pending-Visual-Projektion mit den Attack-Ereignissen

`A0, A1, ..., An`

verwendet r565 ausschliesslich diese Ereignisse von Projektionsstart bis
Projektionsende. Weder ein Attack vor `A0` noch ein Attack nach `An` darf in die
Signatur einfliessen.

### Rhythmus

Die Rhythmussignatur besteht aus:

- allen internen IOIs `Ai+1.canonical_start - Ai.canonical_start`,
- den klingenden Dauern aller Attack-Ereignisse als
  `canonical_sounding_end - canonical_start`.

Alle IOIs und klingenden Dauern werden gemeinsam mit genau einem positiven GGT
normalisiert. Dadurch bleibt die exakte relative Rhythmusform erhalten und eine
gleichmaessige Augmentation bzw. Diminution aendert die Signatur nicht.

Es gibt keine Toleranz, keinen Rundungsabstand und keine Rhythmus-Schwelle.

### Melodik

Die Melodiesignatur besteht aus:

- allen internen Pitch-Centroid-Differenzen zwischen benachbarten Attack-
  Ereignissen,
- der vollstaendigen Folge der `chord_members` innerhalb des Spans.

Absolute Tonhoehe ist nicht Bestandteil der Signatur. Eine globale
Transposition aendert den Descriptor daher nicht. Veraenderte interne
Intervallgestalt oder veraenderte Chord-Member-Struktur bleibt dagegen exakt
sichtbar.

## Suchraum dieses Takes

r565 vergleicht ausschliesslich:

`PendingVisualSpan <-> PendingVisualSpan`

und berichtet pro Projektion:

- `exact_pending_matches`,
- `same_lane_pending_matches`,
- `cross_lane_pending_matches`.

Der groessere Suchraum ueber alle passenden Attack-Windows ist ausdruecklich
noch **nicht** Bestandteil von r565. Er gehoert in einen getrennten Folgetake,
damit zwei unterschiedliche Hypothesen empirisch getrennt bleiben:

1. r564 -> r565: Effekt des entfernten Aussenkontexts,
2. r565 -> spaeterer All-Window-Audit: Effekt des Pending-only Selection Bias.

## Neue Reportprojektion

Der Report erhaelt die Summenzeile:

`PhraseNotationIntrinsicSpanSignatureAudit`

sowie pro bestehender Pending-Projektion eine Zeile:

`PhraseNotationIntrinsicSpanSignature[...]`.

Die Ausgabe markiert explizit:

- `SearchSpace pending_visual_only`,
- `OutsideContextUsed nein`,
- `AllWindowSearch nein`,
- `Similarity none`,
- `Threshold none`,
- `PositiveUnityDecision unclassified`,
- `NotationPolicyDecision unchanged_pending_visual`,
- `MutationAuthority none`.

## Regressionen

r565 fuegt zwei lokale Audit-Regressionen hinzu:

1. Eine intern identische Phrase bleibt bei globaler Transposition und
   gleichmaessiger Augmentation exakt gleich; eine veraenderte interne
   Rhythmusform bleibt verschieden.
2. Exakte Pending-Span-Treffer werden in Same-Lane und Cross-Lane getrennt
   gezaehlt, ohne daraus eine Promotion oder Writer-Autoritaet abzuleiten.

## Harte Nicht-Aenderungen

r565 aendert nicht:

- `PhraseBoundaryEvidence` oder bestehende Boundary-Rollen,
- `PhraseStructure`-Entscheidungen,
- SlurIntent-Allow/Withhold-Entscheidungen,
- `PhraseNotationIntentProjection`,
- Projektionsstart, Projektionsende oder Projektionslaenge,
- den bestehenden r564-Whole-Span-Audit,
- Writer-Reachability,
- die r421-Persistenzsperre,
- MusicXML, Render-MusicXML oder PDF,
- Pitch, Onset, Duration, Staff oder Voice,
- klingende Ereignisse,
- Pass-Reihenfolge,
- Schwellen oder Gewichtungen.

Es gibt keine SimilarityScore-, Edit-Distance-, Fuzzy-, Ornamentierungs-,
ML- oder Reparaturlogik. `structural_joint == 0` erteilt weiterhin keine
Notationsfreigabe.

Der Slur-Writer bleibt geschlossen.

## Erwartete Ausfuehrung

Der erste autoritative Lauf erfolgt mit:

```powershell
.\run.cmd --take "Etüde Nr. 5"
```

Nach gruenem Einzelprojekt folgt der Vollkorpus mit:

```powershell
.\run.cmd --all
```

Die reale Compile-, Strict-Clippy-, All-Targets-Test- und Korpusentscheidung
bleibt den bestehenden fail-closed EtuSchlopf-Gates ueberlassen.
