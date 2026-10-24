# r385 – Warning-Free Pipeline Console Contract Compile-Ownership Closure

r385 basiert direkt auf der real DARK GREEN installierten Authority r384. Strict-Clippy steht auf 0 Warnings; der verbleibende Suppression-Bestand besteht aus 15 `dead_code`- und 8 `clippy::too_many_arguments`-Ausnahmen.

Dieser RefactorOnly-Schnitt pensioniert genau eine `dead_code`-Suppression in `src/pipeline.rs`. Der Pipeline-Namespace kompilierte `src/console_presentation_contract.rs` bisher als eigenes privates `#[path]`-Modul, obwohl derselbe authority-free/std-only Contract bereits durch die Package-Library als `etuschlopf::console_presentation_contract` bereitgestellt wird. Die asymmetrische private Doppelkompilierung erzeugte unbenutzte Contract-Symbole und wurde durch `#[allow(dead_code)]` maskiert.

r385 entfernt ausschließlich diese zweite Compile-Ownership und importiert dieselben Typen/Funktionen aus dem bereits bestehenden Library-Owner. Es gibt keinen neuen Contract, keine neue öffentliche API, keine Änderung an Console-Policy, RunOutcome, Musik-, Writer-, Renderer-, Pass-, Threshold-, Repair-/Recovery- oder Security-Semantik.

Erwartung nach realem GREEN: Strict-Clippy bleibt 0; reale Suppressions 23 -> 22.
