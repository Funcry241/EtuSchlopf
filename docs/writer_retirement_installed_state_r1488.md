# Installierter Writer-Rückbau ohne Dauer-Freeze lebender Dateien

KodexOnly auf r1487. Keine Laufzeitquelle und keine aktive Tabelle ändert sich.

r1486 band den installierten Rückbauzustand an das vollständige r1487-Tupel. Damit
waren auch lebende Dateien (M5, E050-Pausenbrücke, Metrik-Stufe, Identität,
Writer-Spur, Tests) dauerhaft byteweise eingefroren. Das war nicht beabsichtigt.

Neu gilt im installierten Zustand, festgestellt allein aus dem Inhalt:

- Die sechs Kodex-Datentabellen (Exclusions, Surface-Baseline, Legacy-Selection,
  Origins, Envelopes, Proof-Bundle-Vertrag) bleiben exakt auf ihrem r1487-Bild.
- Die sechs gelöschten Dateien bleiben abwesend.
- Die Dateien, die schon vor r1486 exakte Checkpoint58-Nachfolger waren
  (FinalNotation-Support, SWA-Modul, Sink-Authority, Render-Adoption samt Tests),
  bleiben exakt. Damit bleibt auch die #57-Bindung von `render_adopted_state.rs` exakt.
- Die Bezeichner der entfernten Authority (Sinks, B2-Zweig/-Fork, Mints,
  Deskriptoren, Einstiegsfunktionen) fehlen in allen Laufzeitquellen. Geprüft wird
  Code, nicht Kommentare oder Strings.

Alle übrigen lebenden Dateien unterliegen wieder nur ihren gewöhnlichen Gates.
Der historische Erstübergang r1486 -> r1487 verlangt unverändert das vollständige
Tupel, Identitäten, Pfadmenge und den exakt parentabgeleiteten Anchor. Kein Rückweg,
keine neue Befugnis durch Marker oder Metadaten, keine allgemeine Ausnahme.

Bekannte Abhängigkeit: Die semantische Öffnung der sechs Kodex-Daten in
`semantic_capability_diff.rs` wird allein durch die unabhängig prüfende Pfadpolitik
begrenzt; in GovernedChange sind sie nur im gebundenen Erstübergang zulässig.
