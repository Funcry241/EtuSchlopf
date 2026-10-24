# r283 – Registernotation: Sichtzustandspfad-Zielbildbeweis

## Zweck

r283 basiert direkt auf dem real installierten und dunkelgruenen r282 `RegisternotationVierDomaenenUndSichtzustandsPfadBeweis` mit 994/994 Pruefungen. r282 hat die vier Fachdomaenen Logik, Geltung, Sicht und Projektion technisch geschlossen, fuer Etuede 6 aber drei musikalische Sichtentscheidungen absichtlich offen gelassen: einen direkten Darstellungswechsel und zwei stille Kontinuitaetskandidaten.

r283 bleibt `AuditOnly`. Es schreibt keine MusicXML- oder PDF-Datei, veraendert keinen Writer, besitzt keine StructuralWriteAuthority und aendert weder den semantischen `RegisterNotationPlan` noch die r277-Produktivmaterialisierung. Ziel ist ausschliesslich ein vollstaendig bestimmtes, schreibfreies Sichtzielbild, das ein spaeterer MusicMutation-Take ohne eigenes musikalisches Entscheidungsrecht materialisieren kann.

## Entscheidung 1 – direkter Wechsel verschiedener Darstellungen

Ein direkter Kontakt zweier semantischer Plaene unterschiedlicher Darstellung bleibt ein echter Sichtzustandswechsel.

Begruendung:

- Die **Logikdomaene** besitzt bereits die Entscheidung, ob ein Bereich `8va`, `15ma`, `8vb` oder `15mb` benoetigt.
- Die **Sichtdomaene** darf diese Darstellung nicht umdeuten, nur um eine ruhigere Linie zu erhalten.
- Eine Fusion `15ma -> 8va` zu einer einzigen gemeinsamen Darstellung waere deshalb keine reine Sichtentscheidung mehr, sondern eine neue Registerentscheidung.
- r283 besitzt dafuer absichtlich kein Schreib- oder Neuentscheidungsrecht.

Damit bleibt ein solcher Kontakt als Stop-/Start-Paar zweier verschiedener Sichtzustaende erhalten. Die technische Projektion wird erst spaeter entscheiden, wie dieser bereits feststehende Wechsel MusicXML-konform materialisiert wird.

## Entscheidung 2 – Stille als neutraler Sichtzustand

Eine reine stille Luecke zwischen zwei Sichtspannen derselben Darstellung ist nicht automatisch ein Zustandswechsel. Sie kann als visuelle Kontinuitaet ueberbrueckt werden, wenn sie im lokalen rhythmischen Massstab kurz ist.

Die Literatur unterscheidet ebenfalls zwischen kurzen Unterbrechungen, ueber die eine Oktavlinie fortgesetzt werden kann, und ausgedehnteren Pausen, bei denen getrennte Oktavzeichen lesbarer sind. r283 fuehrt daraus **keine feste Dauerzahl** ein. Stattdessen wird die Stille relativ zu den beiden angrenzenden sichtbaren rhythmischen Grenzgestalten beurteilt:

```text
StilleDauer <= min(LinkeGrenzgestaltDauer, RechteGrenzgestaltDauer)
```

Nur dann gilt die Stille als lokale Rhythmusunterbrechung derselben sichtbaren Gestalt.

Eigenschaften dieser Regel:

- keine feste `divisions`-Schwelle;
- keine Taktzahl;
- keine Etuedenkennung;
- keine Gewichtungsformel;
- keine Fusion ueber einen dazwischenliegenden Ton;
- keine Aenderung der klingenden oder geschriebenen Tonhoehe;
- eine laengere Stille bleibt als eigener Sichtbruch erhalten.

Die lokale Grenzgestaltdauer wird aus den realen cursorbewegenden, sichtbaren Tonankergruppen unmittelbar links und rechts der Stille gewonnen. Ein klingender Akkordnachlauf bleibt weiterhin getrennte Quellkonformitaetsevidenz und vergroessert diese rhythmische Grenzgestalt nicht heimlich.

## Erwartetes Zielbild fuer Etuede 6

Aus r282 liegen vor:

```text
Semantische Plaene                 22
Sichere Kontaktfusionen             4
Sichtspannen vor Zielbild           18
Direkte Darstellungswechsel          1
Stille Kontinuitaetskandidaten       2
```

