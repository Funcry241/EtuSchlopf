# Mathematik in EtuSchlopf

EtuSchlopf macht aus MIDI- und WAV-Aufnahmen von Klavieretüden Notensatz (MusicXML und PDF). Viele Teilaufgaben dabei sind Entscheidungen: In welchem System steht ein Ton? Welcher Schlüssel gilt? Wird ein Akkordton gezeigt oder verborgen? Diese Seite zeigt, wo wir dafür Mathematik einsetzen, woher sie stammt und wie belastbar sie heute ist.

**Stand:** r1560 auf der nativ angenommenen Basis r1559 (Oktober 2026). Historische Korpuszahlen sind jeweils bezeichnet. Die 20 Master-/Render-XML-Dateien von r1559 sind bytegleich zu r1556; die 39 PDF-Seiten sind bei 144 dpi pixelgleich.

## Ehrlichkeitsregel

Die **Werkzeuge** sind bekannte Mathematik. Eigene Beiträge gibt es auf zwei Ebenen: die **Modellierung** (welche musikalische Frage wir wie formulieren, mit welchen Gewichten und Schwellen) und drei **projekteigene Formeln**, die mehrere Werkzeuge zu einem Ganzen verbinden: die SchlopfFormel, die Schlöpfchen-Formel und die symbolische Spielformel. Neue mathematische Sätze beanspruchen wir nicht. Jede Zeile ist entsprechend gekennzeichnet:

| Kennzeichen | Bedeutung |
|---|---|
| **bekannt** | Standardmathematik oder Standardinformatik, mit Lehrbuchquelle |
| **eigene Modellierung** | bekanntes Werkzeug, aber Zielgröße, Gewichte oder Schwellen von uns festgelegt; nicht allgemein erprobt |
| **eigene Konstruktion** | projekteigene Formel, die mehrere Bedingungen zu einem Ganzen verbindet; in der Tradition bekannter Ansätze, aber in dieser Form unsere; nicht extern begutachtet |

Der **Status** sagt, was die Mathematik heute tut:

| Status | Bedeutung |
|---|---|
| **entscheidet** | im Produkt aktiv, beeinflusst die Noten |
| **misst** | im Produkt aktiv, nur Bericht, ändert nichts (AuditOnly) |
| **Prüfwerkzeug** | externes Leseskript für Analysen, nicht Teil des Produkts |
| **geplant** | entworfen, noch nicht umgesetzt |

## Leitidee: Notation als geordnete Entscheidungsfolge

1. **Ein Entscheider je Größe.** Schreibweise, Register (Schlüssel und Oktavierung), System, Stimme, Balken und Layout haben je genau eine Stelle, die entscheidet. Alle anderen Stellen lesen nur.
2. **Topologische Reihenfolge.** Entschieden wird in der Reihenfolge der Abhängigkeiten, also Schreibweise → Register → Layout. Wie viele Hilfslinien ein Ton braucht, hängt von seiner Schreibweise ab: Es3 und Dis3 sind derselbe Ton, liegen aber auf verschiedenen Linien.
3. **Invarianten am Ende.** Was eine Stufe nicht ändern darf, wird als Invariante formuliert und nach der Stufe geprüft.

*Herkunft:* bekannt (gerichtete azyklische Graphen und topologisches Sortieren; Invarianten im Sinne von Hoare). *Status:* Prinzip. An der Registerentscheidung (C3) ist es noch nicht durchgesetzt: Ein erster Versuch scheiterte gerade daran, dass zwei Stellen über dieselbe Größe entscheiden.

## Die drei projekteigenen Formeln

### SchlopfFormel: wann eine Änderung zulässig ist

**Leitgedanke:** Eine Veränderung am Notentext ist nur zulässig, wenn drei Dinge gebunden sind: ihre **Herkunft** (welcher Zustand vorher galt), ein **zuständiger Ausführer** (genau ein Owner) und ein **Nachweis** für genau diesen Übergang. Eine einzelne kanonische Gesamtgleichung unter diesem Namen gibt es im Code nicht. Die SchlopfFormel ist die Zusammenfassung mehrerer ausführbarer Bedingungen. Ihr derzeit schärfster Teilkern ist die Übernahmeregel für ein Änderungsbeleg $e$:

$$\mathrm{Permit}(e) = [p(e)=1] \;\land\; [u(e)+d(e)+i(e)=0] \;\land\; \neg\mathrm{Replay}(e)$$

