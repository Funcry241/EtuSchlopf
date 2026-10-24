# r607 PianisticStaticCapabilityFrontierAudit

## Parent und ChangeClass

Autoritativer Parent ist EtuSchlopf `0.10.1839 r606` `RegisterNotationGreenBaseReportEssenceCleanup`.

`ChangeClass: AuditOnly`.

Keine Musikmutation, kein Writer, kein MasterApply, kein RenderXmlWrite, keine neue pianistische Produktregel und keine neue angeblich menschliche Spannweiten-Grenze.

## Motivation

Der r299/r300-Strang bindet bewusst ein `REFERENCE_ASSUMPTION`-Modell mit einer festen aeusseren Handspanne von 12 Halbtoenen. Der Vollkorpus falsifiziert dieses explizite Referenzmodell an statischen Zwei-Hand-Grenzen. Diese Falsifikation darf nicht durch das willkuerliche Ersetzen von 12 durch 13, 14 oder einen anderen vermeintlich realistischen Wert repariert werden.

r607 dreht die Frage deshalb um:

> Welche minimale einheitliche aeussere Spannweiten-Grenze pro Hand verlangt der konkrete Score an jeder simultanen physischen Tasten-Grenze ueberhaupt, wenn genau zwei Haende mit hoechstens fuenf verschiedenen Tasten je Hand betrachtet werden?

Das Ergebnis ist eine Score-Demand-/Kalibrierungsfrontier und kein menschlicher Playability-Claim.

## Exakte statische Frontier

Fuer jede simultane `PhysicalKeyObligation`-Grenze werden zuerst die **verschiedenen physischen Tastenpositionen** bestimmt. Mehrere gleichzeitig aktive unabhaengige Obligations auf derselben MIDI-Taste werden dabei nicht als geloeste Ausfuehrung interpretiert, sondern separat als `SameKeyOverlap`-Demand gezaehlt. Anschliessend werden alle beschrifteten Links/Rechts-Partitionen der verschiedenen Tasten exhaustiv enumeriert.

Zulaessig fuer die Positions-/Frontier-Berechnung sind nur Partitionen mit:

```text
LeftKeyCount  <= 5
RightKeyCount <= 5
```

Fuer jede Partition wird berechnet:

```text
required_span = max(
    outer_span(left),
    outer_span(right)
)
```

Die Boundary-Frontier ist das Minimum dieses Werts ueber alle zulaessigen Partitionen. Sie ist damit eine **positionale Untergrenze**: ein `SameKeyOverlap` kann trotz kleiner Positionsfrontier weiterhin eine eigenstaendige zeitliche/technische Unmoeglichkeit unter einem spaeteren Modell darstellen.

Damit entsteht keine neue feste Handspanne. Die Frontier misst ausschliesslich, welche statische Spanne ein spaeteres Capability-Modell mindestens abdecken muesste, damit diese notwendige Bedingung nicht bereits scheitert.

## Bewusste Grenzen

Nicht modelliert bzw. nicht behauptet werden:

- menschliche Handmorphologie,
- empirische Fingerreichweite,
- schwarze/weisse Tasten als 3D-Geometrie,
- Handposition und Rotation,
- Fingerwechsel oder Substitution,
- Sprung-/Transitionzeit,
- Tempo-Feasibility,
- Pedaltechnik,
- allgemeine menschliche Spielbarkeit.

Same-Key-Overlap wird ausdruecklich nicht stillschweigend zu einer zulaessigen Fingerbelegung umgedeutet. r607 trennt nur die Positionsfrage von der spaeter zu klaerenden Re-Attack-/Hold-/Release-Technikfrage.

`HumanPlayabilityClaim` bleibt `NOT_EVALUATED`.

## Vergleich mit r299

Der bestehende r299-Wert `12` wird nicht geaendert. r607 zaehlt lediglich, wie viele statische Grenzen eine Frontier `>12` besitzen und welcher maximale minimale Frontier-Wert fuer den Score beobachtet wird.

Damit kann der naechste Take gezielt unterscheiden:

```text
r299 scheitert an einer zu engen Positions-/Spannweiten-Referenzannahme
vs.
Same-Key-Overlap ist der eigentliche ungeklaerte Technikgegenfall
vs.
Score besitzt einen anderen statisch unmodellierten Gegenfall
vs.
statische Frontier ist geschlossen und der naechste echte Rest liegt in Transition/Tempo
```

## Falsifikation

r607 ist zu verwerfen, wenn:

- Build, Strict-Clippy oder Tests scheitern,
- MusicXML-/PDF-Bytes oder musikalische Semantik geaendert werden,
- aus einer Frontier ein menschlicher `SUPPORTED`-/`REJECTED`-Claim abgeleitet wird,
- eine neue feste produktive Spannweitenregel eingefuehrt wird,
- die exakte Partitionierung nicht deterministisch und exhaustiv bleibt,
- mehr als fuenf verschiedene Tasten pro Hand stillschweigend akzeptiert werden.

## Naechster fachlicher Schritt

Nach real gruener r607-Korpusqualifikation kann die beobachtete Frontier mit einer unabhaengig begruendeten Capability-Evidenz und anschliessend mit einem echten zeitabhaengigen Transitionsmodell verbunden werden. Die Score-Demands selbst duerfen dabei weiterhin nicht zirkulaer die menschliche Capability definieren.
