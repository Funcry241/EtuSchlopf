# r587 – RegisterNotationVisibleToneAnchorFeasibleBoundaryStateSpaceClosure

## Autoritative Basis

r587 basiert auf dem installierten r586 `RegisterNotationLowerStaffSupportedClefReadabilityStateSpacePackagingRecovery`.

r586 hat den unteren RegisterNotation-Lesbarkeitsraum erfolgreich um die bereits unterstützten Bass-/Violinschlüsselzonen erweitert. Der Einzelprojektlauf Etüde 8 ist grün und beseitigt insbesondere die zuvor unnötige T31-8va. Im anschließenden Vollkorpus blieben neun Projekte grün; ausschließlich Etüde 5 brach in der produktiven Oktav-Sichtmaterialisierung mit Exit 101 ab.

Die frische Einzelreproduktion von Etüde 5 lokalisiert den Fehler exakt:

```text
RegisternotationsSichtmaterialisierung:
kein sichtbarer Dokument-Stopanker fuer System 2 bei 384
```

Build, Strict Clippy und alle integrierten Rust-Tests waren davor grün. Der Fehler liegt damit nicht im Launcher, Renderer oder Take-Transport, sondern an einer RegisterNotation-Zustandsgrenze, die der bestehende Writer gemäß r287/r288 zu Recht nicht materialisieren kann.

## Bereits geltender Vertrag

Seit r287/r288 gilt für jede sichtbare Oktavlinie:

- Start und Stop werden ausschließlich an reale sichtbare Tonanker gebunden;
- es wird kein `<offset>` erzeugt;
- der Writer trifft keine neue musikalische Entscheidung;
- eine Sichtlinie ohne sichtbaren Start- oder Stopanker muss fail-closed bleiben.

Dieser Vertrag wird in r587 nicht verändert.

## Ursache

Der semantische Registerplan bewertet Tonereignisse mit ihrer eigenen Dauer. Innerhalb eines MusicXML-Akkords kann deshalb ein sichtbares `chord`-Member einen späteren semantischen Endzeitpunkt besitzen als der nicht-`chord`-Host, der den Dokumentcursor des Akkordangriffs trägt.

Der bestehende Sichtwriter kann einen Stop ohne `<offset>` jedoch nur an dem realen Dokumentanker materialisieren, der aus dem Host-Angriff und dessen Dokumentcursor entsteht. Ein längeres Chord-Member erzeugt keinen eigenständigen späteren Dokumentcursor und damit keinen zulässigen Stopanker an seiner bloßen Ereignisdauer.

r586 konnte durch die neue Lower-Staff-Gegenrechnung erstmals ein optimales Fenster wählen, dessen semantisches Ende genau auf einer solchen nicht materialisierbaren Chord-Member-Grenze lag. Der Writer verweigerte diesen Zustand korrekt.

## Fachliche Änderung

r587 schließt ausschließlich den **feasiblen Zustandsraum des bestehenden RegisterNotation-Decision-Owners**:

1. Aus den bereits beobachteten `RegisternotationsEreignis`-Objekten werden pro Evidenzsegment die vorhandenen sichtbaren Angriffsträger rekonstruiert.
2. Ein nicht-`akkordton`-Ereignis ist der Host eines möglichen Sichtankers.
3. Unmittelbar folgende Chord-Member desselben Takts, derselben Stimme und desselben Einsatzes können dessen Sichtbarkeit erweitern, ändern aber nicht dessen Host-Ende.
4. Für jedes ohnehin betrachtete sichere Kandidatenfenster werden der erste sichtbare Beginn und das maximale sichtbare Ende bestimmt.
5. Nur Fenster, deren beide Grenzen bereits durch einen solchen sichtbaren Host-Angriff materialisierbar sind, dürfen in die bestehende Gain-/Tie-Break-Konkurrenz eintreten.
6. Unter den verbleibenden feasible Fenstern bleiben Kosten, Verbesserungsschwelle, Oktavartwahl, Grenzqualität und Tie-Break unverändert.

Damit wird kein nachträglicher Writer-Fallback eingeführt. Ein Zustand, den die kanonische StructuralWriteAuthority später zwingend verweigern müsste, wird bereits vom zuständigen Decision-Owner nicht mehr als produktiv feasible gewählt.

## Positive und negative Regression

Die neuen Tests binden beide Seiten der Invariante:

- Ein längeres sichtbares Chord-Member darf keinen künstlichen Stopanker nach dem Ende seines Hosts erzeugen.
- Ein optimales Fenster, dessen einziges Ende auf dieser nicht materialisierbaren Chord-Member-Dauer liegt, wird nicht gewählt.
- Ein gleich langer Chord-Host/Chord-Member-Angriff bleibt vollständig materialisierbar und behält die vorhandene 8va-Entscheidung.

Damit ist die Closure keine allgemeine Chord-Unterdrückung und keine Oktavierungsreduktion.

## Nicht geändert

r587 ändert nicht:

- `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs`;
- StructuralWriteAuthority oder Writer-Reachability;
- den r287/r288-Sichtanker- und No-Offset-Vertrag;
- Bass-/Violinschlüsselzonen aus r586;
- `MINDEST_TONOBJEKTE`;
- `VERBESSERUNGSSCHWELLE`;
- die quadratische Registerkostenformel;
- 15ma/15mb-Bezeichnungsstrafe;
- r578-Grenzklassen oder Tie-Break;
- ClefReadability-DP;
- Pitch, Onset, Duration, Staff oder Voice;
- Beam-, Rest-, Slur-, Renderer- oder SchemaSchlopf-Owner.

Es gibt keine Etüden-, Takt-, Pitch- oder Positionssonderregel und keine neue Repair-Schicht.

## Qualifikation

Nach Installation zuerst:

```powershell
.\run.cmd --take "Etüde Nr. 5"
```

Erwartung:

- kein Exit 101;
- RegisterNotation-Sichtmaterialisierung vollständig;
- Etüde 5 formal grün und PDF vorhanden.

Erst danach:

```powershell
.\run.cmd --all
```

Der Vollkorpus entscheidet, ob r587 die r586-Verbesserung ohne neue Breitenregression schließt.
