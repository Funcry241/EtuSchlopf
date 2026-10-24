# ShadowRest Pre-Owner Authority Retirement

## Stand

`v0.10.1786 r01 – ShadowRestPreOwnerAuthorityRetirement`

## Ausgangslage

Der autoritative Stand `v0.10.1785 r05` ist 10/10 dunkelgruen. Der historische
`UpperStaffShadowRestVoice1KeepWithoutShadowLaneFix` wird dort nach
`StrictStaffOverfullCoverageNormalization` genau einmal aufgerufen, findet aber
korpusweit 0 Kandidaten und schreibt 0-mal. `StrictStaff` selbst bleibt produktiv:
7/10 Projekte schreiben, insgesamt werden 19 Kandidaten adoptiert.

Runtime-Null allein reicht nicht fuer ein Retirement. Deshalb wurde der
DecisionScope des alten ShadowRest-Writers gegen den unmittelbar folgenden
kanonischen Restrollen-Owner verglichen.

## Zwei konkurrierende DecisionScopes

Der alte ShadowRest-Pre-Owner entscheidet nur auf Staff 1 und waehlt zunaechst
einen einzelnen Focus-Takt. Innerhalb dieses Takts behandelt er jede sichtbare
Pause in Voice != 1 bereits dann als Shadow-Kandidat, wenn irgendeine klingende
andere Voice zeitlich ueberlappt. Die Ueberdeckungsstaerke der klingenden Voice
ist fuer diese lokale Entscheidung unerheblich.

Unmittelbar danach laeuft bereits der allgemeine
`FinalVisibleRestRole`-/`WholeRestDisplayCoordinateReadbackTruthFix`-Owner. Er
entscheidet atomar fuer beide Staves und verlangt fuer eine
`AUXILIARY_SHADOW_BOOKKEEPING_REST_LANE` mindestens 50 Prozent klingende
Fremdstimmen-Ueberdeckung sowie die vorhandene Rollenbedingung
(same-voice silent oder Voice-1-Rest-Ueberlappung). Voice 1 bleibt explizit
geschuetzt.

Damit gilt statisch:

`DecisionSet(OldShadowRest)` ist **keine** Teilmenge von
`DecisionSet(FinalVisibleRestRole)`.

Der alte Writer kann also eine Pause vorzeitig in Forward-Zeit verwandeln, die
der spaetere kanonische Owner bewusst als sichtbare Pause schuetzen wuerde.
Das ist Pre-Owner-Preemption und kein legitimes Recovery-Sicherheitsnetz.

## Retirement-Entscheidung

v0.10.1786 entfernt ausschliesslich die produktive Write-Authority des alten
post-Strict ShadowRest-Pre-Owners:

- `apply_upper_staff_shadow_rest_voice1_keep_without_shadow_lane` entfernt,
- `apply_upper_staff_shadow_rest_rewrite_authorized` entfernt,
- `VoiceKeepShadowRewrite`-Checkpoint entfernt,
- der Orchestrator ruft den alten Writer nicht mehr auf,
- der PDF-Source-Evidence-Ledger fuehrt den pensionierten Writer nicht mehr.

`StrictStaffOverfullCoverageNormalization` bleibt unveraendert produktiv.
Der unmittelbar folgende atomare `FinalVisibleRestRole`-Owner bleibt der einzige
produktive Owner der Schatten-/Hilfspausen-Rollenentscheidung.

## Eine Wahrheit, ein Klassifikator

Die atomare Restrollenbeobachtung wird in
`observe_atomic_visible_rest_role()` gebuendelt. Der produktive
`FinalVisibleRestRole`-Owner und der neue read-only committed-state Detector
verwenden dieselbe Kandidatenpraedikatsfunktion
`atomic_visible_rest_is_shadow_bookkeeping_candidate()`.

Damit entsteht keine zweite Wahrheit fuer den Detector.

## Committed-State Gate

Nach dem finalen Commit wird das persistierte Render-MusicXML erneut gelesen.
Neu gilt zusaetzlich:

`CommittedShadowRestCandidates = 0`

Der Detector besitzt kein Write-Recht. Ein spaeterer Writer, der nach dem
kanonischen Restrollen-Owner erneut eine `AUXILIARY_SHADOW_BOOKKEEPING`-Domain
erzeugt, macht den committed-state Beweis fail-closed rot.

Eine read-only Wiederholung des kanonischen Selectors auf den zehn finalen
v0.10.1785-r05 Render-MusicXML-Dateien ergibt bereits 10/10 `0` Kandidaten.

## ChangeClass

`RefactorOnly`.

Keine Pitch-, Onset-, Dauer-, Staff-, Voice-, Beam-, Cursor- oder Master-
MusicXML-Mutation. Der aktuelle Korpus hat 0 produktive ShadowRest-Writes;
daher wird fuer v0.10.1786 vollstaendige Output-Invarianz gegen v0.10.1785 r05
erwartet.

## Abnahme

Zuerst:

```powershell
.\run.cmd --take --strict "Etüde Nr. 6"
```

Erwartung:

- Build/Clippy/Program/Final 0,
- `ShadowRestPost: invoked 0; candidates 0; written 0`,
- `CommittedShadowRestCandidates 0`,
- alle bisherigen committed-state Gates gruen,
- Master-/Render-MusicXML gegen 1785 invariant bis auf Version,
- PDF pixelidentisch.

Danach Vollkorpus 10/10 und derselbe Output-Invarianzbeweis.
