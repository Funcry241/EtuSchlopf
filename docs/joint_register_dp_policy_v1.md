# Gemeinsame Registerplanung: Vergleichspolitik C3, Version 1

PolicyId: joint_register_dp_c3_v1
ContractStage: comparison_policy_only
RuntimeChangeInThisTake: none
ParentRevision: r1548
CausalClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
AcceptedClassDefinitionUnchanged: yes
PolicyStatus: candidate_until_native_take_acceptance

Dieser Vertrag legt eine vorgeschlagene Projektkonvention fuer einen spaeteren
Umsetzungstake fest. Seine Annahme friert C3 als Vergleichspolitik ein. Er
installiert weder einen Selektor noch einen Planer, Writer oder neue
Reparaturautoritaet. Er ersetzt keine bestehende Fehlerklassendefinition.
Ein spaeterer Implementierungstake muss seinen Repair-Scope separat aus der
unveraenderten, im akzeptierten Parent vorhandenen kausalen Klasse ableiten.
Eine bereits vollstaendig geschlossene Vergleichsentscheidung ist kein positiver
Fehlerklassenfall allein deshalb, weil ein anderes Notenbild gefaellt.

## 1. Zweck, Invarianten und fachlicher Bereich

Schluessel und Oktavierungsdarstellung beeinflussen gemeinsam die geschriebene
Position derselben erhaltenen Tonobjekte. Verglichen werden vollstaendige
zulaessige Registerkurse eines quellengebundenen Bereichs in derselben
Entscheidungs- und Autoritaetsepoche. C3 bevorzugt nach den schwersten
Hilfslinienklassen und Koinzidenzen konventionelle Kombinationen vor der
Vermeidung kurzer Registerabschnitte. Diese Ordnung ist eine Projektkonvention,
keine empirisch kalibrierte Lesezeit und kein universelles Lesbarkeitsgesetz.

Erhalten bleiben klingende Tonhoehe, Schreibweise, Onset, Dauer, Reihenfolge und
Identitaet musikalischer Ereignisse, Sichtbarkeit, Rhythmus, Pausen, Akkorde,
Stimmen, Systemzuordnung, Balken, Halte- und Bindeboegen, Dynamik und alle
sonstigen nicht zum Registerkurs gehoerenden Inhalte. Insbesondere gibt es
keine Umverteilung zwischen Systemen und keine automatische Haendezuteilung.
Layoutentscheidungen und Enharmonik gehoeren nicht zum Eingriffsraum.

Die zugelassenen nominalen Registerzustaende je System sind G-Schluessel auf
Linie 2 oder F-Schluessel auf Linie 4, jeweils kombiniert mit loco, 8va, 8vb,
15ma oder 15mb. Vorzeichen bleiben unveraendert. Die fuenf Oktavierungen
entsprechen diatonischen Verschiebungen 0, +7, -7, +14, -14 von klingender zu
abgesenkter/angehobener geschriebener Position. MusicXML size=15 darf nicht als
size=8 interpretiert werden. Weitere Zustaende werden nicht still auf diese
Menge abgebildet.

Nominale Zulaessigkeit ist keine Materialisierungsbefugnis: Nur bereits
zustaendige, nachweislich unterstuetzte Register-/Schluesselautoritaeten duerfen
einen Zustand schreiben. Die Implementation muss die Zustandsmenge und jeden
Uebergang an deren tatsaechliche Faehigkeiten binden. Ungeklaerte Semantik,
Anker, Herkunft, gekoppelte Abhaengigkeit oder Autoritaet ergibt UNKNOWN und
belaesst den vollstaendigen Ausgangskurs. Ein externer XML-Prototyp darf keine
produktive Wahrheit oder nachtraegliche Reparaturschicht werden.

## 2. Eingabe und vollstaendiger Ausgangskurs

