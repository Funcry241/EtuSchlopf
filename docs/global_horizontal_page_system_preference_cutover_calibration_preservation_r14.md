# GlobalHorizontalPageSystemPreferenceCutoverCalibrationPreservationClosure — v0.10.1839 r14

## Rejection basis

The rejected r13 candidate never installed. `Semantic-Capability-Diff` classified the removed
`min_end#1` capability in `horizontal_layout_global_page_system_shadow.rs` as a protected
`CalibrationThreshold`. The policy is intentionally unchanged. r14 therefore preserves the
exact r12 semantic fingerprint of that calibration capability.

## Productive authority change

The productive Page/System architecture remains the intended r13 cutover:

`final notation facts`
→ `read-only global system demand/capacity evidence`
→ `balance-first FullFrontier`
→ `PreferenceSelection`
→ `one joint Page/System candidate`
→ `GlobalPageSystemReflow plan authority`
→ `one existing materializer`.

The old productive system-only promotion and its decision-owner remain retired. No writer,
materializer, width policy, repair layer or etude/measure-specific rule is introduced.

## Preserved calibration capability

The legacy page-slot DP that contains the protected `min_end..=max_end` calibration surface is
retained fingerprint-identically as `observe_legacy_page_slot_calibration()`. It is executed
only as a read-only observation. Its `Result` is converted to optional evidence:
`LegacyPageSlotCalibrationCandidate ja/nein`.

Crucially, that result is not used to:

- select the productive PageCount;
- reject a `PreferenceSelection`;
- constrain page-slot lengths;
- restore the old +/-1 page-slot corridor;
- restore the old exact PageCount.

A missing legacy calibration candidate is therefore evidence, not a productive failure. This
preserves the protected calibration contract while demoting its former authority.

## Productive hard invariants

The selected FullFrontier candidate must still satisfy:

1. exact final-fact / plan / frontier slot binding;
2. exact membership in the feasible balance-first FullFrontier;
3. selected page count inside the tested preference range and not above `PreferredPages`;
4. exact non-empty Page/System coverage of all slots;
5. zero forced-over-capacity singletons;
6. every system load at or below accepted system capacity;
7. every system length at or below the structural candidate maximum;
8. systems per page at or below the observed vertical ceiling;
9. no increase in total systems or singletons;
10. page and system breaks mutually exclusive;
11. candidate diff exactly bound to the audit;
12. post-promotion decision idempotence.

## Deliberately unchanged

- RendererAuto-only width policy;
- explicit-width retirement;
- One Plan / One Materializer;
- final-notation fact basis;
- security and Take gates;
- ThinFirst remains counterfactual evidence only;
- no repair/recovery/fallback layer.

## Validation

Commit through a known-green project:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Then run the corpus without `--take`:

```powershell
.\run.cmd --all
```
