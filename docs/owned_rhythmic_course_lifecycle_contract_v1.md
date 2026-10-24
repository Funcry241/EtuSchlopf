# Owned rhythmic course: constructive lifetime rule

Contract: owned_rhythmic_course_lifecycle_v1
Status: prospective musical policy definition; no productive issuer added.
ChangeClass: KodexOnly.
Parent: accepted r1348, version 0.10.1839.

## Decision, not melody reconstruction

A rhythmic course is an explicitly authored notational responsibility. It is not
a claim about the original composition's melodic voices. The constructive rule
below decides what the arrangement is to represent, before XML is written. It
does not recover continuity from nearby pitches, existing voice labels, interval
coloring, equality of presentation roles or technical renderer requirements.

This policy supplies the missing lifetime decision required by
canonical_rest_context_v1. That contract, role_occurrence_binding_v1 and the
accepted renderer-capacity class remain unchanged. This is a new musical policy,
not evidence that r1348 already contains its runtime authority. A productive
adoption must be admitted as a musical decision change and prove preservation.
Accepting documentation adds no mint capability or writer allowlist entry.

## Rule P: expressly created primary rhythmic course

When the arrangement generator expressly creates a primary rhythmic presentation
course in its planned notation domain, it commits to complete rhythmic coverage
of the specified domain. The domain is the exact planned interval of the generated
part, with its canonical metric and explicit presentation-role/staff segments.
It is fixed before serializing events or searching for gaps.

The generator's primary-course creation is the positive decision that licenses
coverage. The duration of a measure, the presence/absence of notes or a staff
number alone is not that decision. The policy does not force an existing or
imported score to have one musical voice per staff. It applies to courses the
generator deliberately creates; their population comes from the explicit
notation-domain plan, never a fixed voice count or renderer slot budget.

For the MIDI piano generator, the intended adoption point is the creation of its
planned primary staff presentations before build_musicxml/append_staff_voice
writes XML. The admitted decision must explicitly carry the primary course's
role and staff, the complete planned part interval and exact source membership.
The existing function currently accepts staff/voice/stem values and has no such
course object; a future implementation must create that object at the musical
decision boundary rather than wrap a parsed XML label with a new ID.

The course is born at the domain's beginning and ends at its end. Leading,
internal and trailing unrepresented intervals remain within its licensed coverage.
It therefore survives silent intervals without needing a note on either side.
Its explicit boundary, not an inferred last note or a timeout, ends responsibility.
Rhythmically empty primary courses are covered by the same rule.

This is an explicit editorial rule for generated primary notation. It must not
retroactively reinterpret a retained Secondary/Auxiliary source as Primary.
Pitched source roles remain under their existing canonical authority. Any
divergence between a source's current role and the course's role/staff segment
requires an explicit, source-bound owner transition or remains Unknown.

## Rule B: explicitly created bounded branch

A representation owner that deliberately separates material into an independent
auxiliary or secondary course declares its exact support as part of that action.
The branch is born at the first boundary of this explicitly planned support and
ends at its last boundary. Disconnected support components remain disconnected;
their bounding hull does not license silence between them.

Support is the authorized represented interval, not a guess from MIDI note-off,
the next note, a label's first/last event or the enclosing measure. No lead-in,
trailing or between-attack rest is licensed outside that support. Extending support
requires a separate positive coverage-extension decision under the same policy;
gap detection cannot make that decision. A branch may deliberately carry a
licensed rest interval if its creator explicitly included it in the support.

Each branch receives its own creation identity and independently authorized role,
plus a source/fragment ancestry relation and its parent-course relation where one
actually exists. Branches do not inherit an unbounded primary lifetime. Role
equality cannot fuse branches, and overlapping source intervals cannot justify
creating or discarding sounding material. Source membership is transported from
the actual separation action, never reconstructed from XML geometry.

The existing CanonicalFinalChord AuxiliaryLane action is a possible branch
creation boundary. Its current serialization_run_id, allocated per staff within
planning, is not this musical branch identity. Its canonical action interval can
support a declared branch; that does not authorize filling an entire measure.

## Rule C: continued obligation and explicit transitions

An existing course continues through normalization, metric splitting, tied
rematerialization and channel relabeling with the same lifetime and ancestry.
Measure, page or system boundaries do not terminate it. Staff transfer changes
presentation only when its owner explicitly transports the appropriate segment;
it neither creates an unrelated course nor fills the other staff with silence.

