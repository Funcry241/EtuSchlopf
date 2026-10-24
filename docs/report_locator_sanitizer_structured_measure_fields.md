# Locator-Sanitizer: strukturierte Taktfelder

Stand: v0.10.1625 r01

Der vorherige Abschluss bewahrte Zaehler, Verhaeltnisse sowie Offset-/Dauerfolgen
wieder wahrheitsgetreu und anonymisierte Slash- und Kommalisten hinter direkten
`Takt`-/`Measure`-Markern. Im realen Bericht blieben jedoch weitere konkrete
Taktkoordinaten in strukturierten Feldformen sichtbar:

- `target M22` und `Ziel M22`,
- `measure=5`,
- `AffectedScopes 2,10`,
- `CompleteBeatMeasures 4,9,...`,
- `Measures 4,9,...` in einem Complete-Beat-Readback.

Dieser RefactorOnly-Take anonymisiert genau diese strukturierten Locatorfelder.
Ein einzelnes `Measures 31` bleibt als Anzahl erhalten; ebenso bleiben andere
Kommalisten wie `voices=1,2` unveraendert. Musik, Writer, Renderer, Solver,
Freigabe und produktive Auswahl werden nicht geaendert.
