# MIDI seed module identity cutover — r227

r227 is the first stage of the historical FLP owner rename. It intentionally keeps the physical source file `src/flp.rs` for one transition while changing the Rust module identity to `midi_seed` via an explicit `#[path = "flp.rs"]` binding. All direct module references are changed mechanically from `flp` to `midi_seed`.

This staging is required by the existing Take-CrossFile-Interface-Closure: deleting `src/flp.rs` in the same step makes the interface checker correctly treat every struct field defined there as potentially removed before cross-file call sites have been closed. r227 therefore changes no struct, function, field, solver, writer, renderer, MusicXML, layout or musical semantics.

After a real green r227, the physical file relocation can be performed as its own second step, with the interface owner already named `midi_seed`. No compatibility wrapper, duplicate module, `include!` shell or parallel implementation is introduced.
