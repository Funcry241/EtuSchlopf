# CrossStaffHandProjectionReadabilitySharedBeamOwnerClosure r341

## Ausgangspunkt

Autoritative Basis ist der real GREEN gelaufene r340 `RestExistenceGapMaterializationEvidence` auf r339-Vollkorpus-Evidence. r340 hat fuer Etuede 5 / Takt 5 bewiesen, dass die auffaellige untere Staff-Luecke keine Voice-Silence ist: Voice 1 ist ueber beide Klavierstaffs 384/384 divisions klingend belegt. RestExistence ist deshalb nicht der produktive Zielowner.

## Konkreter Lesbarkeitsbefund

Etuede 5 / Takt 5, Zaehleinheit 3 ist eine einzige lueckenlose Voice-1-Geste aus vier Sechzehnteln:

- t=192 B5 Staff 1,
- t=216 C#5 Staff 1,
- t=240 D5 Staff 2,
- t=264 G5 Staff 1.

Im r339/r340-Render ist nur das Staff-1-Praefix B5/C#5 gebalkt; D5 und G5 stehen einzeln. Dadurch wirkt die Staff-2-Note optisch wie ein isoliertes Ereignis, obwohl sie metrisch und stimmlich mitten in derselben Voice-/Beat-Geste liegt.

Der 10er-r339-Korpus enthaelt 23 lueckenlose, beamfaehige Cross-Staff-Voice/Beat-Gruppen in 9 Etueden. Gegen den bereits etablierten `structural_metric_beam_hierarchy_plan(..., 2)` weichen dort 57 Noten in ihrer aktuellen L1/L2-Projektion ab. Der historische `CrossStaffVoiceLedgerMaterializer` hat mit seinem wesentlich engeren Volltakt-/Forward-/all-16th-Selektor im selben Korpus 0 Kandidaten.

## Ownership-Cut

r341 erzeugt keinen neuen Beam-DecisionOwner. Die fruehe BeatScoped-Gruppierung bleibt absichtlich unveraendert `Staff/Voice/Beat`: Voice-Nummern allein sind in Klavier-MusicXML kein hinreichend sicherer globaler Cross-Staff-Schluessel, und zwei gleichzeitig klingende Staff-Linien duerfen niemals durch einen pauschalen `Voice/Beat`-Merge vermischt werden.

Der produktive Cut sitzt deshalb am spaeten `ExternalFeedbackClosure`. Dort wird zusaetzlich eine streng begrenzte Cross-Staff-Projektion erkannt:

- gleiche Voice und gleiche Zaehleinheit,
- nur sichtbare klingende beamfaehige Ereignisse,
- jede Dauer hoechstens eine halbe Zaehleinheit,
- lueckenlos und nicht ueberlappend,
- mindestens ein realer Staff-Wechsel,
- die komplette Gruppe bleibt innerhalb derselben Zaehleinheit.

Nur fuer solche Gruppen konsumiert die Projektion den bereits bewaehrten `structural_metric_beam_hierarchy_plan(..., 2)`. Sie hat im spaeten Render-Abschluss Vorrang vor einem staff-lokalen Praefix, damit Etuede 5 / Takt 5 nicht als `B-C#` plus zwei isolierte Sechzehntel stehen bleibt. Ueberlappende gleich nummerierte Voices auf zwei Staffs werden durch die Lueckenlosigkeitsbedingung fail-closed verworfen.

Der bestehende enge `CrossStaffVoiceLedgerMaterializer` bleibt aus Kompatibilitaetsgruenden als Materialisierungsphase erhalten, entscheidet seine Beam-States aber nicht mehr lokal. Auch sein all-16th-Fall konsumiert nun den gemeinsamen `structural_metric_beam_hierarchy_plan`.

## Invarianten

Nicht geaendert werden:

- Pitch,
- Onset,
- Duration,
- Staff,
- Voice,
- RestExistence und RestVisibility,
- Clef-/Register-Entscheidungen,
- RestSpanConsolidation,
- Passreihenfolge,
- Schwellen/Gewichte,
- Security-/Take-Gates,
- Master-Sounding-Events.

Erlaubtes Delta sind ausschliesslich Beam-L1/L2-Tags auf bereits vorhandenen sichtbaren klingenden Noten innerhalb einer streng qualifizierten lueckenlosen gleichen Voice/Beat-Cross-Staff-Domain. Der fruehe Master-/BeatScoped-Pfad bleibt unveraendert; sichtbar aendern duerfen sich Render-MusicXML und PDF. Deshalb `ChangeClass=MusicMutation`.

## Abnahme

Erster Lauf:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartung: Build/Clippy/Tests/FinalExit GREEN und keine Pitch-/Onset-/Duration-/Staff-/Voice-Aenderung.

Danach Vollkorpus:

```powershell
.\run.cmd --all
```

Besonders zu lesen:

- Etuede 5 / Takt 5, Zaehleinheit 3: vier 16tel als eine Cross-Staff-Balkengeste,
- die 23 corpusweit beobachteten Cross-Staff-Voice/Beat-Gruppen,
- keine Wiederkehr von Achtel+Beam2 oder Beam-State auf Resten,
- Rest-/Shadow-Rest-Gegenbeweise unveraendert GREEN.

AuditOnly-Evidence wird in r341 absichtlich nicht mitgeaendert. Erst nach realem GREEN darf ein separater kleiner Evidence-Projektionsabschluss die r340-Zeile `CrossStaffHandProjectionReadability open` auf den bewiesenen Zustand umstellen.
