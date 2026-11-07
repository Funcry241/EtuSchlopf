# r1608 – exact admission for Render beam custody

This named KodexOnly prerequisite keeps the r1607 runtime byte-identical.
The existing C3 initial-stage checker additionally admits one full source
image. Its digest is bound in the checker and its exact edit set is reproduced
in a synthetic gate test.

The only admitted additions to that runtime file are a custody field in
the initial Render result, a clone passed to the unchanged C3 call, and
transfer of the original custody handle to that field. This extends the
lifetime of already observed evidence across a retained C3 course. It
does not mint evidence, allow stale evidence, change C3, or select any group.

Later consumers must bind actual Render groups to exact projected fragments
through the authorized occurrence continuation. Unknown or ambiguous
continuation grants no alignment. Musical write authority and all occupancy
and neighboring-group checks stay with the existing owner. That binding
and its verification belong to a separate implementation take.
