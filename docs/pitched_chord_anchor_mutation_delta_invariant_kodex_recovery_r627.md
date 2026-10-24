# r627 PitchedChordAnchorMutationDeltaInvariantKodexRecovery

Stand: v0.10.1839 r627  
Parent: r625 `ChordRhythmicCohortAtomicStaccatoClosureWriterReachabilityRecovery`  
ChangeClass: `MusicMutation`

## Ausgangspunkt

r625 ist real GREEN und schliesst die externe Notationsimport-Chord-Kohortenregression in Etuede 2/Takt 13. Die anschliessende Tiefenanalyse identifizierte `apply_structural_same_pitch_tie_merge_render_write` als zweite aktive Anchor-Mutationsgefahr: der bestehende Writer verlaengert den ersten Anker einer geschlossenen Zwei-Noten-Tie-Kette und loescht den zweiten. Unmittelbare `<chord/>`-Member gehoeren jedoch semantisch zum jeweiligen Anker.

r626 implementierte die richtige fail-closed Kandidatenverengung, wurde aber vor Installation vom `NO-NEW-REPAIR-AUTHORITY KODEX GATE` rejected. Ursache war ausschliesslich der neue Regressionstest im produktiven Pipeline-Modul: der Test rief `apply_structural_same_pitch_tie_merge_render_write` direkt auf und wurde dadurch als neuer Writer-Caller ausserhalb der `StructuralWriteAuthority` klassifiziert. r626 besitzt keine Authority.

## r627-Vertrag

r627 setzt die fachliche Regel direkt auf r625 neu auf und trennt Decision von Materialization:

- `same_pitch_tie_merge_anchor_pair_has_immediate_chord_dependency(...)` ist eine pure Decision-Funktion;
- sie verwendet die bestehende Chord-Anchor-Erkennung `note_is_followed_by_chord_note`;
- besitzt erster oder zweiter Tie-Anker unmittelbar abhaengige `<chord/>`-Member, wird der Kandidat vor jeder Mutation verworfen;
- sind beide Anker chord-unabhaengig, bleibt der bestehende SamePitchTieMerge-Writerpfad unveraendert;
- der Regressionstest prueft nur die Decision-Funktion fuer none/first/second/both und ruft keinen produktiven Writer auf.

## Architekturgrenze

Die Regel bleibt ein Transaction-/Delta-Invariant:

`anchor mutation/deletion requires complete ownership of immediate chord dependents`

Da der bestehende SamePitchTieMerge keinen chord-aware Kohortenplan besitzt, ist bei Chord-Abhaengigkeit ausschliesslich `no write` zulaessig. Es wird keine neue Chord-Tie-Merge-Technik erfunden.

MusicXML-seitig bleibt die bestehende Formatwahrheit unveraendert: kuerzere Chord-Member sind legal; nur laengere Member als ihr Anker sind unzulaessig. r627 fuehrt keine globale `ChordMember.duration == Anchor.duration`-Regel ein.

## Writer-Reachability / KODEX

r627 fuegt keinen Writer-Endpunkt hinzu, ruft im neuen Test keinen Writer auf und aendert keine StructuralWriteAuthority-Schnittstelle. Der produktive Writer-Callgraph bleibt derselbe wie in r625; nur die Kandidatenmenge des bestehenden SamePitchTieMerge wird streng verengt.

## Regression

Der eine neue Unit-Test prueft die pure Decision-Grenze mit vier Faellen:

1. kein Chord-Member -> Guard false;
2. Chord-Member am ersten Anker -> Guard true;
3. Chord-Member am zweiten Anker -> Guard true;
4. Chord-Member an beiden Ankern -> Guard true.

Erwarteter integrierter Testbestand bei Green: 1235/1235.
