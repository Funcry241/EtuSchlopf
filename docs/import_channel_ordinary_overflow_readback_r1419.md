# r1419 – Actual ordinary-channel overflow readback

AuditOnly on accepted r1418. No writer, musical selector, role, receipt, adoption,
channel assignment, renderer command or public resource policy changes.

MuseScore 4.7.4 allocateStaves calls VoiceDesc::setStaff(-2) when a regular
channel does not fit. That setter ignores negative values. The channel stays
unassigned and wins again during every remaining ordinary-allocation iteration.
Lower-priority ordinary channels on either staff are not visited, even when
the other staff has free slots. The separate overlapping allocation still runs.

The former readback stored an explicit rejected state and proceeded to smaller
channels. This take removes that state transition and documents the actual
consumer behavior. Report vocabulary and profile identifier remain stable;
LostPitchedOccurrences and UnreservedChannels now include the secondary losses.
The intended abstract slot limit and every musical authority remain unchanged.

Synthetic tests exercise the actual XML readback and allocation function: both
staff presentations, order-preserving label renaming, uniform time scaling,
equal-count key priority, the non-overflow neighbor, overlapping-phase independence
and independent duplicated parts. No real corpus witness is a normative fixture.

Real corpus and native files are separate evaluation evidence only. A zero
profile loss still does not establish complete native material, staff, tie,
beam, rest or playability preservation. This take corrects observation; it
does not repair channel capacity or finish occurrence-role binding.

The user workflow remains ./run.cmd --take --all, without force.
