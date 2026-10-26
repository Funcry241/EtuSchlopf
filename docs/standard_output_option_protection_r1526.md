# Preserve the existing backup on option-bearing project runs

r1526 / StandardOutputOptionProtection / KodexOnly / parent r1525.

The existing four-file public-artifact transaction remains enabled for ordinary
single-project calls with --jobs, --layout-pages, --top, --reference-wav,
--writer-trace, --use-corpus-cache and --lenient-partial-voices. These options
do not change the four output paths. Parsing and musical meaning stay with
the child CLI. The existing commit validation and restart recovery are unchanged.

Custom output paths, partial-output modes, corpus runs and unknown options
remain outside this exact four-file transaction contract. This take does not
claim to protect them. No second recovery owner is introduced and the existing
recovery source remains byte-identical.

The new Stage-A input-admission definition is carried together with this
control-plane improvement. It is required by the parent gate before the
prepared runtime input-admission implementation may be installed. That prototype
is not part of this take. It still needs a later accepted-parent implementation
contract and the full runtime build/test/negative/positive evidence.

Tests prove option eligibility, refusal of nonstandard output sets, and the
actual production finalization function restoring all four previous files after
a failed child run. No music, source decoding, input naming or output naming is
changed by this take. The launcher trust anchor is rebound to the final bytes;
its rule and all gate behavior remain unchanged.
