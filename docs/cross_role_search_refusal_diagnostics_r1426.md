# Cross-role search refusal diagnostics

Parent: accepted r1425. RefactorOnly. No new selector or admission rule.

The previous allocator exported only a final no-partition verdict. This change
records each existing refusal branch without changing its result, traversal order,
conflict relation, guest/host scope, beam witness or projection guard. The original
Option witness API remains an `.ok()` view of the same branches with reason labels.
The diagnostic result is never used to admit a class or alter an assignment.

The existing console export carries FrozenChannelSharingDiagnostic lines, grouped
by the canonical semantic fingerprint (diagnostic correlation only, never a selector).
The summary counts predicate evaluations, not distinct events or root causes.
Initial-channel rows list class counts, eligible class counts, protected resource
kinds, first failing source/fragment identities and reason counts per class.
Search counters distinguish evidence, protected resources, roles, staff, conflict
edges, explored assignments, capacity and each full-partition guard failure.
Only the first detailed witness per guard reason is retained; all occurrences
contribute to its saturating counter. This bound never limits search or admission.
Missing evidence remains missing. Output musical bytes and assignments must match
r1425. This take diagnoses the remaining 15 import losses; it does not repair them.

Synthetic controls cover empty eligibility, missing class evidence, real conflict
edges, failed baseline projection, successful real channel search and resetting
counters between independent search calls. Existing positive and negative beam,
container and complete-partition controls remain in force.
