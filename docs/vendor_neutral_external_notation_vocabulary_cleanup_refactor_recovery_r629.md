# r629 ExternalNotationVendorNeutralVocabularyCleanupRefactorRecovery

Stand: v0.10.1839 r629  
Parent: r627 `PitchedChordAnchorMutationDeltaInvariantKodexRecovery`  
ChangeClass: `RefactorOnly`

## Zweck

Die EtuSchlopf-Repositorysprache bleibt gegenueber externen Notationsprogrammen vendor-neutral. Zwei historische Chord-Architekturdokumente enthielten noch insgesamt drei Nennungen eines konkreten externen Produktnamens. r629 ersetzt ausschliesslich diese Metadatenformulierungen durch neutrale Begriffe fuer externen Notationsimport bzw. Importer-Sonderregeln.

Der inhaltlich identische r628-Kandidat wurde vor Installation durch Take-Change-Control verworfen: `MechanicalOnly` darf den historischen Metadatenpfad `docs/chord_rhythmic_cohort_atomic_staccato_closure_writer_reachability_recovery_r625.md` wegen seines Writer-Markers im Pfadnamen nicht aendern. r629 veraendert deshalb nicht den Cleanup-Inhalt, sondern verwendet die fuer diesen vorhandenen Metadaten-/Refactor-Fall zulaessige Klasse `RefactorOnly`. r628 besitzt keine Authority.

## Grenze

- kein Rust-Code geaendert;
- keine Tests geaendert;
- keine Capability geaendert;
- kein Writer/Call-Graph geaendert;
- keine MusicXML-, Render-, PDF- oder Musikmutation;
- repositoryweiter case-insensitiver Text- und Dateinamen-Readback fuer den entfernten Produktnamen: 0 Treffer.