Verbindliche Eingabe ist der unveraenderte Kontext an den bestehenden fachlichen
Ownern RegisterNotationPlan und ClefReadabilityPrimaryCourse vor ihrer
Materialisierung. Vorhandene SourceEvent-/Attack-Identitaeten werden erhalten.
Eine Staff- oder Voice-Nummer ist kein Ersatz fuer musikalische Identitaet.
Der Vergleich bindet alle betroffenen sichtbaren Koepfe, Akkordmitglieder,
gebundenen Fortsetzungen und auch unsichtbaren Traeger sowie ihren Einfluss
auf Grenzen, Belegung und Anschluss. Unsichtbare Koepfe erhalten keine
sichtbare Hilfslinienlast, bleiben aber semantische Verpflichtungen.

Der echte, unveraenderte Ausgangskurs muss als vollstaendiger legaler Pfad
reproduzierbar sein, einschliesslich der tatsaechlichen Grenzen im Schweigen,
Zeitoffsets, Dokumentordnung bei gleicher Zeit, Ein- und Ausgangszustaende,
Oktavlinienenden und Wiederholungszeichen. Neue Grenzen erhalten ausdrueckliche,
quellengebundene Anker. Die Umsetzung darf nicht nur Noten-Onsets verwenden,
wenn dadurch vorhandene Grenzen verschwinden oder verschoben werden.

Die Projektion alter Zustaende auf Notenkopfsegmente ist als Diagnose erlaubt,
aber kein Ersatz fuer die Ausgangsfassung. Ausgang und Kandidat werden auf
demselben vollstaendigen Modell und Zeitraum bewertet. Eine projizierte Zahl
wird ausdruecklich als projiziert gekennzeichnet. Sie darf nicht mit einer
unprojizierten Wechselzahl als Verbesserung verglichen werden.

Eine unabhaengige Konfliktgruppe umfasst genau die nachgewiesen gekoppelten
Kurse eines vollstaendigen Parts bzw. seiner fachlich geschlossenen Domaene.
Getrennte Systeme duerfen nur getrennt optimiert werden, wenn keine
systemuebergreifende Abhaengigkeit die Fortsetzungen oder Kosten koppelt.
Andernfalls ist ein gemeinsamer Zustand erforderlich oder die Alternative
bleibt UNKNOWN. Unabhaengige Parts konkurrieren nicht um einen globalen Sieger.

## 3. Harte Schutzregel fuer sichtbare Koepfe

Fuer jeden erhaltenen sichtbaren Kopf n seien L0(n) und Lz(n) die exakten
Anzahlen benoetigter Hilfslinien unter Ausgang und Kandidat. Akkordkoepfe und
sichtbare gebundene Fragmente werden einzeln gezaehlt. Es werden weder
verschiedene Striche im Bild noch bloss Angriffe gezaehlt.

Eine Alternative wird ausgeschlossen, wenn es einen Kopf gibt mit

    Lz(n) >= 4 und Lz(n) > L0(n).

Damit wird kein Kopf neu auf vier oder mehr Hilfslinien gebracht und eine
bereits hohe Last nicht weiter erhoeht. Der unveraenderte Ausgang besteht
diesen Filter. Vier ist eine ausdrueckliche Projektkonvention, keine aus einer
Studie abgeleitete universelle Wahrnehmungsgrenze. Andere harte musikalische
und materielle Pflichten gelten zusaetzlich unveraendert. Ein Gruppenbruch ist
ein gezaehltes Kostenmerkmal; echte musikalische oder materielle
Grenzverletzungen werden dadurch nicht erlaubt.

## 4. Kostenvektor und eingefrorene Reihenfolge

Alle Merkmale zaehlen nach derselben festgelegten Interpretation; es gibt keine
gewichtete Summe und keine Pareto-Uebernahmeregel. Minimiert wird lexikografisch:

    (h8, h7, h6, h5, h4, ko, uk, ks, gb, h3, ch, h2, h1).

