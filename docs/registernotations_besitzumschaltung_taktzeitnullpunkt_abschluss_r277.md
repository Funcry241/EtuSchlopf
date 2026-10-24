# Registernotations-Besitzumschaltung – Taktzeitnullpunkt-Abschluss r277

r277 basiert direkt auf dem real dunkelgruenen r274. r275 wurde vor dem Kopieren abgelehnt. r276 bestand PRE-TAKE-Gate, Clippy, Build und 967/967 integrierte Pruefungen, scheiterte aber im realen Etuede-6-Lauf fail-closed an der Ruecklesepruefung der neuen Registernotationsmaterialisierung und wurde deshalb nicht zum neuen Elternstand.

Der Laufzeitbefund war:
`materialisiertes Stop-Ende ist keinem Plan zuordenbar`.

Die Ursache liegt nicht im r262-RegisterNotationPlan. Der r276-Materialisierer setzte Richtungen pauschal hinter das erste `</attributes>` eines Taktes. In der realen Etuede-6-MusicXML besitzen Takt 26 und Takt 30 jedoch einen `attributes`-Block erst nach bereits zeittragenden MusicXML-Ereignissen; der MusicXML-Zeitcursor steht dort jeweils bereits bei 192. Da `<direction><offset>` relativ zum aktuellen Cursor ausgewertet wird, verschob dieser Einfuegepunkt die rueckgelesene Richtung um genau den bereits verstrichenen Taktanteil.

r277 aendert ausschliesslich den Materialisierungsanker. Neue Oktavrichtungen werden hinter den fuehrenden zeitneutralen direkten Taktknoten und zwingend vor dem ersten zeitveraendernden `note`, `backup` oder `forward` eingefuegt. Anfangs-`attributes` bleiben damit vor den Richtungen; mittige `attributes` koennen die Registernotationszeit nicht mehr verschieben.

Ein neuer Regressionstest bildet einen Takt mit 192 Ticks Musik vor einem mittigen `attributes`-Block ab und verlangt die exakte Start-/Stop-Rueckleseparitaet. Die produktive Fachentscheidung bleibt unveraendert der mit r262 bewiesene RegisterNotationPlan. Keine neue Oktavierungsformel, keine Takt-/Etueden-Sonderregel, keine Sicherheits-, Bootstrap-, Recovery- oder Orchestrator-Aenderung.

Erwartung: 968/968 integrierte Pruefungen. Danach entscheidet der reale Etuede-6-Lauf, ob die 22 bewiesenen Plaene erstmals produktiv und exakt materialisiert werden.
