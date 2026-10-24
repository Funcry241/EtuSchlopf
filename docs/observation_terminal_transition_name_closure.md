# Observation Terminal Transition Name Closure

Version: 0.10.1723
Revision: r03
Class: RefactorOnly

This take removes an ambiguity exposed by the semantic capability call-graph scanner.

`SourceStateOwner::complete()` used a generic method name that collided with an existing Security-domain `complete()` function in the visual-preflight finalization path. The scanner therefore conservatively connected the launcher-side observation call with a real writer-reachable Security function.

The shared observation terminal transitions are renamed without changing behavior:

- `complete()` -> `mark_completed()`
- `fail()` -> `mark_failed()`

All existing Project and Corpus call sites are updated synchronously. No Launcher source, Security source, transport, console, artifact, writer, release, MusicXML, Render or musical rule is changed in this take.

The purpose is to make the shared Observation domain unambiguous before the Launcher is connected in the following SecurityOnly take.
