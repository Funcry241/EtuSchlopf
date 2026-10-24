# Original chord beam attachment observation

RefactorOnly on accepted r1422. The original beam producer deliberately excludes
physical chord members from its own planned note rows. FinalChord independently
materializes some such members as auxiliary runs. A complete original head
capture therefore does not establish an independent beam decision for that member.

At the original E053 execution this take records every pitched physical chord
member and its original pitched physical head. Cursor movements and measure
boundaries terminate that physical relationship. Only a head actually present
in the original captured beam decision yields a positive original-head binding.
Missing or unowned heads stay Unknown, including orphan chords and rest anchors.
Every range is carried by the same original beam/rest operation maps and later
original receipt operations. Member content changes invalidate only attachment
observation; the original owner receipt and head decision remain intact.

The current canonical source seam binds both ends through exact original byte
ranges and existing Source/Attack/Fragment locators. The report shows original
head levels separately. They are never relabeled as independent member decisions.
All rows remain observational: IndependentBeamDecisionPermit none. There is no
channel sharing, new beam plan, inferred absence or musical writer change.

This closes an observed prerequisite gap, not the final capacity repair. The
prepared channel-sharing experiment is excluded from this take because its
positive all-level obligation cannot yet be met for auxiliary origins.
