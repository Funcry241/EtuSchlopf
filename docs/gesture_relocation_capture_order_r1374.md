# Gesture relocation capture order closure — r1374

Parent: accepted r1373, SHA256 98d804e35c1625e0dc26199cd98b8f1d6238409f01016de2a340744300785675.
Class: RefactorOnly. Existing edits, writer, owner signatures and custody checks unchanged.

At a shared insertion position the stable descending writer applies earlier edits first,
so later edits precede them in the resulting bytes. Capture now follows that exact
order and binds each insertion by its original pair ordinal, including byte-identical
insertions. Anchor capture recognizes the note start tag rather than a nested notehead.
No identity is reconstructed from musical coordinates or pitch.

Two synthetic tests pass through the actual owner and receiving custody checks:
different pitches at one anchor and identical pitches with distinct source identities.
They check exact source/root/anchor binding, course and interval, sounding pitch,
distinct identities, output order and note count. A third synthetic test runs the
actual owner on an anchor with notehead markup and checks execution-bound physical
births, the complete captured anchor range and retained sounding pitch. It does not
modify or bypass a sealed seed.
Missing captures and illegal transitions remain fail-closed/Unknown as before.

The real corpus, PDF force dependency and dynamic RAM requirement require separate
runtime evidence; this take claims no closure for them.
