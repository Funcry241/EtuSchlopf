# r279 – Register-Sichtzustands- und Anker-Planungsbeweis

## Ausgangspunkt

r277 hat den semantischen `RegisterNotationPlan` produktiv materialisiert und alle internen Plan-/Start-/Stop-Invarianten erfuellt. Die menschliche PDF-Sichtpruefung hat diese Freigabe jedoch falsifiziert: nackte Einzel-`8`, fehlend wirkende, doppelte und ueberlappende Oktavangaben zeigen, dass semantische Planrichtigkeit nicht mit sichtbarer Notationsrichtigkeit gleichgesetzt werden darf.

r279 ist deshalb `AuditOnly` und basiert direkt auf r277. Der noch nicht real ausgefuehrte r278-Kandidat wird nicht als Elternstand verwendet und soll durch r279 supersediert werden.

## Drei getrennte Fachschichten

```text
RegisterNotationPlan
        ↓
sichtbarer Registerzustandsverlauf
        ↓
MusicXML-Ereignisanker + Linienidentitaet + grafische Lage
        ↓
erst spaeter: StructuralWriteAuthority
```

### 1. Sichtbarer Registerzustandsverlauf

Direkt aneinanderstossende semantische Plaene gleicher Darstellung werden in der Auditprojektion zwingend zu einer Sichtspanne vereinigt. Diese Fusion erweitert keine Ereignisabdeckung, weil zwischen Stop und Start keine Zeit liegt.

Treffen unterschiedliche Darstellungen am selben Zeitpunkt aufeinander, bleibt der Zustandswechsel offen. r279 besitzt keine lokale Gewichtungsregel, keine Schwelle und keine Etueden-/Taktsonderregel, um beispielsweise `15ma -> 8va` automatisch zu entscheiden.

Zeitliche Luecken gleicher Darstellung ohne Tonereignis werden nur als Kandidaten fuer sichtbare Kontinuitaet erfasst. Es gibt keine automatische Stille-Brueckenregel.

### 2. Ereignisanker

Eine sichtbare Oktavlinie soll spaeter nicht mehr nur ueber einen semantischen `<offset>` an einer entfernten XML-Position beschrieben werden. r279 prueft deshalb fuer jede Sichtspanne, ob am geplanten Beginn ein Tonereignis desselben Systems beginnt und am geplanten Ende ein Tonereignis desselben Systems endet.

Dieser Beweis schreibt noch keinen Anker. Er macht nur sichtbar, welche Grenzen ohne neue musikalische Entscheidung an reale Ereignisse gebunden werden koennen und welche Grenzen noch einer allgemeinen Ankerregel beduerfen.

### 3. Linienidentitaet und Geometrie

MusicXML-`number` ist keine Systemnummer. Sie wird fuer die Identitaet gleichzeitig aktiver Linien benoetigt. r279 berechnet deshalb die maximale Gleichzeitigkeit sichtbarer Spannen aus ihren Intervallen und beobachtet separat, wie oft die aktuelle r277-Ausgabe `number == staff` verwendet.

Ebenso werden Start und Stop jeder aktuellen Oktavlinie auf `placement`, `default-y` und `size` verglichen. Damit wird sichtbar, wenn eine spaetere Layoutstufe nur den Start verschiebt und Start/Stop derselben Linie geometrisch auseinanderlaufen.

## Harte Nicht-Ziele

r279:

- veraendert keine MusicXML-Datei;
- veraendert keinen Registerplan;
- veraendert keinen Writer und keine StructuralWriteAuthority;
- fuegt keine Takt- oder Etuedensonderregel hinzu;
- fuegt keine neue Oktavierungsgewichtung oder Schwelle hinzu;
- betrachtet gedruckte Taktkennungen nur als bedeutungslose XML-Metadaten;
- benutzt den Partiturtitel ausschliesslich zur Dateiauswahl fuer den Audit.

## Freigabevertrag

Ein spaeterer produktiver Take darf erst entstehen, wenn der schreibfreie Korpusbeweis zeigt:

1. der sichtbare Zustandsverlauf ist ohne unentschiedene lokale Fragmentierung geschlossen;
2. alle produktiv zu schreibenden Grenzen besitzen einen allgemeinen ereignisgebundenen Ankervertrag;
3. Linienidentitaet wird aus Ueberlappung und nicht aus der Systemnummer abgeleitet;
4. Start und Stop einer Linie besitzen eine koharente grafische Lage;
5. die PDF-Sichtpruefung ist ein eigener Freigabebeweis und nicht durch Planzaehler ersetzbar.
