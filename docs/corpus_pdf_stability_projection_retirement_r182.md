# r182 – CorpusPdfStabilityProjectionRetirementClosure

## Problem

Der Korpusbericht fuehrte `PDFStabil` als scheinbar belastbare Projekteigenschaft.
Der Wert besass jedoch keinen eigenen Wahrheitsbesitzer. Er wurde aus historischen
freien Textfragmenten der projektbezogenen Ergebnisdatei erschlossen:

- `pdf stabil ja`
- `pdf-seiten nach rendern`
- `pdf nach finaler musicxml ja`

Die aktuelle Ergebnisdarstellung verwendet diese Formulierungen nicht mehr. Dadurch
meldete der vollstaendig gruene r180-Korpus `PDF-stabil 0`, obwohl das gebundene
Korpus-Laufmanifest fuer alle zehn Projekte sowohl vollstaendige Ausgaben als auch
beobachtete Renderer-Ergebnisse nachweist.

## Entscheidung

`PDFStabil` wird pensioniert statt durch eine neue Heuristik repariert.

Stabilitaet ueber mehrere Zustaende wird an dieser Stelle nicht gemessen. Der
Korpusbericht projiziert bei einem laufgebundenen `--all`-Bericht stattdessen exakt
die bereits autoritativen Manifestdimensionen:

- `outputs_complete` -> `AusgabenVollstaendig`
- `renderer_outcome_observed` -> `RendererBelegt`

Bei einem losgeloesten Korpusbericht ohne passendes Laufmanifest werden beide Werte
als `n/a` ausgegeben. Es wird keine Wahrheit aus veralteter Berichtssprache erfunden.

Die laufgebundene Manifestauswertung liegt in einem reinen Lesebesitzer. Die bestehende
Datei-Schreibfunktion des Korpusberichts erhaelt nur neutrale, bereits projizierte
Ausgabeevidenz. Dadurch entsteht keine neue oder reaktivierte Writer-Reichweite ausserhalb
der `StructuralWriteAuthority`.

## Reichweite

Keine Musik-, MusicXML-, Render-, PDF- oder Schreibentscheidung wird geaendert.
Der unabhaengige `VoiceFiberBehindBarsDecisionLedger` besass zusaetzlich einen lokal
irrefuehrend benannten `pdf_stable`-Bool. Seine Semantik bleibt unveraendert
(`pdf_pages > 0`), wird aber korrekt in `pdf_has_pages` / `PdfHasPages` umbenannt.

Eine testseitige Abschlusspruefung sperrt die drei historischen Textheuristiken,
die alte `PDFStabil`-Korpusspalte und die irrefuehrende lokale Stabilitaetsbenennung.

## ChangeClass

`RefactorOnly`: ausschliesslich technische Bereinigung einer Diagnose-/Berichtsprojektion; keine Musik- oder Schreibsemantik.

## Revisionshinweis

r182 ist der eindeutige Nachfolger der abgelehnten/mehrdeutigen r181-Kandidaten. Die fachliche Bereinigung bleibt unveraendert; die Revision wird bewusst nicht wiederverwendet.