h1 bis h7 zaehlen sichtbare Koepfe mit genau dieser Hilfslinienanzahl. h8
zaehlt acht oder mehr; diese obere Klasse ist explizit zusammengefasst. Zusaetzlich
werden das ungekappte Maximum, das vollstaendige Histogramm, die ungekappte
Hilfsliniensumme und der Nenner sichtbarer Koepfe berichtet. Die obere
Klassenbuendelung darf im Report nicht als exakte Acht ausgegeben werden.

ko zaehlt pro System und Grenzzeitpunkt einen gleichzeitigen Wechsel des
Schluessels und der wirksamen Oktavabbildung. Getrennte XML-Elemente an derselben
musikalischen Grenze werden semantisch gemeinsam bewertet.

uk zaehlt sichtbare Koepfe unter G mit 8vb/15mb oder unter F mit 8va/15ma.
Diese Kombinationen sind teuer, nicht verboten. Die Konvention gilt unabhaengig
davon, ob das System oben oder unten liegt. Sie steht vor ks.

ks zaehlt jeden maximalen Abschnitt mit konstantem kombiniertem Registerzustand,
der kuerzer als eine notierte Taktlaenge ist, einschliesslich des ersten und
letzten. Ein einziger insgesamt kurzer Abschnitt wird genau einmal gezaehlt.
Ein Wechsel von nur einer der beiden Zustandskomponenten beendet den Abschnitt.
Ein Abschnitt mit genau einer Taktlaenge ist nicht kurz. Gleichzeitige
Nullzeit-Zwischenzustaende werden vor der Abschnittsbildung normalisiert.

Die Dauer wird exakt musikalisch bestimmt. Bei Taktwechseln ist die
Taktkoordinate das Integral der musikalischen Zeit geteilt durch die jeweils
gueltige nominelle Taktlaenge. Die Abschnittsdauer ist die Differenz dieser
Koordinate. Ein Auftakt oder verkuerzter Takt liefert nur seinen tatsaechlich
verstrichenen Anteil der nominellen Taktlaenge. Divisions und Taktwechsel werden
zeitgebunden ausgewertet; keine Roh-Tick-Konstante, letzte Taktlaenge oder
Gleitkomma-Rundung definiert einen allgemeinen Takt. Fehlende eindeutige
Zeitsemantik bleibt UNKNOWN. Die Bewertungsdomaene laeuft je aktivem System
vom ersten relevanten Kopf bis zum gemeinsamen, gebundenen Bereichsende.

Der Prototyp verwendet fuer die zehn gleichmaessigen Eingaben eine konstante
Taktlaenge. Seine Vereinfachung ist kein allgemeiner Parservertrag.

gb zaehlt pro System und Grenze einmal, wenn die Grenze eine nachweislich
gebundene Balkengruppe oder Haltekette auftrennt: erster Angriff < Grenze <=
letzter betroffener Angriff. Eine Grenze im Inneren mehrerer Gruppen zaehlt
hier einmal; die betroffenen Gruppen werden zusaetzlich berichtet. Die
Gruppenidentitaet und Reichweite stammen vom fachlichen Owner, nicht aus einer
privaten Heuristik nach Pitch oder gedruckter Voice-Nummer. Nicht aufloesbare
Kopplung bleibt UNKNOWN. Ein Zaehler ersetzt keine unveraenderte harte
Halte-, Akkord- oder Balkenpflicht.

ch zaehlt echte Registerzeichen-Operationen: Schluesselwechsel zaehlen eins;
Beginn oder Ende einer wirksamen Oktavierung zaehlt je eins. Ein direkter
Wechsel zwischen zwei nicht-loco Oktavierungen zaehlt Ende plus Beginn.
Der erste Schluessel ist frei, ein initiales Oktavierungszeichen sowie sein
notwendiges Ende werden gezaehlt. Reine continue-/Erinnerungszeichen sind
keine Zustandsaenderung und werden separat berichtet. Der Abschluss muss den
gebundenen Anschluss erhalten; keine kostenlose Rueckkehr wird unterstellt.