Dabei ist $p$ die Zahl der erlaubenden Einstufungen dieses einen Belegs (keine Wahrscheinlichkeit), und $u, d, i$ zählen fehlende, doppelte und ungültige Nachweise. Ein Beleg, der schon übernommen wurde, wird als Wiederholung abgewiesen. Dazu kommen:
- lückenlose Belegkette: jeder Übergang ist von genau einem Beleg gedeckt;
- Bindung an den tatsächlichen Vor- und Nachzustand;
- getrennte Herkunftsketten für Master und Render;
- Identität durch **Prägung** beim Erzeugen, nie durch Inhaltsvergleich, denn denselben Inhalt kann jeder erzeugen.

*Herkunft:* eigene Konstruktion. Verwandt mit beweistragendem Code (*proof-carrying code*, Necula 1997), Herkunftsnachweisen (*provenance*) und Fähigkeitstypen (*capabilities*). *Status:* entscheidet (`adoption_decision` und `ssc_adoption_verdict` in `src/ssc_evidence_proof.rs`; Belegkette in `src/master_state_identity.rs`).
*Grenze:* Die SchlopfFormel sichert, **dass** eine Änderung erlaubt und belegt ist, nicht, **dass** sie musikalisch gut ist.

### Schlöpfchen-Formel: wie Seiten gefüllt werden

**Aufbau (vereinbarte Formulierung):**
- Getrennte Erfassung je Takt: horizontales und vertikales Notationsprofil, gleichzeitig notierte Köpfe.
- Daraus werden Systemhüllen gebildet, aus Systemhüllen Seiten. Ungenutzter Platz wird beobachtet.
- **Lesbarkeit ist Voraussetzung**, kein Zielwert.
- Unter den zulässigen Kandidaten wird **lexikografisch** gewählt:

$$\min_{\text{zulässig}}\ \big(\text{Seitenzahl},\ \text{übermäßige Dehnung},\ \text{Ausgewogenheit}\big)$$

- Die Reihenfolge der Systeme bleibt erhalten.
- Beobachtete Lücken aus echten Stücken sind keine Mindestmaße.
- Horizontal und vertikal sind getrennte Dimensionen.

*Herkunft:* eigene Konstruktion. Verwandt mit dem Zeilenumbruch nach Knuth und Plass (1981) und mit Packungsproblemen (*bin packing*). *Status:* **geplant.** Eingefroren ist bisher nur die Fehlerklasse „unnötig beibehaltene Seite“ (r1541, `docs/selector_derivation_contract_redundant_page_retention.md`). Die Formel selbst ist nicht implementiert, und die Mindestbreiten und -abstände sind noch nicht verifiziert.

### Symbolische Spielformel: was eine Stelle von den Händen verlangt

$$\exists\ \text{Belegung}: \ \text{Struktur} \land \text{statische Anforderungen} \land \text{Übergänge} \land \text{Wiederanschlag gleicher Taste}$$

Die Variablen sind die physischen Verpflichtungen einer Stelle (welche Töne gleichzeitig, welche nacheinander). Der Wertebereich sind zehn Hand-/Finger-Plätze. Aus einer Belegung werden statische, Übergangs- und Wiederanschlagsanforderungen abgeleitet. Diese **Anforderungen** werden von den **Fähigkeiten** eines Pianistenmodells getrennt gehalten. Ein nicht modellierter Fähigkeitswert wird nie als „kann“ gelesen.

*Herkunft:* eigene Konstruktion auf Basis von Erfüllbarkeit (Constraint Satisfaction, SAT). *Status:* misst (`src/audit_only/pianistic_symbolic_capability_reachability.rs`). Das Referenzmodell ist ausdrücklich `MODEL_RELATIVE_ONLY`: Es beweist weder Spieltempo noch Spielbarkeit für einen konkreten Menschen. Literatur zu Handspanne und Klaviaturgröße begründet die Trennung von Anforderung und Fähigkeit, kalibriert aber keine Schwellen.

## Übersicht

