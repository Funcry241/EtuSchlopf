# r619 PianisticSameKeyRearticulationWitnessDemandClosure

## Parent und Scope

- Parent: r618 `PianisticSameKeyOverlapWitnessAdapterReportContractRecovery`
- Version: `0.10.1839`
- ChangeClass: `AuditOnly`
- Writer / RepairAuthority: none
- MusicMutation / MasterApply / RenderXmlWrite: none
- HumanPlayabilityClaim: unveraendert `NOT_EVALUATED`
- Neue menschliche Schwelle: keine

## Gegenbefund nach dem r618-Vollkorpus

r618 schliesst Same-Key-Overlap korrekt: gleichzeitige nicht-identische Pflichten duerfen einen physischen Slot teilen; ein spaeterer Angriff waehrend eines fortbestehenden Same-Key-Holds benoetigt explizite `SameKeyTechniqueSignature`-Evidenz.

Beim anschliessenden Quellvergleich vor der geplanten symbolischen Spielbarkeits-Closure wurde jedoch eine verbleibende Score-Demand-Luecke sichtbar: Wenn eine gleiche Taste nach dem Release der vorherigen abstrakten Pflicht erneut angeschlagen wird und der konkrete Witness denselben Hand/Finger-Slot wiederverwendet, konnte die alte Logik die identische absolute Handkonfiguration als transition-seitig kostenlos behandeln. Damit konnte eine normale Same-Key-Reartikulation ohne Same-Key-Capability-Evidenz durchlaufen.

Das ist keine fehlende Menschenwahrheit, sondern eine objektiv aus Attack-Zeitpunkten und Witness-Slotbindung ableitbare Demand-Luecke. Eine spaetere Behauptung, die Capability-Bedingungen des v2-Witness seien vollstaendig, waere ohne diesen Abschluss falsch.

## r619-Semantik

Fuer jeden neuen Angriff auf MIDI `m` und den im Kandidaten gebundenen Slot `(Hand,Finger)` wird der zeitlich juengste fruehere Angriff auf dasselbe MIDI gesucht, der im selben Witness ebenfalls genau diesem Slot zugewiesen ist.

Existiert ein solcher frueherer Angriff, gilt:

```text
available_rearticulation_time = current_attack_time - prior_attack_time
```

und der Zweig benoetigt die bereits in r618 gebundene:

```text
SameKeyTechniqueSignature = Hand + Finger + KeySurface
```

Dies gilt unabhaengig davon, ob die alte abstrakte Pflicht am neuen Attack-Zeitpunkt noch aktiv ist.

Damit werden zwei Faelle einheitlich behandelt:

```text
staggered re-attack during older hold
post-release same-slot rearticulation
```

Gleichzeitige Angriffe mit identischem Attack-Zeitpunkt bleiben davon ausgenommen und duerfen wie in r618 einen physischen Attack/Slot gemeinsam erfuellen.

## Fail-closed-Vertrag

```text
fehlende Same-Key-Evidenz
-> UNMODELED
-> MODEL_INSUFFICIENT
```

```text
explizite Same-Key-Evidenz + ausreichende Zeit
-> Zweig kann weiterlaufen
```

```text
explizite Same-Key-Evidenz + Zeit unter explizitem Minimum
-> genau dieser modellierte Zweig wird REJECTED
```

`NO_REALIZATION_UNDER_MODEL` bleibt nur zulaessig, wenn alle strukturell relevanten Alternativen explizit geschlossen sind.

## Search / Replay

Search und Independent Replay verwenden denselben erweiterten Vertrag. Der Replay sucht den letzten frueheren Same-Key-Angriff mit derselben finalen Slotzuweisung und prueft dieselbe attack-to-attack Zeit gegen dasselbe Capability-Modell.

Damit darf ein konstruktiver Witness die Reartikulationspflicht nicht im Search akzeptieren und beim Replay verlieren oder umgekehrt.

## Bewusste Grenze

r619 behauptet weiterhin nicht, dass `SameKeyTechniqueSignature` die gesamte menschliche Repetitionstechnik beschreibt. Insbesondere werden keine score-abgeleiteten Mindestzeiten, keine allgemeinen Instrumenten-Repetitionsgrenzen und keine menschlichen Fingerwechselwahrheiten eingefuehrt.

Wenn zwei nicht ueberlappende Same-Key-Angriffe verschiedene Slots verwenden, wird keine Same-Key-Signatur erfunden. Solche Hand-/Fingerwechsel bleiben der bereits vorhandenen Transition-Capability-Semantik zugeordnet.

## Regressionen

r619 ergaenzt drei Unit-Tests fuer direkt aufeinanderfolgende gleiche Tasten nach Release:

1. gleicher einzig unterstuetzter Slot + fehlende Same-Key-Evidenz -> `MODEL_INSUFFICIENT`
2. gleicher einzig unterstuetzter Slot + explizit ausreichende Same-Key-Evidenz -> Witness + Independent Replay `VERIFIED`
3. gleicher einzig unterstuetzter Slot + explizites Minimum oberhalb des Score-Fensters -> `NO_REALIZATION_UNDER_MODEL`

Der Runtime-Integrity-Probe deckt zusaetzlich den post-release Fall mit und ohne Same-Key-Evidenz ab.

Ausgehend von r618 mit `1220/1220` Tests werden bei vollstaendig gruener Qualifikation `1223/1223` Tests erwartet.

## Naechster Schritt

Erst nach gruener r619-Qualifikation ist die geplante symbolische Capability-Closure fachlich sauber: Ein symbolischer Execution-Ausdruck darf dann Static-, Transition- und Same-Key-Anforderungen transportieren, ohne eine reale Capability zu erfinden und ohne eine Wiederholungsanforderung identischer Konfigurationen zu verlieren.
