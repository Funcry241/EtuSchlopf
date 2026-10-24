# PhraseNotationStructuralVariationFactorizationAuditTestCompileClosure r569


## Rejected predecessor r568

r568 `PhraseNotationStructuralVariationFactorizationAudit` wurde nach erfolgreichem
Produktiv-Build und Strict-Clippy im All-Targets-Test-Compile rejected. Ursache war
ausschliesslich die fehlende Sichtbarkeit dreier bereits implementierter privater
Audit-Symbole im `#[cfg(test)]`-Modul:

- `PhraseNotationStructuralVariationFactorizationAuditRow`,
- `intrinsic_span_variation_projection`,
- `annotate_phrase_notation_structural_variation_factorization`.

r569 wird frisch auf r567 aufgebaut, uebernimmt denselben fachlichen Audit-Schnitt
und schliesst nur diese Testmodul-Imports. Es gibt keine zusaetzliche
Erkennungs-, Suchraum-, Policy- oder Musikmutation.

## Ausgangspunkt

Parent ist der vollstaendig gruen bestaetigte Stand r567
`PhraseNotationIntrinsicParallelismCorpusClosureAudit`.

Der r567-Vollkorpus umfasst 10/10 Projekte. Fuer die Phrase-Notation gilt
korpusweit:

- 74 `allow_structural_pending_visual` Projektionen,
- 74/74 bekannte intrinsische Span-Signaturen,
- 0/74 exakte Pending-gegen-Pending-Treffer,
- 0/74 exakte Pending-gegen-All-Window-Treffer,
- 0 Selection-Bias-Faelle,
- 0 Non-Pending-Exact-Evidenz,
- weiterhin 387 interne `structural_joint`s innerhalb der Pending-Projektionen.

Damit erklaeren weder der in r564 identifizierte Aussenkontext-Leak noch der
in r566 entfernte Pending-only-Auswahlraum das Ausbleiben positiver
Whole-Span-Parallelitaet. Ein Higher-Level-Unity-Take, der Exact-Matches
voraussetzt, waere auf diesem Korpus leer.

## Literaturleitplanke

Die Literatur rechtfertigt fuer den naechsten Schritt eine Zerlegung, nicht
eine neue Aehnlichkeitsentscheidung.

- McAdams & Matzkin, *Similarity, invariance, and musical variation*,
  Annals of the New York Academy of Sciences 930 (2001), DOI
  10.1111/j.1749-6632.2001.tb05725.x: musikalische Verwandtschaft kann unter
  Transformationen invariant bleiben und haengt von mehreren musikalischen
  Dimensionen ab.
- Prince, *Contributions of pitch contour, tonality, rhythm, and meter to
  melodic similarity*, Journal of Experimental Psychology: Human Perception
  and Performance 40(6) (2014), DOI 10.1037/a0038010: Pitch- und Zeitmerkmale
  leisten unterscheidbare Beitraege; Kontur und Rhythmus duerfen deshalb
  diagnostisch getrennt betrachtet werden.
- van Egmond & Povel, *Perceived similarity of exact and inexact
  transpositions*, Acta Psychologica 92(3) (1996), DOI
  10.1016/0001-6918(95)00043-7: exakte und inexacte Transpositionen sind
  unterscheidbar; Konturerhalt allein ist keine Gleichheit.
- Szymczak et al., *Grouping compositions based on similarity of music
  themes*, PLOS ONE 15(10) (2020), DOI 10.1371/journal.pone.0240443:
  Transposition sowie Augmentation/Diminution sind typische
  Motivtransformationen.

Diese Quellen geben r569 kein Schreibrecht und keine Schwelle. Sie
begrenzen nur die audit-only zu trennenden Dimensionen.

## Unveraenderte Invarianzen aus r565/r566

Die bestehende intrinsische Full-Exact-Signatur bleibt unveraendert:

- Melodik: relative Centroid-Deltas plus `chord_members`,
- Rhythmus: interne IOIs plus klingende Dauern, gemeinsam exakt ueber einen
  positiven GGT normalisiert,
- dadurch transpositionsinvariant,
- dadurch invariant gegen gleichmaessige Augmentation/Diminution,
- kein Aussenkontext,
- gleicher All-Window-Suchraum wie r566.

r569 ersetzt diese Signatur nicht.

## Neue audit-only Faktorisierung

Fuer jeden bestehenden Pending-Span wird jedes andere zusammenhaengende
Attack-Window gleicher Attack-Anzahl im Stueck wie in r566 betrachtet. Der
Source-Span selbst bleibt ausgeschlossen.

Ein Full-Exact-Treffer wird erneut beobachtet und muss mit r566
uebereinstimmen.

Daneben werden nur exakte Projektionen gezaehlt:

### `texture_only`

Erhalten bleiben:

- komplette bestehende Rhythmus-Signatur,
- exakte relative Centroid-Bewegung.

Nur die Folge `chord_members` wird weggelassen.

Ein Treffer bedeutet deshalb nicht Phrase-Gleichheit. Er zeigt nur, dass die
Akkord-/Texturgroesse die Full-Exact-Gleichheit verhindert.

### `sounding_duration_only`

Erhalten bleiben:

- interne IOI-Verhaeltnisse, separat exakt ueber ihren eigenen GGT
  normalisiert,
- exakte relative Centroid-Bewegung,
- exakte `chord_members`-Folge.

Nur die klingende Dauergestalt wird weggelassen.

### `interval_magnitude_only`

Erhalten bleiben:

- komplette bestehende Rhythmus-Signatur,
- exakte Richtungsfolge `up / same / down`,
- exakte `chord_members`-Folge.

Nur die Groesse der relativen Centroid-Intervalle wird weggelassen.

Konturgleichheit ist ausdruecklich nur Diagnose und keine motivische
Gleichheit.

### `rhythm_dimension`

Erhalten bleiben:

- exakte relative Centroid-Bewegung,
- exakte `chord_members`-Folge.

Die gesamte Rhythmusdimension wird weggelassen.

### `melody_dimension`

Erhalten bleibt:

- die komplette bestehende Rhythmus-Signatur.

Die melodische Dimension wird weggelassen.

Diese beiden groberen Projektionen dienen nur dazu festzustellen, ob der
Korpus ueberhaupt wiederkehrende reine Melodie- oder Rhythmusgestalten in
den Pending-Laengen enthaelt.

## Harte Grenzen

r569 darf nicht:

- `pending_visual` promoten,
- Higher-Level-Unity entscheiden,
- PhraseMark oder LegatoSlur klassifizieren,
- einen SimilarityScore bilden,
- Gewichte einfuehren,
- Schwellen einfuehren,
- Edit-Distance verwenden,
- Ornamentnoten entfernen oder hinzufuegen,
- Fragmentierung, Inversion oder Retrograd bereits als Gleichheit
  behandeln,
- Writer-Reachability oeffnen,
- MusicXML oder klingende Ereignisse mutieren.

`StructuralVariationDecision` bleibt `diagnostic_only`.

## Erwartete Entscheidung nach dem Korpus

Wenn selbst die exakten Projektionen kaum Treffer liefern, reicht
dimensionale Faktorisierung nicht aus. Dann muss der naechste Audit gezielt
eine einzelne strukturerhaltende Transformation untersuchen, nicht mehrere
gleichzeitig.

Wenn eine Projektion dagegen reichlich und musikalisch lokalisierbar trifft,
wird diese eine Transformationsklasse im Folgetake vertieft. Auch dann
entsteht noch keine NotationPolicy.