| # | Frage | Mathematik | Herkunft | Status |
|---|---|---|---|---|
| 1 | Geht ein Ton verloren? | Erhaltungssatz (Bilanzidentität) | bekannt | misst |
| 2 | Ist die Notation in sich stimmig (Schlüssel, Oktavierung, Haltebögen)? | endliche Zustandsautomaten, Invarianten R1–R5 | bekannt; Regeln: eigene Modellierung | misst |
| 3 | Wie viele Stimmen braucht ein Takt mindestens? | Intervallgraph, Cliquenzahl ω | bekannt | misst |
| 4 | Hat die Spielbarkeitsmessung alles gesehen? | Abdeckungsinvariante | bekannt | misst |
| 5 | Wohin verteilt man verborgene Akkordtöne? | kleinste Quadrate (Gauss) mit gewichteter Zielfunktion | Prinzip bekannt; Zielfunktion: eigene Modellierung | entscheidet |
| 6 | Welcher Lösungskandidat ist der beste? | lexikografische Ordnung | bekannt | entscheidet |
| 7 | Wie schwer ist ein Ton mit $l$ Hilfslinien zu lesen? | Druckfunktion $p(l)$, genutzt von zwei Schlüsselplanern | eigene Modellierung | entscheidet |
| 8 | Welcher Schlüsselverlauf im unteren System? | dynamische Programmierung über Zustände (Lesbarkeitskurs); gemeinsame Optimierung mit Oktavierung nur als Prototyp | bekannt; Kosten: eigene Modellierung | entscheidet (Schlüssel); Prototyp (Schlüssel + Oktavierung) |
| 9 | Legt die Pipeline getrennte Anschläge zusammen? | Injektivität modulo Quellgleichheit | bekannt; Formulierung: eigene Modellierung | misst (R6-Bericht); Platzierungsregel geplant |
| 10 | Was hat sich zwischen zwei Versionen geändert? | Differenz je Größe auf einer Identitätsabbildung | eigene Modellierung | Prüfwerkzeug |
| 11 | Wie viele Töne muss man mindestens weglassen? | monotone Zweiteilung, Untergrenze | eigene Modellierung | Prüfwerkzeug |
| 12 | Ist ein Zustand unverändert? Wer hat ihn erzeugt? | Hashfunktionen (FNV-1a 64, SHA-256) | bekannt | entscheidet (Prüfschranken) |
| 13 | Wann darf man aufbewahrte Daten verwerfen? | Beobachtungsäquivalenz (Projektion) | bekannt | umgesetzt seit r1559 |
| 14 | Woher kommt der Speicherbedarf? | lineare Regression | bekannt; Modell: eigene Auswertung | Hypothese |
| 15 | Wann ist eine Änderung zulässig? | **SchlopfFormel** | eigene Konstruktion | entscheidet |
| 16 | Wie werden Seiten gefüllt? | **Schlöpfchen-Formel** (lexikografische Seitenpackung) | eigene Konstruktion | geplant (Fehlerklasse eingefroren) |
| 17 | Was verlangt eine Stelle von den Händen? | **symbolische Spielformel** (Existenzaussage über Belegungen) | eigene Konstruktion | misst |
| 18 | Wie werden Zeiten exakt verrechnet? | ggT/kgV, kanonische Zeitachsen, metrische Zerlegung stiller Intervalle | bekannt | entscheidet |
| 19 | Welche harmonische Lesart? | Kostenvektoren, Pareto-Vergleich, begrenzte Suche auf lokalen Kandidatengraphen | bekannt; Kosten: eigene Modellierung | entscheidet (durch Owner begrenzt) |
| 20 | Wann ersetzt ein Registerplan einen anderen? | Vergleich gegen den vollständigen eingefrorenen Basisplan (gemeinsame Spanne) | eigene Modellierung | entscheidet |
| 21 | Passt die Aufnahme (WAV) zur MIDI? | neuronales Transkriptionsmodell (Basic Pitch) als unabhängiger Sensor | bekannt (fremdes Modell) | misst (separates Werkzeug, keine Schreibbefugnis) |

## Die Werkzeuge im Einzelnen

### 1. Erhaltungsbilanz

Jeder Ton der Quelle wird genau einmal verbucht:

$$\text{Quelle} = \text{aktiv} + \text{inaktiv}, \qquad \text{aktiv} = \text{sichtbar} + \text{verborgen}$$

Im Korpus (r1556): $10\,149 = 9\,986 + 163$ und $9\,986 = 7\,297 + 2\,689$. Damit sind knapp 27 % der aktiven Anschläge im Master verborgen, meist Akkordtöne, die zugunsten der Spielbarkeit nicht gezeigt werden.

Die Identität gilt in jeder Etüde exakt. Bricht sie, ist ein Ton verloren gegangen oder doppelt verbucht.
*Herkunft:* bekannt (Erhaltungssätze, doppelte Buchführung). *Status:* misst (`src/audit_only/material_conservation.rs`).

### 2. Notationsinvarianten R1–R5

Je System wird die Notenfolge in Quellzeit mit einem kleinen Zustandsautomaten gelesen. Die Regeln:

