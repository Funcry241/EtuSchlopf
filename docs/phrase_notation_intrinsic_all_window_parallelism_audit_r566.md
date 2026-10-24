# PhraseNotationIntrinsicAllWindowParallelismAudit r566

## Ausgangspunkt

r566 basiert direkt auf dem real installierten und fuer Etuede Nr. 5 dunkelgruen bestaetigten
r565 `PhraseNotationIntrinsicSpanSignatureAudit`.

r565 hat die in r564 verwendete verkettete `JointParallelSignature` fuer einen neuen,
rein intrinsischen Vergleich bewusst nicht weiterverwendet. Die neue exakte Span-Signatur benutzt
ausschliesslich Attack-Ereignisse vom Projection-Start bis zum Projection-Ende:

- interne IOIs und klingende Dauern, gemeinsam ueber einen exakten positiven GGT normalisiert,
- interne relative Pitch-Centroid-Differenzen,
- die Folge der Chord-Member-Anzahlen.

Damit gelangen weder `previous` noch `following` ausserhalb des Spans in diese Signatur.
Im realen r565-Lauf von Etuede Nr. 5 waren alle 15 Pending-Visual-Spans intrinsisch beschreibbar,
aber keiner hatte einen exakten Match unter den anderen Pending-Visual-Spans.

Dieser Nullbefund beweist noch nicht, dass im Stueck keine exakte intrinsische Parallelitaet
existiert. Der r565-Suchraum bestand absichtlich nur aus bereits als `pending_visual` selektierten
Spans. Eine strukturell gleiche Stelle mit anderer Boundary-Rolle konnte deshalb nicht gefunden
werden.

## Aenderung

r566 behaelt die r565-Signatur byte-semantisch unveraendert und erweitert ausschliesslich den
read-only Vergleichsraum.

Fuer jeden bestehenden Pending-Visual-Span werden alle zusammenhaengenden True-Attack-Fenster
derselben Attack-Anzahl in allen vorhandenen `(part, staff, voice)`-Lanes desselben Stuecks
betrachtet. Das Quellfenster selbst wird ausgeschlossen.

Ein Treffer liegt nur vor, wenn die vorhandene r565-`IntrinsicSpanParallelSignature` exakt gleich
ist. Es gibt weiterhin:

- keinen SimilarityScore,
- keine Edit-Distance,
- keine Schwelle,
- kein Fuzzy Matching,
- keine Ornamentierungs-Toleranz,
- keine Gewichtung.

Der Audit berichtet getrennt:

- exakte Treffer ueber alle Fenster,
- Same-Lane-Treffer,
- Cross-Lane-Treffer,
- exakte Treffer, deren gefundenes Fenster selbst kein bestehender Pending-Visual-Span ist.

Fuer eine begrenzte Zahl gefundener Fenster werden zusaetzlich rein diagnostisch die bereits
existierenden Boundary-Rollen unmittelbar vor dem Fensterstart und am Fensterende sowie
`matched_window_is_pending` ausgegeben. Diese Rollen sind kein Bestandteil der intrinsischen
Signatur und besitzen keinerlei Selektions- oder Schreibrecht.

## Beweisfrage

r566 beantwortet genau eine neue Frage:

> Verbirgt der bisherige Pending-only-Vergleich exakte intrinsische Parallelstellen, weil passende
> Attack-Windows mit anderer Boundary-Rolle gar nicht im Vergleichsraum liegen?

Damit wird der Selection Bias getrennt vom bereits in r565 isolierten Aussenkontextproblem
gemessen.

## Nicht enthalten

- keine Aenderung von PhraseBoundary-Rollen,
- keine Aenderung von SlurIntent,
- keine Promotion von `allow_structural_pending_visual`,
- keine NotationPolicy,
- kein PhraseMark- oder LegatoSlur-Urteil,
- kein Writer-Reopening,
- keine MusicXML-/Render-/PDF-Mutation,
- keine neue musikalische Pass-Reihenfolge,
- keine Laengen- oder Taktzahlgrenze,
- keine Reparaturschicht.

`ChangeClass=AuditOnly`.

## Erwarteter naechster Schritt

Nach einem gruenen Einzellauf auf Etuede Nr. 5 folgt der Vollkorpus. Erst der anschliessende
Korpusabschluss darf beurteilen, ob exakte intrinsische Parallelitaet ausreichend Evidenz fuer
einen spaeteren Higher-Level-Unity-Audit liefert oder ob gezielt strukturwahrende musikalische
Variation untersucht werden muss.
