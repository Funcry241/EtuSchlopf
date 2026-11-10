# Gemeinsame Registerplanung: Vergleichspolitik C4, Version 1

PolicyId: joint_register_dp_c4_v1
SupersedesComparisonPolicy: joint_register_dp_c3_v1
CausalClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
AcceptedClassDefinitionUnchanged: yes
Authorization: explicit reader agreement to register signs before three-ledger heads
NativeAcceptance: pending

## Vereinbarte Prioritaet

C4 bewertet vollstaendige, zulaessige Registerkurse lexikografisch in der
Reihenfolge

    (h8, h7, h6, h5, h4, ko, uk, ks, gb, ch, h3, h2, h1).

Gegenueber der unveraendert erhaltenen historischen C3-Spezifikation
`joint_register_dp_policy_v1.md` werden ausschliesslich die benachbarten
Prioritaeten `ch` und `h3` vertauscht. Ein Kurs mit weniger echten
Registerzeichen kann deshalb trotz zusaetzlicher Drei-Hilfslinien-Koepfe
gewinnen. Das ist eine ausdrueckliche Leser-/Projektpraeferenz, keine neue
wissenschaftliche Erkenntnis und keine empirisch kalibrierte Lesezeit.

`ch` behaelt seine Definition: Schluesselwechsel eins; Beginn oder Ende einer
wirksamen Oktavierung je eins; direkter Wechsel zwischen Nicht-loco-Zustaenden
Ende plus Beginn. Der erste Schluessel bleibt frei. Reine Erinnerungszeichen
sind keine Zustandsaenderung. Keine Operationskosten oder Gewichte aendern sich.

## Unveraenderte Bedingungen

Alle Definitionen, der vollstaendige Ausgangskurs, sichtbare und verborgene
Verpflichtungen, zulaessige Graphzustaende und Kanten, Originalidentitaeten,
Gruppen, Zeitrechnung, exaktes Abschnittsalter, Anschluesse, Faehigkeiten der
bestehenden Owner, Master/Render-Oktavgleichheit, Gleichstandsregeln und
Materialisierungs-/Readbackpflichten aus C3 bleiben bestehen.

Insbesondere darf ein sichtbarer Kopf weiterhin keine neue oder verschaerfte
Belastung ab vier Hilfslinien erhalten. Auch die Kostenprioritaeten h8 bis h4,
ko, uk, ks und gb behalten ihren Vorrang vor ch. C4 ist keine pauschale
Drei-Hilfslinien-Obergrenze und keine Erlaubnis, Stimmen, Tonhoehen, Dauer,
Sichtbarkeit, Akkorde, Balken oder Boegen zu aendern.

UNKNOWN und unvollstaendige Suchen behalten den vollstaendigen Ausgang.
Nur eine vollstaendige strikt bessere Alternative kann durch die bisherigen
Owner uebernommen werden. Es gibt keinen Laufzeitschalter zwischen C3 und C4.
Die historischen C3-Praefixe bestehender Typen und Dateinamen bleiben aus
Kompatibilitaetsgruenden erhalten; Diagnose und Dokumentation nennen die
aktive Policy und die tatsaechliche Komponentenreihenfolge.

## Nachweise

Der unveraenderte endliche Graph wird mit den neuen Kosten bewertet. Der
unabhaengige synthetische Exhaustivvergleich verwendet die neue explizite
Reihenfolge und prueft weiterhin Graphvollstaendigkeit, Kontextalter und
kanonischen Gleichstand. Eine synthetische Gegenprobe zeigt die erlaubte
Drei-Hilfslinien-Abwaegung und separat die weiterhin erzwungene Verweigerung
neuer Vier-Hilfslinien-Belastung. Korpusadressen sind keine Selektoren oder
normativen Tests.

Die native Abnahme mit frischen vollstaendigen XMLs und PDFs bleibt offen.
Ein berechnetes Optimum beweist keine tatsaechliche Materialisierung und keine
allgemeine menschliche Lesbarkeit oder Spielbarkeit.
