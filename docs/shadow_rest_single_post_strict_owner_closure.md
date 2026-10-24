# Shadow Rest Single Post-Strict Owner Closure

## Scope

`v0.10.1689 r01` ist Take 3 und damit der letzte Take des auf drei Takes
begrenzten `RepairLoopOwnershipClosure`-Superwurfs. Er entfernt ausschliesslich
den produktiven Pre-StrictStaff-Aufruf von
`apply_upper_staff_shadow_rest_voice1_keep_without_shadow_lane` und laesst den
bestehenden Post-StrictStaff-Aufruf als einzigen Owner bestehen.

## Korpusbeweis aus v0.10.1688

Der bestaetigte 10/10-Korpus zeigt fuer den Shadow-Rest-Pfad:

- Pre-StrictStaff: 10/10 invoked, 0 candidates, 0 writes.
- StrictStaffOverfullCoverageNormalization schreibt in 7/10 Projekten.
- Post-StrictStaff: 10/10 invoked, 0 candidates, 0 writes.
- Alle 20 Master-/Render-MusicXML-Dateien sind gegen v0.10.1687 nach reiner
  Versionsbereinigung identisch.
- Alle 57 PDF-Seiten sind pixelidentisch mit 0 veraenderten Pixeln.

Der Pre-Aufruf besitzt damit im aktuellen Korpus keine beobachtete Wirkung. Der
Post-Aufruf bleibt bewusst erhalten, weil StrictStaff erst danach eine sichtbare
Staff-Lane materialisieren kann. Damit wird nicht die Shadow-Rest-Regel
pensioniert, sondern nur ihre historisch zu fruehe zweite Invocation.

## Eigentumswechsel

Vorher:

`ShadowRest (pre)`
-> `StrictStaffOverfullCoverageNormalization`
-> bedingt `ShadowRest (post)`

Nachher:

`StrictStaffOverfullCoverageNormalization`
-> `ShadowRest (single post owner)`

Der Shadow-Rest-Algorithmus, seine Kandidatenlogik, Guards, Invarianten und
Mutation bleiben unveraendert. Der verbleibende Post-StrictStaff-Write benutzt
weiterhin `write_voice_keep_shadow_rewrite_checkpoint_authorized`. Die nur vom
retirierten Pre-Aufruf verwendete
`write_shadow_rest_rewrite_checkpoint_authorized`-Authority, ihre Enum-Variante
und ihr Re-Export werden gleichzeitig entfernt, damit keine tote Writer-
Infrastruktur zurueckbleibt.

## Explizit unveraendert

Nicht geaendert werden:

- `apply_upper_staff_shadow_rest_voice1_keep_without_shadow_lane`,
- StrictStaff-Kandidaten- oder Mutationsemantik,
- ShortRestGlyph-Single-Final-Pass aus Take 2,
- Staccato-Primary-/Persistence-Fallback-Pfad,
- Staff/Voice/Pitch/Onset/Duration,
- Schwellenwerte, Gewichte oder musikalische Regeln,
- Render-/PDF-Auswahl und Releaseentscheidung.

## Zielsystembeweis

Abnahme mit genau einem Befehl:

```powershell
.\run.cmd --take --strict --all
```

Erwartet werden 10/10 OK, Build/Clippy/Final 0, 0 Blocker, 0 Warnungen,
versionsbereinigt identische Master-/Render-MusicXML und pixelidentische PDFs
zu `v0.10.1688`. Die Repair-Loop-Evidence muss `ShadowRestPre.invoked = 0` und
`ShadowRestPost.invoked = 1` bei vorhandener finaler Render-Quelle zeigen.
Danach ist der Superwurf beendet; es gibt keinen Take 4.