| Regel | Verletzung |
|---|---|
| R1 | redundanter Schlüssel (gleich dem geltenden) |
| R2 | Schlüsselabschnitt ohne sichtbaren Notenkopf |
| R3 | Oktavierung ohne Ende oder Ende ohne Anfang |
| R4 | Oktavierung ohne sichtbaren Notenkopf |
| R5 | Haltebogen je Taste: neuer Anschlag, solange ein Bogen dieser Taste offen ist; Bogenende ohne offenen Bogen; offener Bogen am Stückende |

Stand r1556: R1–R4 zusammen eine Meldung (R2 in Etüde 7), R5 keine.
*Herkunft:* bekannt (endliche Automaten); die Regeln sind eigene Modellierung. *Status:* misst (`src/audit_only/notation_invariants.rs`, Zeile `NotationInvariants`).

### 3. Stimmenbedarf ω

Je System und Takt bilden Noten mit gleichem Einsatz und gleicher Dauer ein Ereignis, also das Intervall $[\text{Einsatz}, \text{Einsatz} + \text{Dauer})$. Die Mindestzahl an Stimmen ist die größte Zahl gleichzeitig überlappender Ereignisse, die **Cliquenzahl ω** des Intervallgraphen. Intervallgraphen sind perfekt, deshalb gilt Cliquenzahl = chromatische Zahl. Eine gierige Färbung nach Beginn ist optimal.

Stand r1556, Master, alle Noten: $\omega = 1$ in 732 und $\omega = 2$ in 8 System-Takten. Benutzt werden aber in 325 System-Takten mehr Stimmenbezeichnungen als nötig (Render: 97). Das ist ein Hinweis auf Vereinfachungspotenzial, kein Fehler im strengen Sinn.
*Herkunft:* bekannt (Intervallgraphen; Gilmore/Hoffman, Golumbic). *Status:* misst (Zeilen `VoiceDemand`).

### 4. Abdeckung der Spielbarkeitsmessung

$$\text{gemessene Ereignisse} = \text{aktive Anschläge}$$

Weicht das ab, meldet der Bericht den Grund. Beispiel: In Etüde 7 sah die Messung wegen eines Haltebogenfehlers 0 statt 1.042 Ereignisse. Seit r1556 sind es 1.042.
*Herkunft:* bekannt. *Status:* misst (`src/audit_only/material_reference_coverage.rs`).

### 5. Gauss-Verteilung verborgener Akkordtöne

Ein verborgener Akkordton $t$ wird, wenn das musikalisch günstig ist, an einen nahen sichtbaren Ankerplatz $s$ gelegt. Gewählt wird der Platz mit der kleinsten gewichteten Quadratsumme:

$$E(t,s) = 3\,\Delta t^2 + 24\,\Delta p^2 + 42^2\,[\text{System} \ne] + 12^2\,[\text{Stimme} \ne] + 2\,(\Delta t - 24k)^2 + 12\,000\,[\text{Platz in dieser Verteilung schon vergeben}]$$

Dabei ist $\Delta t$ der Zeitabstand in Ticks (96 je Viertel), $\Delta p$ der Tonhöhenabstand in Halbtönen und $k$ die laufende Nummer in der Gruppe (eine Art gleichmäßiges Auffächern).

Seit r1556 gilt zusätzlich eine harte Bedingung: Ein Platz, an dem im selben System und in derselben Stimme schon dieselbe Taste klingt, ist ausgeschlossen.

*Herkunft:* Das Prinzip der kleinsten Quadrate ist bekannt (Gauss, Legendre). Gewichte und Strafterme sind eigene Modellierung und nicht kalibriert. *Status:* entscheidet (`src/pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs`).
*Bekannte Grenze:* Die Bewertung von Varianten nutzt die harte Bedingung noch nicht, die Ausführung schon. Siehe offene Fragen.

### 6. Lexikografische Kandidatenwahl

Die Suche erzeugt viele Lösungskandidaten. Verglichen werden sie über einen Schlüssel, der der Reihe nach geordnet ist:

$$(\text{Blocker},\ \text{kritische Befunde},\ \text{max. Härte},\ \text{Warnungen},\ \text{Hinweise},\ -\text{Erhalt},\ \dots)$$

Ein späteres Kriterium zählt nur bei Gleichstand aller früheren. So kann zum Beispiel kein Gewinn bei den Hinweisen einen Blocker aufwiegen.
*Herkunft:* bekannt (lexikografische Ordnung, Prioritätenoptimierung). *Status:* entscheidet (`CandidateDecisionKey`).

### 7. Druckfunktion für Schlüsselwechsel

Für einen Ton mit $l$ Hilfslinien:

$$p(l) = l^2 + 8\,\max(l-3,\ 0)$$

Der quadratische Term bestraft viele Hilfslinien stärker. Der Zusatzterm macht mehr als drei Hilfslinien deutlich teurer. Dieselbe Funktion steht an **zwei** Stellen im Code:

- im **Lesbarkeitskurs** (Nr. 8, `src/structural_write_authority/rustig_clef_planning.rs`) als Kostenanteil je Ton;
- im **Hilfslinien-Readback** (`src/notation.rs`, `ledger_bounded_lower_staff_clef_readback`): Er fügt einen Schlüssel ein, sobald ein Ton im unteren System mehr als drei Hilfslinien bräuchte. Im Render-Pfad schlägt er außerdem ganztaktige Wechsel vor, wenn ein Takt mindestens 3 Anschläge und mindestens 2 Viertel betroffene Dauer hat und der Druck um mindestens 3 sinkt.

*Herkunft:* eigene Modellierung. *Status:* entscheidet.
*Bekannte Grenze:* Zwei Stellen schreiben Schlüssel, nach der Leitidee darf es nur einen Entscheider geben. Beide sind im Korpus sichtbar wirksam (siehe „Was wirkt auf das Notenbild?“).

### 8. Schlüsselverlauf als kürzester Weg (dynamische Programmierung)

**Im Produkt (Lesbarkeitskurs, Schlüssel im unteren System):** Der Zustand ist (aktiver Schlüssel, Ursprung eines kurzen Ausflugs, Anschläge im Ausflug). Entlang der Zeitachse wird eine Front optimaler Teilwege fortgeschrieben (`solve_readability_course_with_boundaries`). Kosten eines Verlaufs:

$$C = \sum_{\text{Töne}} p(l) + 24\cdot\#\text{Wechsel} + 1\cdot\#\text{redundante Schlüssel} + 24\cdot\#\text{kurze Ausflüge}$$

Ein kurzer Ausflug umfasst höchstens 2 Anschläge.
*Herkunft:* bekannt (Bellman; Viterbi-artige Vorwärtsrechnung). Zustandsraum und Gewichte sind eigene Modellierung. *Status:* entscheidet.

**Als Prototyp (gemeinsam mit Oktavierung):** Zustand Schlüssel $\in \{G, F\}$ × Oktavierung $\in \{0, +8, -8\}$, mit lexikografischem Kostenvektor je Takt (Kollisionen, Hilfslinien-Histogramm von schwer nach leicht, Wechselzahl). *Status:* Prüfwerkzeug und Entwurf für die gemeinsame Registerentscheidung (C3). Ein erster Versuch scheiterte, weil die Schreibweise erst danach entschieden wurde und weitere Schlüsselschreiber folgten.

### 9. Keine neue Koinzidenz

Für zwei Anschläge $a, b$ mit Rohposition $r$ aus der Quelle und Platzierung $\pi$ in der Notation:

$$\pi(a) = \pi(b) \ \Rightarrow\ r(a) = r(b) \qquad \text{mit } \pi, r \in (\text{Einsatz}, \text{MIDI-Taste})$$

Die Zuordnung soll also injektiv sein, **außer** wenn die Gleichheit schon in der Quelle steht (Quell-Unisono). System und Stimme spielen keine Rolle, damit ein Systemwechsel einen Fall nicht verdeckt.

Stand: 16 Verletzungen in r1555, 3 in r1556, keine neuen. Quell-Unisoni (r1555: 6, r1556: 4) werden getrennt geführt.
*Herkunft:* bekannt (Injektivität, Äquivalenzrelationen); die Formulierung als Abnahmeregel ist eigene Modellierung. *Status:* misst als R6-Bericht ab r1560 (`src/audit_only/material_coincidence.rs`); als gemeinsame Regel für alle Platzierer geplant. Der Bericht vergleicht gebundene Quellanschläge mit platzierten Anschlägen auf der typisierten Notationszeitachse. Vorschlagsnoten werden separat als ausgeschlossen gezählt; Fortsetzungen sind keine eigenen Anschläge. Erhaltene Quell-Unisoni werden getrennt gezählt. Herkunftsbeleg-Mitgliedschaften allein beweisen keinen einzelnen Verursacher. Fehlende oder mehrdeutige Eingaben ergeben unbekannt, niemals null Verletzungen.

### 10. Änderungsmatrix zwischen Versionen

