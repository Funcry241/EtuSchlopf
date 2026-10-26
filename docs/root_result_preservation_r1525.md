# Root result preservation

r1525 / RootResultPreservation / KodexOnly / parent r1524.

Root hygiene must not infer that a result is disposable because its project is
not selected or its input pair is currently incomplete. Startup and post-run
cleanup preserve existing MusicXML, result reports, strict-Clippy reports,
inventory outputs and PDFs regardless of project selection. MIDI inputs using
the long extension are explicitly protected alongside `.mid` and WAV inputs.
Temporary-residue handling remains active.

This changes artifact cleanup only. It does not grant a writer, change musical
selection, change source decoding, rename output files or claim that all failure
paths now preserve results. In particular, destructive preparation in the direct
pipeline and batch, overlapping explicit input/output paths, incomplete batch
discovery and optional-WAV behavior remain separate open findings.

Two synthetic tests exercise startup and finalization cleanup for missing,
batch and visual-preflight selections, byte preservation of unrelated and
incomplete-project outputs, and continued cleanup of temporary residue.
