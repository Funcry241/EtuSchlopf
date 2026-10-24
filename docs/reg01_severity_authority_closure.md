# Reg01SeverityAuthorityClosure – v0.10.1617 r01

## Befund

`RIGHT_HAND_LOW_REGISTER` und `LEFT_HAND_HIGH_REGISTER` sind im typisierten
Validatorvertrag als Runtime-Hinweise definiert. Der zentrale RuleBook-Eintrag REG01
meldete dagegen WARNUNG. Dadurch enthielt jeder aktive REG01-Bericht gleichzeitig
`runtime_severity=HINWEIS` und `rule_severity=WARNUNG`, obwohl die formale
Freigabeautorität korrekt bei HINWEIS blieb.

## Änderung

Nur die deklarative RuleBook-Severity von REG01 wird von WARNUNG auf HINWEIS
ausgerichtet. Das Gewicht 850 und die Runtime-Severityprofile bleiben unverändert.
Die RuleBook-Severity wird im produktiven Quellstand ausschließlich in Issue- und
Auditberichten projiziert; sie steuert keine Kandidatenwahl, Mutation oder Freigabe.

## Erwarteter Nachweis

- `RuntimeVsRuleSeverityMismatches 0` bei aktivem REG01,
- `SeverityAuthority: gruen`,
- Ergebnis weiterhin OK mit einem Hinweis,
- Master-/Render-MusicXML nur in der Werkzeugversionszeile verändert,
- PDF pixelidentisch.