Restification of a represented occurrence changes its disposition under its
existing admitted owner. The occurrence retains its course and exact represented
support; its new rest responsibility needs an explicit owner transition bound to
that support. A hidden pitched event still occupies sounding time. An omission,
a changed duration or an independent new attack cannot silently extend coverage.

The creation decision fixes a course's lifetime. An early termination or coverage
extension is a new musical decision and cannot be issued merely because no later
sound is observed. Imported/existing unbound rests retain their original protected
obligation and Unknown context until actual origin authority is positively bound.
This rule cannot retroactively attach them to a generated primary course by staff,
role, voice label, temporal coincidence or majority vote.

For an admitted course and complete, source-bound inventory, the rest obligation
is its licensed coverage minus already represented interval union, exactly as
defined in canonical_rest_context_v1. Positive sounding conflicts, physical chord
hosts, ties, cursor operations and supported metric realization retain separate
proof obligations. A course license alone does not permit a typed XML write.

## Executable decision table

| Creation/transition | Course lifetime | Gap responsibility |
| --- | --- | --- |
| Explicit primary-course generation | Exact planned part domain | Every licensed unrepresented component |
| Explicit bounded branch | Exact declared support components | Only silence explicitly inside that support |
| Representation split or channel rename | Unchanged parent coverage | Conserved, never enlarged |
| Explicit support extension | Old coverage plus declared extension | Requires independently admitted musical decision |
| End of declared coverage | Ends there | None beyond the boundary |
| XML grouping, pitch adjacency, sounding absence | No creation decision | Unknown; grants no new rest |
| Imported unbound rest | Existing obligation retained | Unknown context; no automatic primary adoption |

## Actual dependency order

1. The genesis/arrangement owner creates the planned notation domain and explicit
   primary course decisions before emission. These prospective decisions supply
   lifetime and rest-role authority, rather than importing it from later labels.
2. Every material creator captures course membership, source ancestry and exact
   creation identity. Existing normalization carries the obligations it receives.
3. Canonical pitched role/chord decisions retain their current prerequisites.
   The owner must explicitly transport, split or branch affected course segments.
   The policy does not equate a temporal lane with a musical course to break a cycle.
4. The complete final context is frozen; only its admitted obligations are realized
   by the existing structural owner. No new generic repair layer is introduced.
5. All later writers and role/run-sensitive consumers preserve the actual course
   relation separately from technical channels, then bounded channel reuse may be
   evaluated under the unchanged capacity class.

Any phase that cannot explain a created rest, source transition or branch remains
Unknown for the affected new mutation. Implementing only the first constructor
would not close the pipeline. The current legacy producers cannot simply receive
permission to keep selecting rest targets by label under a new type name.

## Why these rules are general

Complete coverage for an explicitly generated primary course is a chosen
notation obligation, not a corpus-conditioned silence heuristic. A deliberately
bounded branch has no authority outside its declared support. Exact continuation
cannot change the lifetime that the original owner chose. These premises are
independent of project identity, measure labels, pitch/register, candidate count,
source ID values and consumer capacity.

Renaming, identity bijections, label changes and metric time scaling cannot change
the chosen coverage. Staff permutation must transport the corresponding role and
presentation domain. Independent courses remain additive; duplicate observations
are idempotent while newly created branches remain distinct. Two equal-looking
courses are not a common obligation and cannot be merged to save a voice slot.

## Admission and limits

Later runtime adoption must consume the unchanged accepted
selector_derivation_contract_unbound_rest_context.md and separately state its
repair/decision scope. Full actual-owner tests must cover primary birth before XML,
silent-course coverage, disconnected branches, explicit extension/termination,
restification, source-role divergence, imported Unknowns, cross-staff transitions,
exact split ancestry and final rest/host/tie/cursor readback. Boundary tests must
reject label-based reconstructions, forged/stale declarations and implicit hull
extensions. No corpus witness can become a normative fixture.

The executable model in owned_rhythmic_course_lifecycle_foundation_r1349.md models
these constructive coverage choices. It does not authenticate product capabilities
or prove real musical consumer behavior. This policy's completeness and readability
require actual pipeline and corpus evaluation; no channel feasibility, unforced PDF
success, authored melodic recovery or full role-binding closure is claimed.
