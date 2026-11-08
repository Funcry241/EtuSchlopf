# r1662: Exact allocation evaluation reuse and phase measurements

Parent: accepted r1661. Change class: RefactorOnly.

The native E7 single run accepted r1661 but required 24:57 (initial render
transaction 22:13). A local reproduction uses the authentic MIDI and WAV and
reaches the same allocation scope, 08ca038bf6ff88ef. Self-sampling the unchanged
strict optimized parent identifies repeated full canonical projections and
class/measure closure scans as substantial costs. This is not a full-corpus
measurement and does not establish a native speedup for this candidate.

## Exact reuse boundaries

1. Binding-source XML and parsed measurements share one immutable lifetime in
   a private value. Every projection still uses the ordinary owner binder,
   ordinary byte projector, receipts and preservation guards. The value lives
   inside one immutable canonical/actual-owner scope; it cannot be paired with
   a different XML argument. Failed parses remain failures.
2. Class/measure incidence is indexed once per search. Guest evacuation groups
   are connected components in the induced bipartite graph. Reachability is
   the least fixed point of the former repeated whole-score scans. Guest
   membership, minimum seed order and original member order are preserved;
   staff is not an extra filter. Exhaustive guest subsets compare exact output
   with the previous algorithm, including absent classes and duplicates.
3. The two search phases share only exact compatibility refusals. A key is the
   complete partition and complete role map. Its baseline, original partition,
   canonical source, class map and capture are immutable within that search.
   Successors and phase-specific visited sets remain independent. A memo hit
   substitutes the same false predicate result; it does not prune successors.
   The memo is dropped at search end. No hashes authorize reuse.

The mathematical identities are referential substitution f(x)=f(x), and
reachability as the least fixed point of an immutable incidence relation.
There is no search budget, new admission, removed guard, notation rule change,
corpus identity selector, or downstream repair.

## Necessary unchanged-edge preservation before projection

An admitted result must preserve every occurrence identity, physical chord
relation and non-voice byte. Thus its beam states, staff and onset already equal
the baseline, and the proposed labels determine its written connections.
With distinct physical-head onsets in each proposed lane, ordinal changes cannot
alter the ordered readback. The existing written-connection function can then
compute the necessary edge relation by label substitution. If its edges between
unmoved sources differ, the ordinary unchanged-edge guard must fail.

Missing proposed labels or equal physical-head onsets return unknown and use
the ordinary complete path. A positive preflight never grants admission. This
is the contrapositive of an existing mandatory guard; changing any premise in
an actual projection would itself violate another unchanged guard. Synthetic
binary label assignments compare the certificate with the full readback under
both ordinal orders and explicitly exercise the ambiguous-onset fallback.

## Diagnostics and acceptance

Audit timing spans report allocation, graph construction, greedy assignment,
semantic signature, overflow recolouring, cross-role search and projection.
Progress reports visited states. Times and counters are never read by decisions.
Nested durations are inclusive and must not be summed as wall time.

Acceptance requires unchanged MusicXML, Master and Render across the native
full corpus. Local synthetic and integration tests are additional evidence,
not a substitute for that corpus run. Native E7 timing must be compared with
accepted r1661 before claiming the regression resolved.

## Same owner proof on the preserved readback

The same necessarily preserved written-edge relation also determines which
written pairs would be new. Every new pair must pass the existing authentic
original-owner cohort proof at its beam level. That exact proof is one shared
function called both on the necessary readback before projection and by the
ordinary complete transition checker afterwards. Failure on the necessary
readback is an early refusal only. Unknown onset order still reaches the full
path. This neither adds a source rule nor admits any proposal without the full
projector and all existing guards. The source class and repair scope above stay
unchanged. It avoids materializing proposals already known to fail the same
mandatory owner proof.
