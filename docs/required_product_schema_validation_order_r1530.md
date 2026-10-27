# Required-product schema validation order

The real Windows package run 40424796109 proves that a selected Master without
Render transformation history can be rendered by MuseScore, but the registered
pair gate ran too early: the unchanged required Render product is realized only
at the final sovereign closure. Its missing file was cached as a final failure,
while the later report still claimed OK.

Before MuseScore, the selected persisted source must pass the unchanged declared
MusicXML-4.0 product check and the same bound SchemaSchlopf-r021 validator.
Registration binds both its input path and PDF destination. This source gate
does not cache or claim complete-product validation. It neither creates nor
repairs a file, and never accepts a missing or invalid selected source.

After the existing required-product closure, both persisted Master and Render
must pass the complete product gate before report construction or an OK console
verdict. A missing, malformed, schema-invalid or wrong-version unselected product
still prevents successful release. The outer process-exit check remains. The
four exact successor files bound by accepted r1529 remain byte-identical.

This changes validation sequencing only. It introduces no musical selector,
rounding, product writer, transformation history, renderer repair or exception
for a project, pitch, measure, voice, corpus member or file name.

The original foreign Mutopia MIDI remains unchanged and required to succeed in
the package tests. Its 43/384-quarter offset is not exactly representable in the
current 96-division source model. This ordering repair does not claim to solve
that separate compatibility failure, and a green synthetic PDF test cannot
qualify the whole EXE for release.
