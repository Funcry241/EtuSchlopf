# r1031 – ProxyIdentityTargetConditionedSelectorKodexClosure

## Zweck

r1031 schliesst eine Governance-Luecke, die nach dem technisch gruenen, fachlich rejected r1030 sichtbar wurde: Ein Selektor kann syntaktisch ohne Projekt-/Taktidentitaet auskommen und trotzdem semantisch eine Proxy-Identitaet bilden, wenn Merkmale nach Kenntnis eines Ziel-Witness so lange nachgeschaerft werden, bis genau dieser Fall uebrig bleibt.

## Parent- und Recovery-Semantik

Autoritativer logischer Parent bleibt r1029 r02 `NotationVoiceMaterializationSafetyConjunctionAudit`. r1030 `PostRestThreeBeatCrossVoiceBeamDebtIntersectionAudit` ist fachlich rejected Failure Evidence. Der technisch gruen gelaufene r1030-Kandidat hatte SHA-256 `87189bcb735e75a0441f612788ea5a4a8a4b1897978d5af3ca993d194fa0b0e0`; dieser Hash bleibt nur Rejection-/Failure-Evidence und besitzt keine Parent-Authority. Weil r1030 technisch installiert wurde, deklariert dieser Take den physischen Rueckbau seiner Audit-Datei und Dokumentation ausdruecklich als Teil des Incoming-Diffs; daraus entsteht keine r1030-Parent-Authority.

## Neue Kodex-Regel

Selector-sensitive neue Entscheidungslogik ausserhalb des Kodex muss kuenftig einen geaenderten `docs/selector_derivation_contract_*.md`-Vertrag mit `SelectorDerivationContract: causal_class_first_v1` tragen. Der Vertrag verlangt insbesondere:

- bekannte Witnesses werden nicht zur Merkmalsableitung benutzt;
- keine target-conditioned Merkmalsverengung;
- Fehlerklasse vor Korpusverteilung;
- Einzigartigkeit als Ergebnis, nicht als Ziel;
- explizite Kausalmerkmal-Pruefung und explizit ausgeschlossene nichtkausale Kontextmerkmale;
- Fehlerklassendefinition, Klassifikationscode und Repair-Scope zeitlich getrennt;
- die kausale Fehlerklasse muss zuerst **ohne selector-sensitiven neuen Code** als `error_class_definition` dokumentiert und in einem akzeptierten Parent eingefroren werden;
- erst ein spaeterer `error_classification`-Take darf diese eingefrorene Klasse als Code abbilden und die Korpusverteilung messen;
- ein Repair-Scope darf ebenfalls nur auf einen bereits im installierten Parent vorhandenen Error-Class-Vertrag aufsetzen;
- Klassifikationscode darf nur die eingefrorene kausale Klasse implementieren; Repair-Narrowing nur aus Authority-/Safety-/Materialization-/Invarianten-Gründen, niemals aus rein diskriminierendem Kontext.

Die Erkennung ist bewusst ein frueher Governance-Trigger, kein semantischer Musikbeweis: Sie erkennt neu hinzugefuegte selector-sensitive Decision-Control-Logik und erzwingt einen Implementierungsvertrag, dessen Error-Class-Definition bereits aus dem installierten Parent stammen muss. Damit kann eine bekannte Korpusverteilung nicht im selben Take zur nachtraeglichen Merkmalsoptimierung benutzt werden. Die fachliche Wahrheit der kausalen Klassendefinition bleibt weiterhin durch aktuelle Authorities, Evidenz und Review zu belegen.

## Essenz

E09, E15 und E24 werden geschaerft. Die Cold-Start-Essenz sagt nun ausdruecklich: Identity-free Syntax ist nicht identity-free Semantik; Fehlerklasse zuerst, Korpusverteilung danach; Repair-Scope erst separat und nur aus unabhaengig begruendeten Sicherheits-/Authority-Grenzen. Das historische r1030-Gegenbeispiel wird als bekanntes Fehlerbild 28 dokumentiert.

## Keine Musik-/Writer-Aenderung

Dieser Take fuehrt keine MusicMutation, keinen Writer, keine Repair-, Selection- oder Release-Authority ein und veraendert keine produktive Musiklogik. Die r1030-Audit-Verengung wird auf die r1029-Bytes zurueckgesetzt.
