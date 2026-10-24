# SchlopfNarfCanonicalTemporalEdgeAuthorityCutover r932

**Authority parent:** `0.10.1839 r931`  
**ChangeClass:** `RefactorOnly`  
**Musikmutation:** keine neue  
**HAN03-Schwelle/Severity/Gewicht:** unveraendert  
**Report-/MusicXML-/PDF-Format:** unveraendert

## Reale r931-Basis

r931 ist die released Green Base nach dem Canonical-Endpoint-Identity-Cut. Im anschliessenden normalen Lauf von Etuede Nr. 7 blieb genau eine HAN03-Warnung in Takt 25 bestehen. Der entscheidende neue Befund ist nicht mehr Identity-Drift: HAN03 und VoiceFiberBehindBars arbeiten nun auf denselben kanonischen A/B-Endpunkten. Trotzdem berechnete VoiceFiberBehindBars `DirectAtoB` noch ein zweites Mal aus dem spaeteren AtomicNote-Inventar.

## Schmerzstelle

Vor r932 existierten fuer denselben kanonischen Sprung zwei Besitzer der zeitlichen A/B-Relation:

```text
HAN03
  A/B + Onset-Abstand

VoiceFiberBehindBars
  dieselben A/B erneut aufloesen
  -> same staff/voice erneut pruefen
  -> A.end erneut berechnen
  -> DirectAtoB erneut entscheiden
```

Damit war die Endpoint-Identity bereits vereinheitlicht, die Transition-Semantik aber noch doppelt besessen.

## Schnitt

r932 erweitert die bestehende interne `Han03JumpIdentity` um eine typisierte `Han03TemporalEdge`:

```text
from_end
onset_delta
release_to_attack_gap
direct_attack_successor
```

Diese Relation wird genau einmal im HAN03-Owner aus den bereits ausgewaehlten A/B-Events gebildet. Die bestehende `Zeitabstand`-Pruefung verwendet denselben kanonischen `onset_delta`; Schwelle und Warnungsentscheidung bleiben unveraendert.

`VoiceFiberBehindBars` berechnet `DirectAtoB` nicht mehr selbst aus den spaeter aufgeloesten AtomicNotes, sondern konsumiert ausschliesslich `identity.temporal_edge.direct_attack_successor`. A/B muessen dort weiterhin ueber den r931-Locator exakt und eindeutig aufloesen; fehlt diese Bindung, bleibt der Pfad fail-closed.

Die Verlagerung aendert die bestehende Entscheidung nicht: Der r931-Locator verlangt fuer beide Endpunkte bereits exakte Uebereinstimmung von Measure-Scope, Staff/Voice-Scope, Start, Dauer, Chord-Rolle, Tie-Stop-Rolle und Pitch. Wenn die AtomicNotes nicht mehr zu den bei HAN03 gebundenen Endpunkten passen, erreicht BehindBars die Temporal-Edge-Entscheidung gar nicht.

## Nicht geaendert

- keine neue oder geaenderte HAN03-Schwelle,
- keine Severity-/Weight-Aenderung,
- keine Unterdrueckung der Etuede-7-Warnung,
- keine neue Reparatur oder Mutation,
- keine neue Writer-Authority,
- keine Etuede-/Takt-/Pitch-Sonderregel,
- keine nearest-note-Heuristik,
- keine Aenderung am BehindBars-Neutralisierungspraedikat ausser der Entfernung der doppelten `DirectAtoB`-Berechnung,
- keine persistierte neue Report-/MusicXML-/PDF-Struktur.

`Han03TemporalEdge` bleibt wie die umgebende JumpIdentity interne, nicht serialisierte Runtime-Identity.

## Regressionen

Neue HAN03-Tests beweisen:

1. ein zeitlich anschliessender Angriff besitzt `release_to_attack_gap=0` und `direct_attack_successor=true`,
2. ein Angriff mit positiver Luecke besitzt denselben eindeutigen A/B-Kontext, aber `direct_attack_successor=false`,
3. selbst bei zeitlicher Kontiguitaet ist ein anderer Voice-Scope kein direkter Nachfolger.

Die bestehenden r931-Tests bleiben zusaetzlich erhalten: kanonische A/B-Identity statt Issue-Text und fail-closed ohne diese Identity.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 3"
```

Nur `dark_green / released / FinalExit 0` akzeptiert r932 als neue Green Base. Erst danach wird Etuede 7 normal ohne `--take` erneut ausgefuehrt. Dieser Take soll die Warnung dort bewusst noch nicht wegentscheiden; er entfernt ausschliesslich den zweiten Besitzer der Temporal-Edge-Wahrheit.
