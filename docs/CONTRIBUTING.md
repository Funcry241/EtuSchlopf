# Zu EtuSchlopf beitragen

EtuSchlopf ist ein experimentelles, privat gepflegtes Projekt. Beiträge und
Fehlerberichte sind willkommen; eine Übernahme oder Bearbeitungsfrist ist nicht
zugesagt. Die Projektsprache ist Deutsch. Es gibt keine garantierten
Antwortzeiten, Fehlerbehebungen oder Unterstützung älterer Versionen.

## Zuerst den passenden Weg wählen

- Fehler bitte über die [GitHub-Issues](https://github.com/Funcry241/EtuSchlopf/issues) melden.
  Suche vorher nach einem passenden bestehenden Bericht und ergänze ihn gegebenenfalls.
- Größere Änderungen, neue Funktionen, Architekturänderungen und Änderungen an
  Prüfregeln zuerst in einem Issue besprechen, bevor du viel Arbeit investierst.
  Die Entscheidung über Umfang und Übernahme liegt beim Repository-Inhaber.
- Sicherheitslücken nach der [Sicherheitsrichtlinie](SECURITY.md) melden.
- Für den Austausch gilt der [Verhaltenskodex](CODE_OF_CONDUCT.md).

## Einen hilfreichen Fehlerbericht schreiben

Nenne Version und Revision, Betriebssystem, genauen Aufruf, Schritte zur
Reproduktion, erwartetes und tatsächliches Ergebnis. Bei Notationsproblemen
helfen ein kleiner Ausschnitt und eine Erklärung der musikalischen Erwartung.
Ein grüner Lauf und ein gut lesbares Notenbild sind unterschiedliche Aussagen.
Ein konkreter Takt darf im Fehlerbericht lokalisiert werden.

Verwende möglichst ein kleines selbst erstelltes Beispiel. Teile nur Musik und
Dateien, die du veröffentlichen darfst. Prüfe Logs, Screenshots und Exporte vor
öffentlichem Hochladen auf personenbezogene Daten und private Pfade. Ein
vollständiger Assistant-Export kann Quellcode, Berichte und weitere lokale
Informationen enthalten; er ist keine Voraussetzung für einen öffentlichen Bericht.

## Einrichten und ausführen

Die [technische Referenz](DEVELOPMENT.md) beschreibt Eingaben, Ausgaben und die aktuelle
Aufrufreferenz. Die maßgebliche Rust-Toolchain samt Komponenten steht in
[rust-toolchain.toml](../rust-toolchain.toml); Abhängigkeiten stehen in
[Cargo.toml](../Cargo.toml) und dem lokalen beziehungsweise im Git-Checkout
mitgeführten Root-Lockfile. Für PDF-Ausgaben ist ein externer Renderer nötig.

Der gepflegte Produktablauf läuft unter Windows aus der Projektwurzel:

```cmd
.\run.cmd "Mein Projekt"
.\run.cmd --all
```

Der erste Befehl verarbeitet das eigene WAV-/MIDI-Paar; der zweite alle erkannten
Paare. Das ursprüngliche Evaluationskorpus wird nicht mitgeliefert. Ein frischer
Checkout ohne eigene Eingaben kann deshalb keinen solchen Korpuslauf liefern.
Andere Plattformen werden hier nicht als vollständig unterstützter Produktablauf
zugesagt. Die separaten Python-Diagnosen und ihre Voraussetzungen sind in ihrer
[eigenen Anleitung](../tools/assistant_diagnostics/README.md) dokumentiert.

## Änderungen und Prüfungen

Halte einen Pull Request auf eine nachvollziehbare Aufgabe begrenzt. Beschreibe
Problem, Ursache, betroffene Zuständigkeit und erwartete Auswirkungen. Bestehende
Verträge und Gates sind im aktuellen Quellstand nachzulesen; eine Dokumentation
von Abläufen erteilt keine zusätzliche Änderungsbefugnis.

Der reguläre Launcher prüft standardmäßig Strict-Clippy, Build und Tests. Die
[Aufrufreferenz](DEVELOPMENT.md#vollständige-aufrufreferenz) und der aktuelle
[Launcher](../src/bin/etuschlopf_launcher.rs) bestimmen den genauen Prüfvertrag.
Gib im Pull Request an, welche Prüfungen du tatsächlich ausgeführt hast und
welche wegen fehlender Plattform, Eingaben oder Renderer noch ausstehen.
Behaupte keine vollständige native Prüfung aus einem einzelnen Cargo-Aufruf.

Bei Codeänderungen brauchen normative Tests synthetische, vom realen Korpus
unabhängige Beispiele. Reale Musik dient der Evaluation. Der
[Architektureinstieg](assistant_memory/000_start_here.md) führt zu den bestehenden
Verträgen und erläutert diese Trennung. Notationsänderungen brauchen zusätzlich
eine Bewertung der tatsächlich erzeugten XML-/PDF-Ergebnisse.

## Git und Takes

Ein Git-Commit, ein Quellarchiv und ein akzeptierter Take sind verschiedene
Identitäten. Ein GitHub-ZIP ist nicht automatisch installierbar. Der Take-Ablauf
bleibt bei `.\run.cmd --take --all` mit einem dafür deklarierten vollständigen
Paket und dem passenden akzeptierten Parent. Ein normaler Git-Beitrag muss nicht
so tun, als sei er bereits ein angenommener Take.

Die Veröffentlichung von GitHub-spezifischen Vorlagen ist im
[Veröffentlichungsablauf](community/GITHUB_PUBLISHING.md) beschrieben.
