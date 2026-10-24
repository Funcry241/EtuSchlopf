# Exakte Zulassung des gemeinsamen Vier-Writer-Rückbaus

KodexOnly auf r1485. Keine produktive Quelle und keine aktive Tabelle ändern sich.
Der vorbereitete vollständige GovernedChange-Nachfolger r1487 entfernt E006,
E008/E009, E047 und E048 einschließlich B2/G4/S3. Der !avail-Rückfall über B2
entfällt auch außerhalb des Korpus. E049/M5 und E050 bleiben bestehen.

Checkpoint58 bindet alte/neue Pfad-SHA-Mengen und ausdrückliche Abwesenheit.
Nur der gesamte Nachfolger ist zulässig; nach dem Übergang kein Rückweg.
Die bestehenden R57-, RSP-, K2- und Klassenprüfungen bleiben zuständig.
Der Trust-Anchor wird aus dem exakten Parentbild ausschließlich über die
namentlich eingefrorenen alten/neuen Runtime-Pin-Zeilen abgeleitet; alle
anderen Bytes müssen unverändert bleiben. Damit entsteht kein zirkulärer
Hash zwischen Checkpoint58 und dem Anchor, der Checkpoint58 selbst bindet.
Keine allgemeine Pin-Normalisierung und keine unabhängigen A-oder-B-Pins.

Abnahme: Master bytegleich, Render ausschließlich Leerzeilen-Unterschiede;
Writer-Berichtabschnitte entfallen, RenderLineageContract ändert sich,
Mint-Zähler sinken, Owner-Source-FNV ändert sich. Keine verlorene provisorische
Custody, Native-Versuche 1/7 ohne -f. MuseScore 4.7.5 ist mit Otter vereinbart;
ein zusätzlicher Parent-Renderlauf nur für den Rendererwechsel ist nicht verlangt.