Jede Note erhält die Identität (Takt, Einsatz, MIDI, $j$), wobei $j$ die laufende Nummer bei gleichen ersten drei Werten ist. Dann wird je Größe gezählt, wie viele Noten sich ändern: Bestand, Dauer, Sichtbarkeit, Schreibweise, System, Stimme, Register, Bogen, Balken.
*Herkunft:* eigene Modellierung. *Status:* Prüfwerkzeug.
*Bekannte Grenze:* Bei gleichen Tönen im gleichen Einsatz ist die Zuordnung über $j$ nicht identitätstreu. Ein Multimengenvergleich, der nur eindeutige Paare zuordnet, ist dort genauer.

### 11. Untergrenze für weggelassene Töne

Je Einsatzgruppe wird geprüft, ob sich die Töne monoton auf zwei Hände teilen lassen: links tiefer als rechts, je Hand höchstens eine Spannweite und höchstens fünf Töne. Was übrig bleibt, muss mindestens weggelassen werden. Sprünge zwischen den Gruppen werden ignoriert, deshalb ist es eine **Untergrenze**. Gerechnet auf r1556 liegt sie bei 99 bis 140 von 9.986 Anschlägen, also 1,0–1,4 %. Der Wert hängt von der Spannweite (12 oder 14 Halbtöne) ab und davon, ob gehaltene Töne mitzählen. Das liegt weit unter dem verborgenen Anteil von knapp 27 %. Woraus sich der Unterschied zusammensetzt (Sprünge zwischen Gruppen, Lesbarkeit, Regeln der Akkordreduktion), ist nicht weiter untersucht. Die Untergrenze ist ausdrücklich keine Aussage darüber, was spielbar ist.
*Herkunft:* eigene Modellierung. *Status:* Prüfwerkzeug.

### 12. Hashing und Identität

Unveränderlichkeit wird über Prüfsummen festgestellt: FNV-1a 64 Bit für schnelle Schranken, SHA-256 für Zustände und Pakete. Identität entsteht durch Prägung beim Erzeugen eines Zustands, nicht durch Inhaltsvergleich, denn denselben Inhalt kann jeder erzeugen.
*Herkunft:* bekannt (Fowler/Noll/Vo; FIPS 180-4). *Status:* entscheidet (Annahme und Ablehnung von Änderungen).

### 13. Beobachtungsäquivalenz beim Aufbewahren

Ein aufbewahrter Datensatz $D$ darf auf eine Projektion $\pi(D)$ reduziert werden, wenn jeder später noch erreichbare Leser $L$ nur über $\pi$ zugreift:

$$L(D) = L(\pi(D)) \quad \text{für alle erreichbaren Zustände}$$

Hinreichend ist eine nach dem Einfügen unveränderliche Eigenschaft, die jeden vollen Zugriff ausschließt. Beispiel: Kandidaten mit Blockern oder Warnungen kann die spätere Auswahl nie übernehmen. Im historischen Pool von Etüde 6 erfüllen 1.833 von 1.834 Einträgen dieses Ausschlusskriterium. Das ist eine Obergrenze für reduzierbare Einträge: r1559 hält den aktuellen und alle vorherigen Gewinner vorsorglich vollständig, ebenso alle kompatiblen Kandidaten. Reihenfolge, Gleichstand und Prüfzeilen bleiben erhalten.
*Herkunft:* bekannt (Beobachtungsäquivalenz, Lebendigkeitsanalyse). *Status:* umgesetzt seit r1559 (`src/pipeline_shards/solver_candidate_retention.rs`). Nativ bestätigt: 20 XML bytegleich, 39 PDF-Seiten pixelgleich bei 144 dpi, Kandidaten- und Auswahlzeilen unverändert. Eine quantitative Speicherersparnis wurde nicht isoliert gemessen; Otter akzeptiert die beobachtete Speichernutzung.

### 14. Speichermodell (Hypothese)

Spitzenbedarf je Etüde in GB, angepasst an zehn gemessene Läufe:

$$M \approx 0{,}0054\, F S + 0{,}0061\, G S$$

$F$ ist die Zahl der aufbewahrten Kandidaten, $G$ die Zahl der neu erzeugten Master-Zustände, $S$ die Dokumentgröße in MB. $R^2 = 0{,}999$, aber nur 10 Punkte für 3 Parameter. Der $F$-Term hängt fast allein an einer Etüde, und $G S$ ist stark mit der Laufzeit korreliert. Das Modell ist eine Vorhersage, kein Ursachenbeweis.
*Herkunft:* bekannt (kleinste Quadrate); das Modell ist eigene Auswertung. *Status:* Hypothese.

### 15–20. Weitere Bestandteile (Kurzfassung)

