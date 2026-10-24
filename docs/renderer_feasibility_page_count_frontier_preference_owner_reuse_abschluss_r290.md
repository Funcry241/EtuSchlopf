# r290 – RendererFeasibility: PageCount-Frontier mit bestehendem Preference-Owner

## Elternstand und Negativ-Evidenz

r290 basiert direkt auf dem real installierten und dunkelgrünen r288 `RegisternotationMusicXMLDokumentordnungsIdentitaetsabschluss` mit 1005/1005 Prüfungen und grüner Etüde 6.

r289 `RendererFeasibilityPageCountFrontierConstraintGenerationAbschluss` wurde **vor Installation** vom `Semantic-Capability-Diff` abgewiesen. Der fachliche Frontier-Gedanke war richtig, aber r289 hatte im Renderer-Pfad die bereits vorhandene PageCount-Präferenzordnung teilweise nachgebaut und dabei die neue lokale Variable `upgrade_budget` eingeführt. Der Sicherheitsvertrag klassifizierte diese korrekt als neue `CalibrationThreshold`. Eine neue Kalibrierungs-/Budgetwahrheit ist für diesen Architekturabschluss weder erforderlich noch erlaubt. r289 ist daher kein Elternstand.

## Unverändertes Architekturproblem

Der reale r288-Gegenlauf von Etüde 3 scheitert erst im produktiven Renderer-Feasibility-Oracle. Upstream besitzt der horizontale Planner bereits eine gebundene `feasible_page_counts`-Frontier. Vor dem Renderer wurde diese Frontier jedoch auf `selected_pages` reduziert. Nach einem Renderer-No-Good konnte deshalb nur innerhalb einer einzigen vorläufig bevorzugten Seitenzahl neu geplant werden.

Die harte Korrektur bleibt:

```text
gebundene PageCount-Frontier
+ unveränderte Measure-Demands
+ unveränderte Capacity-/Shape-Grenzen
+ run-lokale Renderer-No-Goods
→ Joint Candidate je weiterhin zulässiger Seitenzahl
→ vorhandener PageCount-Preference-Owner bewertet die Kandidaten
→ bester verbleibender Plan
→ Renderer erneut als reines Feasibility-Veto
```

## r290-Korrektur gegenüber r289

r290 dupliziert **keine** Präferenzformel im Renderer-Pfad. Jeder `JointPageSystemCandidate` wird ausschließlich in den bereits bestehenden `PageCountPreferenceCandidate` projiziert. Für den Vergleich wird dessen bereits grüner `is_preference_better_than`-Besitzer wiederverwendet.

Damit entstehen in r290:

- keine neue `budget`-Definition,
- keine neue Threshold-/Cutoff-/Weight-/Penalty-/Tolerance-Definition,
- keine neue Kalibrierungsautorität,
- keine zweite Preference-Wahrheit.

Die vorhandenen upstream-Merkmale – Systeme, Singletons, dünne Nicht-Endseiten, vermeidbare dünne Seiten, Abstand zur bevorzugten Seitenzahl, Balance und Slack – bleiben exakt beim bestehenden Besitzer. Der Renderer-Pfad besitzt nur die Frontier-Iteration und die Projektion in diesen vorhandenen Besitzer.

## Autoritätsgrenzen

- RendererAuthority bleibt `feasibility_veto_only`.
- Die PageCount-Frontier wird weder erweitert noch neu kalibriert.
- Nur upstream bereits als hart zulässig bewiesene Seitenzahlen dürfen verwendet werden.
- Initiale PreferenceSelection bleibt unverändert, solange sie rendererbestätigt ist.
- Ein Seitenzahlwechsel ist nur Folge realer run-lokaler Renderer-No-Goods.
- `singletons` und die übrigen Qualitätsmerkmale bleiben Präferenzen, keine neuen harten Veti.
- Keine Änderung an Registernotation, Tonhöhen, Dauern, Stimmen, Staff-Zuordnung oder Oktavdarstellung.
- Keine Etüden-, Takt-, Datei- oder Positionssonderlogik.

## Fail-closed und Regressionen

Die transportierte Frontier muss weiterhin nicht leer, strikt aufsteigend, positiv, an `preferred_pages` gebunden und mit der initial gewählten Seitenzahl konsistent sein. Wenn kein Frontier-Kandidat die Renderer-No-Goods überlebt, bleibt der Lauf rot.

Die zwei allgemeinen r289-Regressionen bleiben Bestandteil von r290:

1. Nach No-Goods darf eine rendererunlösbare initiale Seitenzahl verlassen werden, wenn eine andere bereits gebundene Frontier-Seitenzahl weiterhin zulässig ist.
2. Wenn mehrere Seitenzahlen zulässig bleiben, entscheidet weiterhin die bereits bestehende PageCount-Präferenzordnung; es gibt keinen pauschalen Mehr-Seiten-Fallback.

Gegenüber r288 kommen zwei Rust-Prüfungen hinzu. Erwartung: **1007/1007**.

Prüfreihenfolge nach erfolgreicher Übernahme: Etüde 3, Etüde 6 als Nichtregression, danach Vollkorpus.
