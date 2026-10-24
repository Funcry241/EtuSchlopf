# PhraseNotationIntrinsicParallelismCorpusClosureAudit r567

## Autoritativer Parent

r567 basiert direkt auf dem fuer Etuede Nr. 5 real dunkelgruen bestaetigten
r566 `PhraseNotationIntrinsicAllWindowParallelismAudit`.

Der reale r566-Einzellauf bestaetigte:

- `PendingVisualRows 15`,
- `RowsWithExactAllWindowMatches 0`,
- `RowsWithExactNonPendingWindowMatches 0`,
- Build und Strict Clippy gruen,
- keine Musikmutation,
- Writer weiterhin geschlossen.

Damit ist fuer Etuede Nr. 5 weder der in r564 identifizierte Aussenkontext der
verketteten `JointParallelSignature` noch der in r566 geoeffnete Pending-only-
Suchraum die Ursache des Nullbefunds. Dieser Einzelbefund darf aber nicht als
Korpusurteil verallgemeinert werden.

`ChangeClass=AuditOnly`.

## Zweck

r567 fuegt keine neue musikalische Erkennung hinzu. Der Take schliesst die drei
bereits vorhandenen Exact-Messungen lediglich in einer gemeinsamen,
read-only Reportprojektion zusammen:

1. r564: legacy Whole-Span-Folge lokaler `JointParallelSignature`s,
2. r565: intrinsische Exact-Signatur im Suchraum Pending gegen Pending,
3. r566: dieselbe intrinsische Exact-Signatur im Suchraum Pending gegen alle
   gleich langen True-Attack-Windows desselben Stuecks.

Damit kann der nachfolgende Vollkorpus die beiden urspruenglichen Hypothesen
getrennt und gemeinsam auswerten, ohne die Erkennung erneut zu veraendern.

## Identitaetsbindung

Die drei Audit-Reihen werden nur dann miteinander verglichen, wenn folgende
Projektionsidentitaet uebereinstimmt:

- Lane `(part, staff, voice)`,
- `projection_start.canonical_start`,
- `projection_end.canonical_start`,
- `projection_attack_count`.

Abweichungen werden als `RowAlignmentMismatches` berichtet. Die Closure
repariert oder reordnet keine Zeile.

## Closure-Klassen

Pro Projekt werden unter anderem berichtet:

- Legacy-Exact-Zeilen,
- Intrinsic-Pending-Exact-Zeilen,
- Intrinsic-All-Window-Exact-Zeilen,
- Legacy-only Exact,
- Intrinsic-only Exact,
- Exact in beiden Modellen,
- Exact in keinem der beiden Whole-Span-Modelle,
- Zeilen, in denen All-Window mehr Treffer als Pending-only findet,
- Zeilen mit exakter Evidenz ausschliesslich ausserhalb bestehender
  Pending-Visual-Spans,
- Verteilung der All-Window-Treffer `zero / one / multiple`,
- Cross-Lane- sowie kombinierte Same-and-Cross-Lane-Treffer,
- Gesamtzahl der All-Window- und Non-Pending-Treffer.

Die vorhandenen r566-Match-Samples werden ausserdem nach Start- und Endrolle
gezaehlt. Diese Rollen bleiben rein diagnostisch und sind weder Bestandteil
der intrinsischen Signatur noch eine Freigabebedingung.

## Was r567 ausdruecklich nicht tut

r567 aendert nicht:

- die r565 `IntrinsicSpanParallelSignature`,
- den r566 All-Window-Suchraum,
- `PhraseBoundaryEvidence`,
- Boundary-Rollen,
- `PhraseStructure`,
- SlurIntent,
- `allow_structural_pending_visual`,
- `PhraseNotationIntentProjection`,
- NotationPolicy,
- Writer-Reachability,
- r421,
- MusicXML, Render-MusicXML oder PDF,
- Pitch, Onset, Duration, Staff oder Voice,
- klingende Ereignisse,
- Pass-Reihenfolge,
- Schwellen, Gewichte oder Scores.

Es gibt weiterhin keinen SimilarityScore, keine Edit-Distance, kein Fuzzy
Matching, keine Ornamentierungs-Toleranz, kein ML und keine Reparaturschicht.

Die Closure-Ausgabe markiert deshalb explizit:

- `RecognitionChanged nein`,
- `SignatureChanged nein`,
- `SearchSpaceChanged nein`,
- `CorpusDecision deferred_until_all_projects`,
- `StructuralVariationDecision not_taken`,
- `PositiveUnityDecision unclassified`,
- `NotationPolicyDecision unchanged_pending_visual`,
- `MutationAuthority none`.

## Beweisfrage des Vollkorpus

Nach gruenem Einzellauf beantwortet `run.cmd --all` die naechste fachliche
Frage:

> Liefert exakte intrinsische Parallelitaet corpusweit genuegend positive
> Higher-Level-Unity-Evidenz, oder bleibt sie so selten, dass als naechster
> Schritt gezielt strukturwahrende musikalische Variation untersucht werden
> muss?

r567 selbst beantwortet diese Frage nicht vorzeitig.

## Erwartete Ausfuehrung

Zuerst:

```powershell
.\run.cmd --take "Etüde Nr. 5"
```

Nach gruenem Take:

```powershell
.\run.cmd --all
```

Erst der vollstaendige 10-Etueden-Korpus entscheidet ueber den naechsten
fachlichen Audit.