- **Exakte Zeitordnung:** Alle Dauern werden mit ggT/kgV auf eine gemeinsame Teilung gebracht, ohne Gleitkomma. Stille Intervalle werden entlang der metrischen Hierarchie des Takts zerlegt. *Ort:* `src/model/metric_rest_notation.rs`.
- **Harmonik-Kandidaten:** Kostenvektoren und Pareto-Vergleich: Ein Kandidat schlägt einen anderen nur, wenn er in keiner Komponente schlechter und in mindestens einer besser ist. *Ort:* `src/pipeline_shards/sheaf_noether_projection_core.rs`. Dateinamen wie *sheaf* oder *noether* sind dabei **keine** Belege für einen bewiesenen Satz aus Garben- oder Symmetrietheorie. Belegbar sind nur die konkreten Operationen.
- **Gemeinsame Spanne:** Ein neuer Registerplan ersetzt den alten nur innerhalb eines geschlossenen Konfliktbereichs, mit Pflichtabdeckung, echten Ankern und Prüfung der Haltebögen. *Ort:* `src/registernotation/gemeinsame_spanne.rs`. Mit r1512 wurden so in 7 Stücken 16 bessere Alternativen bestätigt. In Etüde 4 sank die höchste Hilfslinienzahl von 5 auf 3.

## Was wirkt auf das Notenbild?

Die Spalte „Status“ sagt, was eine Stelle **darf**. Ob sie im Korpus auch **tatsächlich** etwas am Notenbild ändert, steht in dieser Tabelle. Belege aus den zehn Etüden (r1556):
- Herkunftsbelege `MaterialMutationReceipt` je Owner;
- Kennzeichnungen, die die Schreiber in die MusicXML setzen;
- Berichtszeilen.

| Mathematik | Wirkung | Beleg im Korpus |
|---|---|---|
| Gauss-Verteilung (5) | **sichtbar**: verschiebt Töne in Zeit, System, Stimme | GaussChain 32 Anschläge (alle 10 Etüden), ConstrainedTail 9 (5 Etüden) |
| Gesten-Verteilung (BrokenGesture, eigene Bewertungsfunktion, wie 5) | **sichtbar**: verschiebt Töne in der Zeit | 27 Anschläge (alle 10 Etüden) |
| Lesbarkeitskurs (8) mit $p(l)$ (7) | **sichtbar**: Schlüssel im unteren System | gekennzeichnete Stellen in 9 von 10 Etüden. **Alle** Schlüssel, die nur im Render und nicht im Master stehen, stammen von hier (Etüden 4, 5, 10) |
| Hilfslinien-Readback mit $p(l)$ (7) | **sichtbar**: Schlüssel im unteren System | gekennzeichnete Stellen in 6 Etüden; im Render von Etüde 6 und 10 je eine zusätzlich |
| Gemeinsame Spanne (20) | **sichtbar**: Oktavierungen und Schlüssel | 16 von 234 Vergleichen übernommen, in 7 Etüden |
| Lexikografischer Kandidatenschlüssel (6) | **mittelbar**: wählt, welche Fassung weiterläuft | in jedem Lauf; Einzelwirkung nicht isolierbar |
| Exakte Zeitrechnung (18) | **sichtbar**: Pausen und Dauern | in jedem Lauf |
| Harmonik-Portfolio, Pareto (19) | **darf** Tonhöhen tauschen | **im Korpus nie übernommen**: Der Beleg deklariert Tonhöhe, System und Stimme als mögliche Wirkung, und alle drei Zähler sind in allen 10 Etüden 0 |
| SchlopfFormel (15), Hashing (12) | **schützt**: lässt Änderungen zu oder weist sie ab; wählt selbst keine Noten | wirkt bei jeder Übernahme |
| Bilanz (1), R1–R5 (2), ω (3), Abdeckung (4), Spielformel (17), WAV-Sensor (21) | **nur Bericht** | einziger Leser im Produkt ist der Berichtsschreiber (`final_report_projection_owner.rs` bzw. `report.rs`). Die Spielformel hat außerhalb des Audit-Bereichs keinen Leser |
| Koinzidenz (9) | **nur Bericht**: keine Platzierung und keine Freigabebefugnis | R6-Bericht ab r1560; native Gegenprobe noch ausstehend |
| Änderungsmatrix (10), Untergrenze (11), Speichermodell (14) | **keine**: externe Prüfwerkzeuge | – |
| Beobachtungsäquivalenz (13) | **Speicheraufbewahrung**: musikalische Ergebnisse unverändert | r1559 nativ angenommen |
| Schlöpfchen-Formel (16) | **keine**: geplant | – |

