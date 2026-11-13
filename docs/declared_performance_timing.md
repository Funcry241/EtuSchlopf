# Declared performance timing in the notation-time owner

A timing map is explicit input metadata in MIDI Marker meta events (FF 06).
The first marker is `EtuSchlopf notation v1 meter N`, where N is 2, 3 or 4
quarter beats. Its MIDI tick maps to written position zero at the start of a
full measure. Use `EtuSchlopf notation v1 meter N pickup P` when it starts
an incomplete measure of length P. The units are 96 divisions per quarter.
An incomplete opening measure keeps the declared meter and carries its
physical duration through the seed, arrangement and MusicXML. Subsequent markers
use `EtuSchlopf notation v1 anchor Q`, with the absolute written position Q
in the same units. Markers must bracket all note boundaries. Raw ticks and
written positions strictly increase and written anchors are at most one
quarter apart. An existing MIDI meter must agree with the declared meter.

The owner interpolates each boundary between adjacent anchors, then admits
the nearest six-unit position only within a quarter of that spacing.
It refuses a collapsed duration or two attacks of the same key on the same
MIDI channel at one written position. No implicit performance tempo or
ornament is inferred. The map supports one constant meter and an optional opening pickup.
Existing exact MIDI and its narrow grace projection use their existing input
path.

Anchors declare a time map. Their existence is not a proof that a pianist
intended a particular meter, pulse or note value. Pulse estimation and
metrical-phase inference for unmarked recordings remain separate work.

## Inferred performance timing

When exact MIDI projection fails and the input has no declared meter or
declares 2/4, the notation-time owner can admit a steady binary subdivision
from the played MIDI attacks. It groups near-simultaneous attacks, estimates
local subdivision length for written releases and checks that recurring bass
accents establish a quarter-note phase. It uses 2/4 for this binary model.
Without meter metadata this is a notation choice. A short isolated opening
attack followed by the first supported bass accent can form an eighth-note
pickup. Short stepwise approaches inside a group can attach as grace notes.
The original MIDI ticks remain source provenance. WAV remains the audio
input for the existing audio analysis. The timing model uses MIDI evidence.


The seed keeps every attack in an inferred timeline. Arrangement and
playability repair preserve visible attacks at their written onset and pitch.
For simultaneous groups with no bound source voices, the seed can redistribute
notes if both hands meet the validator's span and key-count limits. It prefers
fewer staff changes, then the smaller maximum span.