C3 ist genau diese feste Reihenfolge. Die Kosten-Schnittstelle darf spaeter
andere versionierte Politiken technisch tragen, aber der Umsetzungstake
bekommt keinen frei waehlbaren Laufzeitschalter fuer neue Rangfolgen. Eine
andere Rangfolge ist eine ausdrueckliche neue Policy-Transition. B+uk und C2
sind Vergleichsdiagnosen, keine still verfuegbaren produktiven Ausweichmodi.

## 5. Uebernahme und Gleichstand

Uebernommen wird nur eine vollstaendig nachgewiesene Alternative, deren
primärer C3-Vektor strikt kleiner als der Vektor des echten Ausgangskurses
ist. Ein Gleichstand belaesst den Ausgang. Fehlende Evidenz wird nicht als
Nullkosten oder Gewinn gedeutet. Die urspruengliche r1548-Skalarkostenregel
bleibt in diesem Vertragstake produktiv unveraendert; erst der gesonderte
Umsetzungstake darf die beschriebene C3-Regel beim zustaendigen Owner ersetzen.

Unter gleich guten, strikt verbesserten Alternativen bevorzugt die spaetere
Umsetzung zuerst die geringere Zahl geaenderter Kopfsegment-Zustaende,
anschliessend die lexikografisch kleinste kanonische Zustandsfolge. Die
Zustandsreihenfolge lautet G/loco, G/8va, G/8vb, G/15ma, G/15mb, F/loco,
F/8va, F/8vb, F/15ma, F/15mb. Ein Kopfsegment ist die gleichzeitige Kopfgruppe
eines Systems; seine Zustandsabweichung vom Ausgang zaehlt einmal, unabhaengig
von der Zahl seiner Akkordkoepfe. Das kanonische Wort durchlaeuft den
unveraenderlich definierten Graphen musikalisch chronologisch und bei
Gleichzeitigkeit in der gebundenen physischen Systemreihenfolge von oben nach
unten. Reine ID-/Labelumbenennung veraendert diese Reihenfolge nicht.
Diese Tie-Break-Regel nutzt keine Projekt-, gedruckte Takt-, Event-ID-,
Entdeckungs- oder Thread-Reihenfolge und ist synthetisch zu pruefen. Sie darf
den C3-Vektor nicht verschlechtern. Die Diagnosebilder sind konkrete optimale Prototyppfade,
keine Zusage identischer produktiver Bytes bei Kostengleichheit.

## 6. Exaktheit des Suchkerns

Der endliche Graph enthaelt alle im deklarierten Modell zulaessigen Kurse sowie
den echten Ausgangskurs. Zustand und Kanten muessen gleiche legale
Fortsetzungen und gleiche inkrementelle Folgen fuer zusammengelegte Praefixe
beweisen. Neben Schluessel/Oktavierung traegt der Zustand fuer ks die exakte
Zeit seit der letzten Zustandsgrenze, oberhalb der Mindestdauer gesaettigt.
Anschluss-, Gruppen- und simultane Abhaengigkeiten sind ebenfalls zu tragen,
soweit sie nicht bereits unveraenderlich in der Eingabe geschlossen sind.

Die Histogramm- und Operationsbeitraege sind additiv. ks wird beim Wechsel
und Abschluss verbucht. Auf einem vollstaendigen hinreichenden Zustand ist
diese lexikografische Ordnung erweiterungsvertraeglich. Ein bestes Label je
solchem Zustand genuegt; ein Label je blossen Registerzustand genuegt nicht.
Der Alterszustand kann deutlich mehr Labels erzeugen. Es gibt keine pauschale
Zusicherung einer polynomiellen Laufzeit fuer beliebige Abhaengigkeiten.

