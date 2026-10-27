# Community-Dateien auf GitHub veröffentlichen

## Zwei Paketrollen

Die gepflegten Community-Texte liegen im Take unter `docs/`. GitHub kann
Beitragsregeln, Verhaltenskodex, Sicherheitsrichtlinie und PR-Vorlage dort finden.
Die README verlinkt zusätzlich die Erklärung zur Barrierefreiheit.

Issue-Vorlagen brauchen dagegen `.github/ISSUE_TEMPLATE/` auf dem Default-Branch.
Dieser Ordner gehört nicht zum bestehenden verwalteten Take-Baum. Seine
Veröffentlichung ist deshalb eine Git-spezifische Projektion der folgenden
gepflegten Vorlagen, keine Erweiterung des Take-Vertrags:

| Quelle im akzeptierten Quellstand | Ziel im Git-Checkout |
| --- | --- |
| [01_fehler.md](issue_templates/01_fehler.md) | `.github/ISSUE_TEMPLATE/01_fehler.md` |
| [02_wunsch.md](issue_templates/02_wunsch.md) | `.github/ISSUE_TEMPLATE/02_wunsch.md` |

Die Zielbytes müssen den Quellen entsprechen. Bei späteren Änderungen zuerst die
gepflegte Quelle ändern, danach dieselben Bytes veröffentlichen. Das mitgelieferte
separate GitHub-Paket enthält diese beiden Zielpfade. Es ist kein Take und darf
nicht mit `--take` angewendet werden. Sein Inhalt gehört nur in den Git-Checkout.

## Reihenfolge

1. Community-Take auf seinem angegebenen akzeptierten Parent nativ prüfen und
   annehmen. Bei einem inzwischen neueren lokalen Stand zuerst neu aufsetzen;
   keinen älteren vollständigen Baum darüber einspielen.
2. Den angenommenen verwalteten Stand in den Git-Checkout übernehmen. Bestehende
   Git-Dateien wie `LICENSE` und `.gitignore` erhalten; Root-`Cargo.lock` gemäß dem
   bestehenden Git-Ablauf behandeln. Den lokalen Target-/Evidence-Baum nicht
   veröffentlichen. Ein natives Take-ZIP enthält keinen Root-`Cargo.lock`.
3. Die beiden Issue-Vorlagen an die oben genannten Git-Ziele projizieren oder das
   separate GitHub-Paket dafür verwenden. Den Diff prüfen und als gemeinsame
   Community-Änderung veröffentlichen. Ein Branch/PR macht den Diff prüfbar;
   aktiviert werden die Vorlagen erst auf dem Default-Branch.
4. Die [Repository-Beschreibung](repository_description.txt) als GitHub-Einstellung
   setzen. Dieser Text wird durch ein Take oder einen Commit nicht automatisch
   zur Repository-Beschreibung.
5. In den Repository-Einstellungen private Sicherheitsmeldungen prüfen und nach
   Möglichkeit aktivieren. Die aktuelle Integration kann diese Einstellung nicht
   abfragen oder setzen. Keine Aktivierung aus einer vorhandenen `SECURITY.md`
   ableiten. Die Richtlinie enthält einen Ersatzweg ohne öffentliche Lückendetails.
6. Nach Veröffentlichung README-Links, Issue-Auswahl, PR-Vorlage und Community-Profil
   auf GitHub prüfen. Vorhandene Dateien mit höherer GitHub-Suchpriorität dürfen
   die neuen `docs/`-Dateien nicht durch ältere Inhalte verdecken.

## Grenzen der Kontaktwege

Issues sind öffentlich. GitHubs private Sicherheitsmeldungen sind ausschließlich
für technische Sicherheitslücken vorgesehen. Für vertrauliche
Verhaltensbeschwerden ist kein eigener privater Projektkanal zugesagt; der
Verhaltenskodex beschreibt eine neutrale Kontaktanfrage und getrennt den
GitHub-eigenen Missbrauchsmeldeweg. Keine fiktive E-Mail-Adresse einsetzen.

## GitHub-Dokumentation

- [Community-Dateien](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file)
- [Issue- und PR-Vorlagen](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/about-issue-and-pull-request-templates)
- [Private Sicherheitsmeldungen](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately)
