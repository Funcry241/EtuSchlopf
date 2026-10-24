# r512 – Final-Notation Timeline Clearance Edge Evidence Audit

## Ziel

r512 erweitert ausschließlich die bestehende writerfreie
`RendererSystemBreakReadbackRhythmicShape`-Evidenz. Die bereits vorhandene
`musicxml_timeline_columns_v1`-Rekonstruktion bleibt die Timeline-Basis; es
wird kein zweiter XML-Scan, kein neuer Layout-Owner und keine neue
Spacing-Policy eingeführt.

Der Audit beobachtet zusätzlich pro Takt, wie viele sichtbare
16tel-Pausen in derselben Staff-/Voice-Lane unmittelbar zwischen zwei
gleich positionierten Tonhöhen liegen, wenn der folgende Ton eine
Alterationsänderung bzw. ein explizites Vorzeichen trägt. Diese Klasse ist
nur eine horizontale Clearance-Evidenz. Sie besitzt weder Rest-Placement-
noch Layout-, Writer-, Repair- oder Renderer-Authority.

## Architekturgrenze

Die neue Kennzahl `clearanceEdges` wird im bestehenden
`PlannedSystemShapeEvidence` neben Timeline-Spalten, klingenden Spalten,
Notenelementen und Stack-Tiefe ausgegeben.

Der bereits vorhandene
`RendererSystemBreakReadbackDemandBinding` liefert unabhängig davon die
aktuellen Measure-Demands und System-Headrooms. r512 verbindet diese
beiden Beobachtungen nicht zu einem neuen Score und ändert weder Demand
noch Capacity.

Damit kann geprüft werden, ob ein lokaler sichtbarer
Rest/Vorzeichen-Clearancebedarf in einem Takt vom heutigen groben
Measure-Demand ausreichend repräsentiert wird, ohne eine neue
Layout-Entscheidung vorwegzunehmen.

## Unverändert

- keine MusicXML- oder PDF-Mutation,
- keine neue Writer-Aufrufkante,
- keine StructuralWriteAuthority-Änderung,
- keine `measure/@width`-Reaktivierung,
- keine `default-x`-Writes,
- keine Layout→Renderer→Notation-Feedback-Schleife,
- keine neue Spacing-Schwelle oder Gewichtung,
- keine Etüden-, Takt-, Pitch- oder Projektidentitätsregel,
- keine Wiederbelebung pensionierter Width-/Register-Audits.

## Falsifikation

Der nächste produktive Layoutschritt ist nur gerechtfertigt, wenn die
writerfreie Clearance-Evidenz zusammen mit den bereits gebundenen
Measure-Demands die menschlich auffällige Klasse von sauberen
Vergleichsfällen sinnvoll unterscheidet. Andernfalls bleibt r512
diagnostisch und es erfolgt kein Cutover.