Vier Nachweise sind getrennt erforderlich: Vollstaendigkeit des Graphen,
Hinreichendheit des Zustands, sicheres Aussortieren und korrekter Abschluss.
Optimal bedeutet ausschliesslich optimal im deklarierten symbolischen Modell.
PDF-Geometrie, Lesezeit und eine allumfassende Notationsoptimalitaet sind damit
nicht bewiesen. Ein Ressourcenabbruch wird als unvollstaendige Suche gemeldet;
der Ausgang bleibt erhalten, ohne optimal-Behauptung.

## 7. Umsetzung, Materialisierung und letzte Schreibstelle

Die spaetere Implementation verwendet die bestehenden Register- und
Schluessel-Owner in einer gemeinsamen belegten Eingabeepoche. Nur der
gewaehlte semantische Kurs wird ueber die zustaendigen bestehenden
Materialisierer geschrieben. Receipt, Adoption, tatsaechliche letzte
Schreibstelle und finale Artefaktquelle werden gebunden und zurueckgelesen.
Keine nachtraegliche Python-/XML-Bereinigung und kein PDF-Patch.

Die symbolische Entscheidung reicht nicht fuer die Freigabe: Nach der
Materialisierung muessen Erhaltung, Kopfschutz, Rueckkehr und Zielfunktions-
Readback erneut stimmen. Spaetere Owner duerfen die angenommene Darstellung
nicht still ueberschreiben. Kollisionen oder fehlender finaler Nachweis
verhindern die behauptete Abnahme. Bestehende Release-Gates bleiben aktiv.

## 8. Nachweispflichten vor produktiver Abnahme

- Synthetische, corpusunabhaengige Positiv- und Negativfaelle fuer beide
  Schluessel, alle fuenf Oktavierungen, zwingende 15ma/15mb, Originalzustand
  ausserhalb unterstuetzter Faehigkeiten, Chords, Ties, Beams, Hidden Carrier,
  Pausen, Gleichzeitigkeit, Offsets, gleiche Zeit mit verschiedener XML-Ordnung,
  echten Anschluss und gekoppelte Systeme.
- Abschnittsfaelle: erster, letzter und einziger kurzer Abschnitt, genau eine
  Taktlaenge, mehrere kurze Abschnitte, Takt-/Divisionswechsel und Auftakt.
- Differenztest der tatsaechlichen produktiven Zielfunktion, Schutzregel und
  Abschlusskosten gegen unabhaengige Vollaufzaehlung kleiner Graphen. Erwartete
  Einzelkosten werden zusaetzlich unabhaengig festgelegt, damit derselbe
  Zaehlerfehler nicht auf beiden Seiten unentdeckt bleibt.
- Nachweis der echten unveraenderten Ausgangsfassung im Graphen, Kosten-
  Readback und UNKNOWN-/Gleichstands-/Ressourcenabbruch-Rueckfall.
- Verschiebungs-, Umbenennungs- und Duplikationsproben: unabhaengige gleichartige
  Domaenen bleiben unabhaengig; kein scoreweiter erster Gewinner oder
  mengenabhaengiger Schreibausloeser.
- Nachweis von erhaltenen klingenden Ereignissen und unveraenderten weiteren
  musikalischen Verpflichtungen nach Materialisierung; MIDI-Gleichheit ist
  eine Zusatzprobe und ersetzt insbesondere keine Identitaets-/XML-Erhaltung.
- Native Pflichtpruefungen und Regression ueber alle zehn aktuellen Etueden,
  vollstaendige maschinelle Erhaltungs-/Kopfschutzpruefung und frische finale
  XML/PDF-Artefakte. Bilder umfassen die bekannten Pruefstellen und eine
  dokumentierte Stichprobe weiterer Aenderungen; Bilder ersetzen keine
  flaechendeckenden maschinellen Pruefungen.

Korpusadressen und die C3-Prototypzahlen gehoeren ausschliesslich in die
getrennte Evaluationsnotiz. Sie sind weder Selektorbedingungen noch normative
Test-Fixtures, Grenzwerte oder zugesicherte produktive Ergebnisse.
