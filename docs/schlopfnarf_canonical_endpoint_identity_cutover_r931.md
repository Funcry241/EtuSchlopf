# SchlopfNarfCanonicalEndpointIdentityCutover r931

**Authority parent:** `0.10.1839 r930`  
**ChangeClass:** `RefactorOnly`  
**Musikmutation:** keine neue  
**Report-/MusicXML-/PDF-Format:** unveraendert

## Reale r930-Basis

r930 ist fuer Etuede Nr. 3 `dark_green / released / FinalExit 0`. Strict Clippy meldet `TotalWarnings: 0`; Build und Renderer sind `passed`. Damit ist r930 die neue released Green Base fuer den NarfSchlopf-Komplexitaetsabbau.

## Schmerzstelle

HAN03 kannte die beiden tatsaechlich gewaehlten Sprungendpunkte A und B bereits im Validator. Danach wurde diese Identitaet jedoch auf Text reduziert:

```text
HAN03 A/B
-> Issue.details mit Linie, Tonlabels und Zeitabstand
-> spaetere Consumer parsen den Text erneut
-> find_jump_issue_note_pairs baut aus AtomicNote erneut eine Extreme-Line
-> A/B werden erneut gesucht
```

Gleiche Tonlabels sind aber keine stabile Endpoint-Identity. Die r922-Evidence hatte bereits gezeigt, dass `G#2 -> G#4` bei gleichem sichtbaren Label von 48 auf 96 divisions wechseln konnte.

## Schnitt

r931 transportiert die von HAN03 bereits bestimmte A/B-Identitaet direkt und typisiert im bestehenden `Issue`:

```text
Han03JumpIdentity
  line_kind
  from { start, duration, midi, pitch_label, is_chord, tie_stop }
  to   { start, duration, midi, pitch_label, is_chord, tie_stop }
```

Die Identity ist `serde(skip)` und veraendert deshalb weder den persistierten Ergebnisbericht noch bestehende externe Reportformate.

`find_jump_issue_note_pairs()` rekonstruiert keine HAN03-Linie mehr und parst weder Tonlabels noch `Zeitabstand` aus `Issue.details`. Es loest nur noch die beiden kanonischen Endpunkte exakt gegen das AtomicNote-Inventar auf.

Es gilt fail-closed:

- fehlende `Han03JumpIdentity` -> kein Paar,
- unbekannter `line_kind` -> kein Paar,
- fehlender Voice-Scope fuer eine Voice-Line -> kein Paar,
- A oder B nicht exakt eindeutig -> kein Paar,
- A und B loesen auf dieselbe Note -> kein Paar.

Die bestehenden Atomic-/Corpus-/Restify-/Move- und VoiceFiberBehindBars-Consumer benutzen damit dieselbe von HAN03 erzeugte Endpoint-Identity. Die Legacy-Funktionen `parse_jump_labels`, `parse_jump_line_kind`, `AtomicJumpPoint` und `build_atomic_extreme_line` entfallen aus diesem Pfad.

## Nicht geaendert

- keine HAN03-Schwelle,
- keine Severity,
- keine Regelgewichtung,
- keine neue Reparaturart,
- keine neue Writer-Authority,
- keine Staff-/Voice-/Pitch-/Onset-/Duration-Mutation,
- keine Etuede-/Takt-/Tonhoehen-Sonderregel,
- keine nearest-note-Heuristik,
- kein Report-Text als Maschinen-Authority.

`Issue.details` bleibt fuer Menschen und bestehende Reports byte-semantisch gleich; nur seine fruehere Rolle als Rueckkanal fuer Maschinenidentitaet wird entfernt.

## Regressionen

Die Locator-Tests beweisen jetzt explizit:

1. Ein absichtlich nicht parsebarer Legacy-Detailtext verhindert die Aufloesung nicht, wenn die kanonische Identity vorhanden ist.
2. Ein vollstaendig plausibler alter Detailtext erzeugt ohne kanonische Identity keinen Reparaturanker.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 3"
```

Nur `dark_green / released / FinalExit 0` akzeptiert r931 als neue Green Base. Danach soll Etuede 7 wieder direkt laufen; entscheidend ist, ob der verbliebene SchlopfNarf nun auf derselben kanonischen A/B-Identity durch Report, BehindBars und Repair verfolgt wird, statt erneut aus Tonlabels rekonstruiert zu werden.
