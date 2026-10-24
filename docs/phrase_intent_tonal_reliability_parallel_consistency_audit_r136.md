# Phrase Intent Tonal Reliability Parallel Consistency Audit r136

## Estimated plan position

Take 7/8 (estimated) on the compressed phrasing path.

## Parent

Accepted DARK-GREEN parent: EtuSchlopf v0.10.1839 r135
`PhraseTonalClosureStructureIntentAuditSanitizerTokenRecovery`.

The full 10-etude hard stop after r135 is green and shows that the qualitative phrase structure is
conservative enough to continue, but two release gaps remain before any productive slur writer:

1. a score-wide tonal best profile may have a weak best-vs-second evidence margin while still
   producing raw tonal closure cues;
2. exact piece-internal parallel patterns may receive divergent qualitative phrase roles without an
   explicit named contextual explanation.

r136 closes only those release gaps. It adds no new music theory, no scalarization and no writer.

## Tonal reliability hardening

The existing score-wide tonal owner already uses a profile-green evidence-margin criterion of 48.
r136 does not alter that owner, its 24 profiles, weights or scoring algorithm. The phrase audit
continues to consume only the r133 read-only best/second projection and applies the same existing
profile-green margin criterion as an authorization gate.

Raw tonal observations remain visible even when the profile is unreliable:

- tonic/dominant presence;
- bass arrival and dominant-to-tonic motion;
- leading-tone resolution and unresolved leading tone;
- raw closure support and raw anti-closure.

Only the derived authorization fields are gated:

- `ClosureAuthorized`
- `AntiClosureAuthorized`

`closure_supported` and tonal `conflict` phrase roles may use only authorized tonal evidence.
Therefore low-margin tonal context can inform diagnosis but cannot create PhraseSpan or SlurIntent
write eligibility.

This is an authorization hardening step, not a new tonal score or threshold family.

## Exact parallel-role explanation

The existing exact piece-internal parallel signatures remain unchanged. r136 does not introduce a
similarity score or fuzzy-match threshold.

For each exact repeated group whose qualitative phrase roles diverge, the audit now records which
already-existing role-driving context family differs:

- `tonal_context`
- `metric_structural_context`
- `continuity_context`
- `compound_context`
- `unexplained`

The explanation is derived only from the existing qualitative role inputs:

- authorized tonal closure / anti-closure;
- structural closure gate / local structural joint;
- continuation support.

No majority vote is introduced. Divergence may be musically valid; it must only be explainable.

## SlurIntent hardening

A closure-ending local group is additionally withheld if its ending boundary belongs to an exact
parallel group with `unexplained` role divergence.

New withhold reason:

`unexplained_parallel_role_divergence`

Writer precondition for the later productive take:

`DivergentUnexplained = 0`

This does not force exact parallel passages to share one phrase role. It only prevents unexplained
inconsistency from gaining notation authority.

## Hard safety contract

r136 remains AuditOnly:

- no AnchorResolver;
- no MusicXML slur write;
- no render/master/sounding mutation;
- no PhraseConfidence;
- no probability;
- no weighted scalarization;
- no local-key claim;
- no Roman-numeral or cadence classifier;
- no Security rule change;
- no external corpus, model or learned weight.

Default remains `no slur`.

## Known corpus implication before the real Windows gate

The r135 full corpus contains exactly one score-wide tonal profile below the existing owner margin
criterion: Etude 1, margin 5. Its raw tonal observations remain present in r136, but they lose phrase
closure authorization. All other known corpus profiles are at or above the existing criterion.

Therefore the static minimum consequences are:

- raw tonal closure support remains 941 corpus-wide;
- authorized raw tonal closure support is at most 897 because Etude 1 contributes 44 raw closure
  positions under an unreliable profile;
- the four r135 `closure_supported` boundaries in Etude 1 cannot remain tonal closures;
- the 84 r135 closed level-1 spans can therefore be at most 80 before any further contextual
  reclassification;
- the three r135 Etude-1 allowed SlurIntents cannot remain authorized;
- exact new parallel-context reason counts are intentionally left to the real corpus run rather than
  inferred from aggregate reports.

## Required hard stop after r136

After a green single-project gate, run the full corpus.

Before Take 8/8 is allowed, require:

- 10/10 projects green;
- `DivergentUnexplained = 0` corpus-wide;
- no SlurIntent authorized from an unreliable tonal profile;
- all raw tonal evidence still visible;
- no unexplained parallel divergence granted SlurIntent;
- no mutation and no writer capability.

Only then may Take 8/8 introduce AnchorResolver and a conservative render-only slur writer.