Der direkte Wechsel bleibt erhalten. Beide stillen Kandidaten besitzen im aktuellen Falsifikationsstueck eine Stilledauer von 24 und werden beidseitig von sichtbaren rhythmischen Grenzgestalten der Dauer 24 begrenzt. Sie sind damit nach derselben allgemeinen Regel lokale Rhythmusunterbrechungen.

Erwartung:

```text
Darstellungswechsel bestaetigt       1
Stille Bruecken verbunden            2
Stille Bruecken getrennt             0
Offene Sichtentscheidungen           0
Sichtlinien im Zielbild              16
```

Insbesondere entsteht im oberen System eine ruhigere 8va-Fuehrung:

- der semantisch feststehende `15ma -> 8va`-Wechsel bleibt sichtbar;
- die kurze Stille zwischen dem Ende der ersten 8va-Teilgestalt und ihrem naechsten Einsatz unterbricht die Linie nicht;
- ein tatsaechliches dazwischenliegendes loco-Tonereignis bleibt weiterhin ein harter Grund fuer Stop und Neustart;
- die spaetere kurze Stille derselben 8va-Gestalt wird ebenfalls ueberbrueckt.

Diese Beschreibung ist nur Ergebnisbeobachtung. Keine Produktivregel kennt die konkrete Taktposition.

## Literatur- und Formatbezug

- Elaine Gould, *Behind Bars*: Oktavlinien sollen ueber kurze Pausen fortgesetzt werden koennen; bei ausgedehnteren Pausen sind getrennte Zeichen vorzuziehen. Unnoetige Unterbrechungen einer zusammenhaengenden Gestalt sind zu vermeiden.
- *Essential Dictionary of Music Notation*: kurze Pausen duerfen innerhalb einer Oktavlinie liegen; bei groesseren Pausen soll die Linie getrennt und neu eingefuehrt werden.
- Dokumentation externer Notensatzsoftware: Oktavlinien sind eigenstaendige zeitlich ausgedehnte Notationselemente und koennen ueber System- und Seitenumbrueche fortgesetzt werden.
- MusicXML 4.0: `octave-shift` trennt Start/Stop/Continue, `number` unterscheidet ueberlappende Kontrollelemente. Diese technische Identitaet besitzt kein Recht, die musikalische Sichtentscheidung umzudeuten.

Die Literatur liefert bewusst keine universelle numerische Grenze fuer „kurz“. r283 operationalisiert deshalb konservativ relativ zur unmittelbar angrenzenden rhythmischen Gestalt statt mit einer globalen magischen Zahl.

## Zielarchitektur

```text
RegisterNotationPlan          // Logik
        ↓
rhythmische Ereignisanker     // Geltung
        ↓
Sichtzustandspfad             // Sicht
        ↓
Sichtzielbild                 // 0 offene Sichtentscheidungen
        ↓
Sichtschreibauftrag           // weiterhin schreibfrei
        ↓
spaeter: MusicXML-Projektion  // kein musikalisches Entscheidungsrecht
```

## Stopregel

r283 ist abgeschlossen, wenn der reale Etuede-6-Lauf gleichzeitig nachweist:

- alle drei r282-Sichtentscheidungen sind nach allgemeinen Regeln entschieden;
- das Zielbild besitzt keine Takt-/Etuedensonderlogik;
- der direkte Darstellungswechsel bleibt erhalten;
- die beiden lokal kurzen stillen Unterbrechungen werden verbunden;
- daraus entstehen 16 sichtbare Zielspannen;
- Start-/Stopanker und Dokumentbereiche bleiben fuer alle Zielspannen exakt;
- Liniennummern bleiben aus Ueberlappung ableitbar;
- kein `default-y` wird vorentschieden;
- `MusicXMLWrite nein`;
- die aktuelle r277-PDF bleibt bis zum folgenden MusicMutation-Take visuell gesperrt.

Erst danach darf r284 genau dieses bereits feststehende Zielbild produktiv materialisieren und unmittelbar an Etuede 6 visuell geprueft werden. Ein Korpuslauf bleibt bis dahin ausdruecklich zurueckgestellt.
