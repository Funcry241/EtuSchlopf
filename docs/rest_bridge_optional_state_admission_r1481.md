# r1481: exakte Transportzulassung fuer vorhandenen oder fehlenden Render-State

Parent r1480; KodexOnly; Version 0.10.1839. Keine produktive Quellaenderung.

Bei der Pruefung des Reparaturentwurfs wurde eine echte Nebenpfadverletzung im
vorbereiteten r1480-Bild gefunden: Die neuen B1-/B2-Fortsetzungen werteten
RenderSovereignState::require_document unbedingt aus. Die bestehenden spaeten
Branches duerfen ohne Render-Historie auf ihrer geschlossenen Original-Wurzel
beginnen. Wenn ihr Writer nichts adoptiert, bleibt der Render-State rechtmaessig
abwesend. Ein fehlender optionaler Erweiterungsbeleg darf diesen Pfad nicht
abbrechen. Der bedingungslose Read war deshalb ein Fehler meines Entwurfs.

Dieser Take ersetzt nur den einen zukuenftigen Stage-Digest der geschlossenen
Zweierzulassung; der zweite Digest und alle anderen bestehenden Pins bleiben
gleich. Das alte unbedingte Bild ist danach ausdruecklich nicht mehr zugelassen.
Die produktiven Dateien des installierten r1480 bleiben bytegleich.

Das korrigierte kuenftige Bild uebergibt bei B1 und B2 render_state.document()
als Option an continue_rest_bridge_extensions_at_state. Ohne Beleg ist fehlender
State ein No-op. Mit aktivem adoptiertem Beleg bleibt fehlender State ein Fehler.
Es wird weder ein Render-State erzeugt noch eine Original-Wurzel als Render-State
ausgegeben. Die CrossingRest-Fortsetzung bleibt an das echte Transaktionsdokument
gebunden. Der Implementierungstake ist r1482.

| Exakter kuenftiger Pfad | SHA-256 |
| --- | --- |
| src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs | b96623d3f09f026acd026aed1e22afb885b230b0b53b8344275763b1ba29a26b |
| src/structural_write_authority/mod.rs | b4035a8ad5c0b35e934e6aef38d737863238a7d4f19a2877a25d537bde98ed68 |

Acht Gate-Tests pruefen die geschlossene Paarzulassung, Verwerfung des alten
Bildes, einseitigen Drift, fremde Pfade, Zusatz-/Duplikatzeilen und die bestehenden
Checkpoint-Pins. Die vorhandenen Source- und Identity-Gates bleiben unabhaengige
Bedingungen. Dieser Take aktiviert noch keine Balkenreparatur.

Reihenfolge: r1480 erfolgreich annehmen, dann r1481 mit
`.\run.cmd --take --all --writer-trace`; erst danach r1482 bereitstellen.
Noch kein nativer Windows-/Renderer-Lauf dieser vorbereiteten Takes behauptet.
