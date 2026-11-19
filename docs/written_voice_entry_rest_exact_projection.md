# Teilweise vorhandene Anfangspausen vervollständigen

Die Projektion verwendet den [Vertrag für metrische Stimmeinsätze](selector_derivation_contract_written_voice_entry_silence.md).

- Der Planer zieht vom gebundenen Anfangsintervall die vereinigte Dauer der bereits sichtbaren Pausen dieser Stimme ab.
- Der metrische Pausenplaner zerlegt jedes verbleibende Intervall in exakt darstellbare Dauern. Die Projektion übernimmt den vollständigen Plan, sobald alle Teilintervalle darstellbar sind.
- Beim Schreiben überbrückt ein Zeitcursor die bereits notierten Abschnitte. Der Cursor erreicht danach genau die ursprüngliche Zeitposition des Stimmeinsatzes.

Ein zweiter Durchlauf erkennt die nun vollständige Abdeckung. Tonhöhe, Stimme, System, Einsatzzeit und Dauer der klingenden Noten bleiben erhalten.
