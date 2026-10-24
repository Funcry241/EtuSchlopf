# r577 – RegisterNotationLocalReadabilitySpanBoundaryClosure

## Ausgangspunkt

r577 basiert direkt auf dem real installierten und fuer Etuede Nr. 5 dunkelgruen
bestaetigten r576 `PersistedMusicXmlSchemaSchlopfValidationGateClosure`.
SchemaSchlopf r020 bleibt unveraendert und ist nicht Gegenstand dieses Takes.

Der offene musikalische Befund liegt im bestehenden Registernotations-Owner:
`plane_segment()` entscheidet bisher fuer jeden durch die tragende
Grenzenevidenz gebildeten Abschnitt genau eine Oktavdarstellung fuer den ganzen
Abschnitt. Diese Abschnittsgranularitaet kann lokal lesbare Registerinseln
verdecken und zugleich eine an sich richtige 8va-/15ma-Entscheidung ueber lokal
ungeeignete Randbereiche ziehen.

Die beobachteten Gegenformen sind allgemein:

- ein hoher lokaler Abschnitt bleibt ohne Oktavierung, obwohl die umgebenden
  Loco-Noten die Gesamtentscheidung neutralisieren;
- der Beginn einer 8va/15ma liegt vor dem lokal vorteilhaften Registerbereich;
- das Ende einer 8va/15ma liegt hinter dem lokal vorteilhaften Registerbereich;
- eine weniger als vier Tonobjekte kurze extreme Randinsel liegt unmittelbar
  vor oder nach einem bereits qualifizierten Nachbarplan und wird allein durch
  die bestehende Mindestmenge nicht erneut bewertet.

Es gibt keine Etueden-, Takt-, Pitch- oder Positionssonderregel.

## Unveraenderte musikalische Konstanten

r577 aendert weder die bestehende Bewertungsfunktion noch ihre Schwellen:

- `MINDEST_TONOBJEKTE = 4`
- `VERBESSERUNGSSCHWELLE = 8`
- `DOPPELOKTAV_BEZEICHNUNGSSTRAFE = 24`
- obere Lesbarkeitszone 60..84
- untere Lesbarkeitszone 36..60
- Unteres-System-Schluesselvorrang ab MIDI 69
- vorhandene Kandidaten 8va, 15ma, 8vb, 15mb

Die vorhandene quadratische Hilfslinien-/Registerlast bleibt die einzige
numerische Lesbarkeitsautoritaet.

## Lokale Intervallauswahl im bestehenden Owner

Innerhalb jedes bereits durch `evidenz_segmente()` qualifizierten Abschnitts
bildet r577 sichere Angriffsschnitte. Ein lokaler Schnitt ist nur zulaessig,
wenn

1. ein neuer Angriff beginnt und
2. kein vor dem Schnitt begonnenes Ereignis in diesen Angriff hineinragt.

Damit werden Akkorde und sustainende/ueberlappende Texturen nicht kuenstlich
auseinandergerissen.

Auf diesen Schnitten wird fuer jedes zulaessige Intervall dieselbe vorhandene
Loco-/Oktavkostenrechnung ausgewertet. Der Intervallnutzen ist exakt

`Loco-Lesbarkeitswert - Darstellungs-Lesbarkeitswert`.

Die 15ma/15mb-Bezeichnungsstrafe ist im Darstellungswert bereits enthalten.
Ein Intervall bleibt nur Kandidat, wenn die unveraenderte
`VERBESSERUNGSSCHWELLE` erfuellt ist.

Nicht ueberlappende Kandidaten werden dynamisch so ausgewaehlt, dass die Summe
dieser bereits vorhandenen Lesbarkeitsvorteile maximal ist. Bei exakt gleicher
Summe gilt ausschliesslich als deterministische Tie-Break-Reihenfolge:

1. weniger sichtbare Oktavspannen,
2. groessere strukturelle Abdeckung.

Es wird keine neue Gewichtung eingefuehrt. Der bisherige Gesamtabschnitt ist
selbst weiterhin ein zulaessiges Intervall und bleibt damit immer als alter
Kandidat in der Auswahl enthalten.

## Kurze Randinsel-Kontinuitaet

Die harte fachliche Mindestmenge von vier Tonobjekten bleibt bestehen: Eine
1..3 Ereignisse kurze Insel darf keine eigenstaendige neue Oktaventscheidung
erzeugen.

r577 erlaubt lediglich eine enge Kontinuitaetsprojektion an einen bereits
qualifizierten Nachbarplan. Eine solche kurze Randinsel wird nur uebernommen,
wenn

- sie lueckenlos klingend an den Nachbarplan anschliesst,
- keine fremde ueberlappende Ereignisoberflaeche im Kandidatenintervall liegt,
- die unveraenderte `entscheide_oktavdarstellung()`-Rechnung bei kuenstlich
  erfuellter Mindestmenge genau dieselbe Darstellung wie der Nachbarplan waehlt,
- und damit auch die bestehende Verbesserungsschwelle erfuellt bleibt.

Das entspricht der bereits in r511 etablierten Logik fuer klingende
Zwischeninseln, nun symmetrisch fuer kurze offene Randinseln. Die Randinsel
leiht sich nur die bereits bestehende Nachbar-Darstellung; sie erzeugt keine
neue Oktavart und keine neue Schwelle.

## Erwartete Wirkung

Die Closure ist auf die allgemeine Fehlerklasse der Oktavierungs-Spangrenzen
ausgerichtet:

- fehlende lokale Hochregisterspanne,
- zu frueher Start,
- zu spaeter Start bei einer kurzen extremen Randinsel,
- zu spaetes Ende,
- Span-Overreach ueber lokal unguenstige Registerbereiche.

Die konkreten menschlich beobachteten Stellen in Etuede Nr. 6 dienen nur als
PDF-Falsifikationsadressen. Sie erscheinen nicht im Produktivcode.

## Governance

`ChangeClass=MusicMutation`.

- keine neue Writer-/Repair-/Recovery-Authority
- keine neue MusicXML-Schreibfunktion
- keine neue Schwelle oder Kalibrierung
- keine Aenderung an SchemaSchlopf/XSD-Gate
- keine Aenderung an StructuralWriteAuthority
- keine Takt-/Etueden-/Pitch-Sonderlogik
- Master-Inhalt soll invariant bleiben; Render-MusicXML/PDF duerfen sich durch
  geaenderte Registernotationsspannen sichtbar aendern

Vor einem Korpuslauf muss zuerst Etuede Nr. 6 technisch gruen gerendert und die
bekannten Oktavierungsstellen menschlich im PDF geprueft werden.
