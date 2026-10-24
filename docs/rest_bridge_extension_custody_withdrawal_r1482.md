# r1482: adoptierte E050-Erweiterung vor spaeter Stimmenzuteilung zuruecknehmen

Parent: r1481, SscK2GovernanceRevisionKodex. Klasse: GovernedChange.
Version bleibt 0.10.1839. Die native Annahme von r1479/r1480/r1481 bleibt Voraussetzung.

## Verhalten und Belegkette

E050 liefert neben seinem unveraenderten PRESERVING-Receipt einen separaten,
nicht klonbaren Vorschlagsbeleg aus den tatsaechlich ausgefuehrten Gruppen.
Quell-/Attack-/Fragmentidentitaeten stammen aus dem bereits authentifizierten
kanonischen Bind. Originale positive Mitgliedschaft kommt ausschliesslich aus
dem echten Original-Owner. Der Census ist weiterhin authority=none.

Der Vorschlag wird erst nach erfolgreicher Main-Adoption uebernommen. Die
lokale Main-State-Bindung, das genaue Post-Dokument und dieselbe Quellinstanz
muessen uebereinstimmen. Verlorene Vorschlaege, fremde States und stale Inputs
erteilen keine Ruecknahmebefugnis. Ein optionaler Beleg veraendert keine Stimme.

Die Belegkette prueft die tatsaechlichen Metrik- und FinalNotation-Uebergaenge.
Unveraenderte gepitchte Folge, beteiligte Balken, Timing, Stimmen, Bindungen und
vollstaendige Akkordressourcen sind Voraussetzung. Nicht unterstuetzter Drift
pensioniert die ganze betroffene Verbindung dauerhaft; ein spaeter wieder
passendes XML erzeugt keinen neuen Beleg. FinalChord uebernimmt ausschliesslich
seine tatsaechlichen eindeutigen Output-Occurrences derselben bereits bestehenden
Identitaeten. Die FinalPdf-Fortsetzungen behalten diese Herkunft.

Vor dem bestehenden finalen Stimmen-Commit wird der bereits gebundene simultane
Zuteilungsplan gelesen. Trennt er eine erhaltene Erweiterungsverbindung, waehrend
die urspruengliche positive Komponente zusammenbleibt, stellt derselbe E050-Owner
genau seine vorherigen Balkenzustaende der Ebenen 1/2 wieder her. Hoehere Ebenen
und alle anderen Noteninhalte bleiben erhalten. Ein vollstaendiger Vergleich der
Balkenoberflaeche prueft den Output vor der normalen Main-Adoption. Anschliessend
werden die vorhandenen Stimmen-Edits mit ihren vorhandenen Identitaeten neu an
die verschobenen Bytepositionen gebunden; die Zuteilung selbst bleibt gleich.

Unabhaengige Verbindungen entscheiden lokal; ueberlappende Schreibressourcen
werden gemeinsam abgelehnt. Es gibt keine Korpus-, Takt-, Tonhoehen-, Zahlenstimmen-
oder Haeufigkeitsauswahl. Vertrag:
selector_derivation_contract_rest_bridge_extension_withdrawal.md.

Die beiden r1481-Bytepins bleiben exakt. Kein Gate, Trust-Anchor, SSC-Commit,
RenderSovereignState oder Master-Lineage-Writer wird in diesem Take geaendert.

## Diagnose

`BridgeExtensionCustody` meldet Main-Uebernahme, pensionierte Verbindungen bei
Owner-/FinalChord-Uebergaengen und gebundene Ruecknahmezeilen. Diese Zahlen sind
Diagnose, keine Auswahl- oder Schreibbefugnis. `voice_write=false` beschreibt die
Ruecknahmeoperation; der anschliessende bestehende Stimmen-Commit bleibt aktiv.

## Native Abnahme

Nur nach erfolgreich angenommenem r1481 installieren:

    .\run.cmd --take --all --writer-trace

Korpusbezug nur fuer Evaluation: E6/T32 soll das Originalpaar 192/216 behalten;
264 soll ohne Balken bleiben. Stimmen gegen r1480 vergleichen, alle Master-XMLs
bytegleich verlangen, Render-Differenzen vollstaendig auf die freigegebene
Balkenruecknahme begrenzen. Die r1478-Ergebnisse in E3/T6, E5/T36/T37 und E6/T34
muessen erhalten bleiben. PDF- und Sichtpruefung erfolgen im nativen Lauf.
Dieser Quell-Take behauptet keinen bereits erfolgten Windows-/Renderer-Lauf.
