# NonSecurityEffectiveVisibilitySuperClosure

## Ausgangspunkt

Autoritative Basis ist der maschinell gruen bestaetigte v0.10.1657-r02-Stand.
Stufe 1 hat 153 echte Deklarationen der Security-/Build-Gate-Control-Plane auf
effektive Sichtbarkeit normalisiert. Der normale Strict-Lauf fuer Etuede Nr. 4
endete mit Build 0, Clippy 0 und FinalExit 0; seine sechs PDF-Seiten sind gegen
v0.10.1656 pixelidentisch.

## Physischer Scope

Stufe 2 bearbeitet ausschliesslich die verbliebenen 362 echten
`pub(crate)`-Deklarationen in 51 Non-Security-Dateien:

- `src/structural_write_authority/`: 283 Deklarationen
- `src/audit_only/`: 14 Deklarationen
- uebrige Non-Security-Module: 65 Deklarationen

Jede Fundstelle wird nur von `pub(crate)` auf `pub` umgeschrieben. Zeilenzahl,
Attribute, Itemname, Generics, Parameter, Rueckgabetyp, Funktionskoerper,
Initialisierer und Modulinhalte bleiben unveraendert.

## Effektive Sichtbarkeit

Die betroffenen Items liegen hinter unveraenderten privaten Modulgrenzen oder
in den internen Grenzen des Binaercrates. `pub(crate)` und `pub` besitzen dort
fuer alle bestehenden Aufrufer dieselbe effektive Erreichbarkeit. Der Take
fuegt keinen neuen `pub mod`-Pfad an einer extern erreichbaren Bibliothekswurzel
hinzu und erzeugt keine neue Package-API.

## Schutzgrenzen

Nicht veraendert werden:

- Security-, Bootstrap-, Launcher- und Build-Gate-Dateien
- Trust Anchors, FNV-Pins und Scannervertraege
- Musikregeln, Schwellen und Passreihenfolge
- Writer- oder Materialisierungsentscheidungen
- MusicXML-, Render- und PDF-Vertraege
- Reportinhalte und Releaseentscheidungen

Es wird kein Lint erlaubt und kein `cargo fix` eingesetzt.

## Beweisplan

Vor Paketierung werden reale Diff-Pfade, Change-Class, Restvorkommen,
Security-Abwesenheit, Allow-Bilanz, physische Rust-Zeilen, Manifest-FNVs und
ZIP-Bytegleichheit statisch geprueft. Der Nutzerlauf liefert Build-, Strict-
und Etuedenbeweis. Ein anschliessender Deep-Clippy-Lauf bestimmt den exakten
All-Targets-Abbau und bestaetigt die vollstaendige Familienclosure.
