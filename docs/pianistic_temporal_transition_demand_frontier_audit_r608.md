# r608 PianisticTemporalTransitionDemandFrontierAudit

## Parent und ChangeClass

Autoritativer Parent ist EtuSchlopf `0.10.1839 r607` `PianisticStaticCapabilityFrontierAudit`.

`ChangeClass: AuditOnly`.

Keine Musikmutation, kein Writer, kein MasterApply, kein RenderXmlWrite, keine neue pianistische Produktregel, keine menschliche Geschwindigkeitsgrenze und kein Tempo-Feasibility-Claim.

## Ausgangsevidenz r607

Der reale r607-Vollkorpus ist 10/10 GREEN (`FinalExit 0`), Build und Strict-Clippy sind GREEN und der integrierte Testbestand liegt bei `1193/1193`.

r607 hat die statische Positionsfrage von Same-Key-Overlap getrennt. Im 10er-Korpus wurden dabei insgesamt `72` Grenzen mit ueberlappenden Verpflichtungen auf derselben physischen Taste und `85` zusaetzliche Same-Key-Verpflichtungen sichtbar. Diese Befunde duerfen weder als Spannweitenproblem noch automatisch als menschliche Unspielbarkeit interpretiert werden.

Die statische Frontier selbst reicht im Korpus von maximal 12 Halbtoenen in Etuede 1 bis maximal 27 Halbtoenen in Etuede 6. Der bisherige r299-Wert 12 bleibt eine bewusst enge Referenzannahme und wird weiterhin nicht durch einen aus dem Score abgeleiteten angeblich menschlichen Wert ersetzt.

## Ziel r608

r608 schliesst die **Score-seitige Zeitachse** fuer den naechsten Capability-Schritt. Die vorhandene r292-`TempoMap` liefert fuer jeden Attack/Hold/Release bereits exakte physische Zeitpunkte in Mikrosekunden. r608 materialisiert daraus zwei read-only Demand-Frontiers:

1. **Attack-Boundary-Zeitfenster** zwischen aufeinanderfolgenden verschiedenen Angriffspunkten.
2. **Same-Key-Temporal-Demand** fuer nicht-identische Verpflichtungen auf derselben physischen Taste, deren Zeitintervalle sich ueberlappen.

Damit beantwortet r608 nicht die Frage, wie schnell ein Mensch oder eine Hand sein muss. Es beantwortet nur die vorgelagerte Frage:

> Wann und in welchen zeitlichen Abstaenden fordert der konkrete Score ueberhaupt Zustandsaenderungen?

## Attack-Boundary-Frontier

Alle Grenzen mit mindestens einem neuen physischen Tastenangriff werden in kanonischen 96 Ticks/Viertel und ueber die r292-TempoMap in Mikrosekunden geordnet.

Fuer die positiven Zeitfenster werden ohne feste Schwelle ausgegeben:

```text
minimum
q25
median
q75
maximum
```

sowohl fuer kanonische Ticks als auch fuer reale Mikrosekunden.

Die acht kuerzesten beobachteten Attack-Fenster werden mit Start-/Ziel-Tick, physischer Zeit, Zahl der Ziel-Attacks und Zahl gleichzeitig aktiver physischer Verpflichtungen ausgegeben.

Diese Werte sind Score-Demand. Es gibt keinen produktiven Grenzwert wie `unter X ms = unspielbar`.

## Same-Key-Temporal-Demand

Die bestehende DemandTopology koalesziert weiterhin nur **exakte** Duplikate mit identischem MIDI, Attack, Release und physischer Zeit. Nicht-identische zeitlich ueberlappende Verpflichtungen auf derselben MIDI-Taste bleiben absichtlich getrennt.

r608 klassifiziert jede solche Paarueberlappung in:

```text
SimultaneousAttack
oder
StaggeredReattackDuringHold
```

und unabhaengig davon:

```text
SecondReleaseNestedInFirst
oder
ExtendingRelease
```

Gemessen werden unter anderem:

```text
InterAttackTicks
InterAttackTimeUs
RemainingPriorHoldTimeUsAtReattack
```

Damit kann ein spaeterer Technik-/Capability-Schritt unterscheiden, ob ein Befund aus gleichzeitiger Mehrstimmennotation derselben Taste, einer echten Re-Attack-Anforderung waehrend fortbestehender anderer Verpflichtung oder einer anderen zeitlichen Struktur stammt.

r608 entscheidet **nicht**, ob ein einzelner physischer Anschlag mehrere notierte Stimmen erfuellen darf. Eine solche Entscheidung waere Semantik-/Technikmodell und gehoert nicht in diesen Audit.

## Bewusste Grenzen

Nicht modelliert bzw. nicht behauptet werden:

- Handzuordnung,
- Fingerzuordnung,
- Handposition,
- Sprunggeschwindigkeit,
- Finger-Repetitionsgeschwindigkeit,
- menschliche Reaktionszeit,
- empirische Technikfaehigkeit,
- Tempo-Feasibility,
- Pedaltechnik,
- Score-Reparatur.

`HumanPlayabilityClaim` bleibt `NOT_EVALUATED`.

## Architekturbezug

r608 baut ausschliesslich auf bereits vorhandenen Ownern auf:

```text
PianisticExecutionScoreAuthority
    -> PianisticExecutionPlanWitnessDemandTopology
    -> PianisticTemporalTransitionDemandFrontierAudit
```

Die neue Projektion besitzt keine Writer- oder Decision-Authority.

Die r297-Source-Staff-Affinity-Probe bleibt unveraendert. r608 benutzt `staff != hand` konsequent und erzeugt selbst keine Handwahrheit.

## Falsifikation

r608 ist zu verwerfen, wenn:

- Build, Strict-Clippy oder Tests scheitern,
- MusicXML-/PDF-Bytes oder musikalische Semantik geaendert werden,
- ein menschlicher Zeit-/Geschwindigkeitsgrenzwert eingefuehrt wird,
- Same-Key-Overlap automatisch als unspielbar oder automatisch als loesbar klassifiziert wird,
- exakte Duplikatkoaleszierung der bestehenden DemandTopology umgangen wird,
- aus Source Staff eine Handzuordnung wird,
- ein Score-Repair oder Writer entsteht.

## Naechster fachlicher Schritt

Nach real gruener r608-Qualifikation existieren fuer den Capability-Zweig getrennt:

```text
r607: statische Positions-/Spannweiten-Demand-Frontier
r608: zeitliche Transition-/Same-Key-Demand-Frontier
```

Erst danach ist es sinnvoll, eine **unabhaengig begruendete** Capability-Evidenz fuer Hand-/Finger-Konfigurationen und Transitionen zu binden. Score-Demands duerfen weiterhin nicht zirkulaer ihre eigenen Capability-Antworten definieren.