**Folgerung:** Sichtbar wirken heute fünf Mathematik-Bausteine:
- zwei Verteiler (Gauss und Geste);
- zwei Schlüsselplaner, die dieselbe Druckfunktion nutzen;
- die gemeinsame Spanne.

Dazu kommt mittelbar die Kandidatenwahl. Alle Schwellen und Gewichte, die diese Bausteine steuern, sind eigene Setzungen (siehe nächster Abschnitt). Dort lohnt sich mehr Mathematik am meisten.

## Was ist nicht bekannte Mathematik?

Ehrlich gesagt: **kein Satz und kein Verfahren auf dieser Seite ist mathematisch neu.** Neu im Sinne von „so gibt es das woanders nicht“ sind drei Dinge:

1. **Die drei projekteigenen Formeln** (SchlopfFormel, Schlöpfchen-Formel, symbolische Spielformel). Ihre Bausteine sind bekannt (Boolesche Bedingungen und Zähler, lexikografische Optimierung, Erfüllbarkeit). Die Zusammensetzung für den Notensatz und für eine Entwicklung, die KI durchführt, ist unsere.
2. **Die Abnahmeregeln für Notensatz-Änderungen:** Erhaltungsbilanz plus „keine neue Koinzidenz“ plus Invarianten R1–R5 plus Änderungsmatrix als **gemeinsamer Abnahmetest**. Jede einzelne Prüfung ist Standard. Ihr Zusammenspiel als Gegenprobe ist es nicht: Im Korpus geht die Zahl der XML-Doppeltöne exakt in Koinzidenzen, Quell-Unisoni und Fortsetzungen auf (23 = 16 + 6 + 1).
3. **Die Gewichte und Schwellen** (Gauss-Bewertung, Druckfunktion, Schlüsselwechsel-Schwellen). Sie sind gesetzt, nicht hergeleitet und nicht kalibriert. Gerade weil sie unsere sind, sind sie die schwächste Stelle.

Ob davon etwas über EtuSchlopf hinaus trägt, ist nicht extern geprüft. Wir behaupten es deshalb nicht.

## Offene Fragen, an denen mehr Mathematik ansetzen kann

- **Haltebögen als Zuordnungsproblem:** Bogenanfänge und Bogenenden je Taste als bipartites Matching, statt sie der Reihe nach abzuarbeiten.
- **Eine Belegungsregel für alle Platzierer** (Nr. 9), dazu dieselbe Regel in Bewertung und Ausführung (Nr. 5).
- **Überlappungen** gleicher Tasten: Nr. 9 von Zeitpunkten auf Intervalle erweitern.
- **Ein Entscheider für das Register** (Nr. 7 und 8 zusammenführen), mit der Schreibweise als Eingabe.
- **Prüfschranken als Aufrufgraph:** Erreichbarkeit statt Textsuche, ob neuer Code in eine Musikentscheidung einfließt.
- **Kalibrierung** der Gewichte in Nr. 5 und Nr. 7 an Beispielen, die ein Mensch beurteilt hat, statt per Setzung.

## Quellen

- C. F. Gauss, *Theoria motus corporum coelestium* (1809); A.-M. Legendre (1805): Methode der kleinsten Quadrate.
- R. Bellman, *Dynamic Programming* (1957); A. Viterbi (1967).
- M. C. Golumbic, *Algorithmic Graph Theory and Perfect Graphs* (1980): Intervallgraphen.
- C. A. R. Hoare, „An Axiomatic Basis for Computer Programming“ (1969): Invarianten.
- FIPS 180-4 (SHA-256); Fowler/Noll/Vo, FNV-Hash.
- R. M. Bittner et al., „A Lightweight Instrument-Agnostic Model for Polyphonic Note Transcription and Multipitch Estimation“ (ICASSP 2022): Basic Pitch, nur als WAV-Sensor; die Benchmark-Werte des Papers sind keine EtuSchlopf-Messwerte.
- G. C. Necula, „Proof-Carrying Code“ (POPL 1997): Verwandtschaft der SchlopfFormel.
- D. E. Knuth, M. F. Plass, „Breaking Paragraphs into Lines“ (1981): Verwandtschaft der Schlöpfchen-Formel.
- Kontext zur Spielformel, nicht zur Kalibrierung: C. Turner et al., *Applied Ergonomics* 133 (2026), 104709; J.-Y. Chi et al., *Applied Ergonomics* 97 (2021), 103518.
