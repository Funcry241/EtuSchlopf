# Real, audible package-test inputs

`synthetic.mid` is the project-authored eight-bar synthetic example (CC0-1.0).
`menuet.mid` is the unchanged external Mutopia edition, catalog 75,
Mutopia-2017/01/19-75, typeset by Allen Garvin from the Bach-Gesellschaft edition.
Its catalog explicitly identifies the contributor's edition/MIDI as Public Domain.
The catalog's historical Bach attribution is retained in the original MIDI;
the piece BWV Anh. 114 is now attributed to Christian Petzold.

Sources checked 2026-10-27:
- https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=75
- https://www.mutopiaproject.org/legal.html
- https://www.mutopiaproject.org/ftp/BachJS/BWVAnh114/anna-magdalena-04/anna-magdalena-04.mid

Each WAV is audible, nonzero PCM16 mono audio, rendered from that exact MIDI
using the accompanying deterministic harmonic synthesis script. This audio is
our own generated performance, dedicated to CC0-1.0, with no sampled soundfont.
It is not an independent acoustic recording or an independent timing witness.
The engine receives the original unchanged MIDI and the generated WAV as inputs;
no product code has a piece-specific branch. This external piece is evaluation,
not an optimization target or a normative musical oracle.

`manifest.json` binds MIDI, WAV and gzipped-WAV SHA256, duration and measured RMS.
CI decompresses the committed WAV and rejects hash mismatches, silence, absent
products, report/exit contradictions and missing real PDFs. Public-domain status
of a composition alone was not used as permission for an arbitrary recording.
The exact MIDI edition's public-domain dedication and our own audio origin are
recorded separately.
