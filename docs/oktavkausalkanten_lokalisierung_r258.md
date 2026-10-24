# r258 – OktavKausalkantenLokalisierung

## Zweck

r258 lokalisiert ausschliesslich die erste Verarbeitungskante, an der eine im finalen Masterstand sichtbare Abweichung zwischen Oktavspanne und gedrucktem Tonbereich erstmals auftritt. Der Schnitt schreibt und repariert nichts.

## Beobachtungsfolge

Vor K00 wird der XML-Zustand unmittelbar vor `apply_octave_notation_policy` als reine Ausgangsreferenz erfasst. Danach gelten:

- K00: unmittelbar nach der Oktavierungsentscheidung und Erstmaterialisierung
- K01: nach der unteren Schluessellesbarkeit
- K02: nach der letzten fruehen Notationsbereinigung
- K03: nach der Exportnormalisierung
- K04: nach dem visuellen Layoutdruck
- K05: nach der pianistischen Lesbarkeitspolitur
- K06: nach der komponistentreuen Wiederherstellung
- K07: nach der redaktionellen Stimmenentscheidung
- K08: nach der Salienz-Kandidatenstufe
- K09: nach der diskreten Salienzübersetzung
- K10: nach der Konflikt-Systemteilungsprobe
- K11: nach der zertifizierten lokalen Umschreibung
- K12: Eingang der initialen Rendertransaktion; absichtliche Uebergabekontrolle zu K11
- K13: nach der erneuten Master-MusicXML-Politur
- K14: nach der Teil-Schlag-Balkenbehandlung
- K15: nach der Schluesselmaterialisierung
- K16: nach der Balken-/Pausenvervollstaendigung
- K17: unmittelbar nach Bildung des endgueltigen Master-Export-Zwischenstands
- K18: nach der Cursor-Invariantenbehandlung des Master-Exports

## Lokalisierungsregel

Nur Bereiche, die in K18 eine vorhandene eindeutige Oktavspanne besitzen und dort gedruckte Tonereignisse vor dem Beginn, nach dem Ende oder eine nicht positive Spannweite aufweisen, werden zurueckverfolgt.

- Ist die Abweichung bereits in K00 vorhanden, lautet die erste Grenze `K00 / Erstmaterialisierung`.
- Andernfalls wird die erste Kante `Kxx->Kyy` gemeldet, an deren Nachzustand die Abweichung erstmals auftritt.
- Fuer diese Kante wird getrennt klassifiziert, ob sich Oktavmarken, sichtbare Tonereignisse oder beide Strukturen veraendert haben.

Die Ausgabe aggregiert nur Fallzahlen je erster Kante. Sie waehlt keine Etuede und keinen Takt als Arbeitsziel.

## Sicherheits- und Befugnisgrenze

Die Beobachtung lebt fachlich in `src/audit_only/`. Die Aufrufe an produktiven Verarbeitungskanten lesen `best.layout_xml` beziehungsweise `final_export_xml` ausschliesslich als `&str`. Es wird kein XML zurueckgegeben, kein Kandidat ersetzt, keine Schwelle veraendert und keine Schreibfunktion aufgerufen.

Der technische Aenderungsvertrag lautet `RefactorOnly`, weil produktive Orchestrierungsdateien fuer die Beobachtung um reine Leseaufrufe ergaenzt werden. `AuditOnly` wird bewusst nicht umgangen oder erweitert.

## Stopregel

Nach realer Einzelprojekt- und Vollkorpusabnahme wird nur die erste Abweichungskante ausgewertet. Danach endet der Untersuchungsstrang. Eine Reparatur oder Neuordnung der Oktavierung ist nicht Bestandteil von r258.
