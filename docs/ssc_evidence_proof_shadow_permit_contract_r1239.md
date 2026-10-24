# r1239 — SscEvidenceProofShadowPermitContractKodex

Basis: accepted r1238 LegacyPhysicalRetirementDependencyClosure, Version 0.10.1839. Die jüngste übergebene Acceptance bestätigt r1238 mit Build/Clippy/FinalExit 0, 1676 Tests und 10/10 Korpus. Diese Parent-Nachweise sind keine Acceptance des vorliegenden Takes.

## Vertrag und Grenze

Dieser Take installiert ausschließlich Kodex: keine musikalische Mutation, keinen produktiven Permit, kein Enforcement, keinen Writer-Umbau und kein Retirement. Die Rust-Snippets in den .txt-Verträgen sind ausdrücklich nicht kompilierter Runtime-Code. Der reservierte Receipt-Adapter leiht ausschließlich die vorhandenen Block-, Exact-, Preserving- oder AdditiveTransformationReceipt-Typen. Er öffnet nicht die private interne TransformationReceipt-Transport-Enum. Die künftigen privaten Zertifikattypen sind reserviert, nicht implementiert und nicht ausgestellt. Ein späterer Implementierungs-Take muss ihre konstruktiven Issuer, semantischen Nachweise und produktiven Anschlussstellen gesondert herstellen und prüfen. Der jetzige Source-Gate ersetzt diesen Beweis nicht. Insbesondere müssen die späteren versiegelten Issuer Receipt-Ursprung, vorhandene Ereignisidentität und exakten Pre-State gemeinsam prüfen; öffentliche Carrier-/Zertifikat-Konstruktoren oder erneutes Minting sind nicht erlaubt.

Transformation-Evidence beschreibt die tatsächliche Änderung aus dem originären Receipt. Invariant-Proof belegt konstruktiv die erhaltenen Eigenschaften. Necessity-Witness belegt konstruktiv Defekt oder Verpflichtung, die den Eingriff erfordert. Alle drei leihen dasselbe EventBinding-Objekt, das bestehende owner-gemintete opaque Identität und den exakten unveränderlichen Pre-State bindet. Hash, Position, Reporttext und ein erneut geminteter Ersatz sind keine Ereignisidentität. Die Pointer-Gleichheit im reservierten Beobachter überprüft allein den gemeinsamen Borrow-Ursprung; sie ist keine persistierte Ereignisidentität.

Positive Zertifikate sind zunächst ausschließlich für F7 Rest/Voice/Strict-Staff, F5 atomare Voice/Staff/Visibility-Moves, F9 Authority/Serialization und ADDITIVE reserviert. Dies behauptet keine bereits vorhandene Proof-Fähigkeit. Für andere Familien beziehungsweise fehlende positive Zertifikate bleibt Unknown zulässig und nicht autorisierend. Vorhandene, aber unterschiedlich gebundene Proof/Witness-Paare ergeben WouldDeny. Nur gemeinsam gebundene positive Zertifikate können künftig WouldPermit ergeben. Auch WouldPermit erteilt keinerlei Schreib-, Auswahl-, Ranking-, Repair-, Release- oder Veto-Autorität.

ShadowPermitVerdict ist geschlossen: WouldPermit | WouldDeny | Unknown. Keine bool-Accessor-, Vergleichs-, Cast- oder Conversion-Schnittstelle. Verdict und Observation werden nur im reservierten Shadow-Core erzeugt und nur von src/ssc_shadow_report.rs beziehungsweise src/ssc_shadow_measurement.rs konsumiert. Ausschließlich src/final_report.rs darf deren reine unit-returning Append-Schnittstellen aufrufen. Root-Module-Wiring ist zulässig; sonstige Referenzen einschließlich Alias/Wildcard und literaler Include-/Path-Umgehungen aus produktiven Pfaden werden zurückgewiesen. Sink-Exporte dürfen keinen Entscheidungswert, Callback oder mutierbare Authority zurückgeben/entgegennehmen; nur ein String-Reportpuffer darf mutierbar sein.

WAV-Salience, Validator-Zähler, rekonstruierte ProofMath-Labels und AuditOnly-Daten sind keine konstruktiven Proofs. feedback_proof_transport bleibt bytegleich, NonLinkable und WriterEligible nein. Die künftige positive Zertifikaterzeugung darf diese Beobachtungsdaten nicht durch Umbenennung legitimieren.

## Restabhängigkeiten und Abschluss

ssc_exit_dependencies_r1238_v1.tsv ist die unveränderliche, maschinenlesbare Ausgangsmenge N_A(r1238); ssc_exit_remaining_v1.txt hält deren offenen Teil fest. Die 15 IDs zählen Abhängigkeitsgruppen, nicht Rust-Zeilen, Notenedits oder einzelne Population-Owner.

Enthalten sind #57/#67 deferred, #28 WAV pending, produktiver WAV-Akzentpfad, WAV-Salience-Master-Adoptionen, WAV-Metronom, E031-Tenuto, RA5/AccentReserve, fünf test-only Macro-Vertragszeugen, die akzeptierten Master*-Receipt-Legacy-Namensalternativen des Population-Gates und AuditOnly-Grandfathering im Carrier-Scan. Die Legacy-Receipt-Namen sind Gate-Vertragsalternativen; es werden keine existierenden Runtime-Master*-Receipt-Typen behauptet. Der aktuelle Korpus wendet weiterhin 105 WAV-Akzent-Edits mit finalen Receipts an; diese Population ist nicht tot.

N_A beschreibt hier verbleibende Legacy-Abhängigkeiten. Fehlende Proof-, Shadow-, Enforcement- und regenerative Abschlussfähigkeiten sind gesonderte SSC-Exit-Pflichten, keine Legacy-Code-Abhängigkeiten. Folgetakes dürfen N_A nur verkleinern, niemals neue/reopened IDs einführen. Dieser Vorbereitungs-Take erteilt jedoch keine Resolution-Capability: alle 15 bleiben OPEN. Ein späterer SSC-K2-Kodex muss die berechtigte, nachgewiesene Auflösung erlauben. Leere Menge, unbelegte Einzelstreichung und vorzeitiger SSC-Exit werden jetzt abgewiesen.

SSC wird erst nach tatsächlich konstruktivem Evidence→Proof→Shadow, gemessener Abdeckung, gesondertem Sovereign-Permit-Enforcement, legitimem K2-Retirement und dem Nachweis regenerativer Fortsetzung ohne alten Root geschlossen.

## Implementierung und Prüfung

Der neue Gate läuft im Incoming-Take-Pfad, im installierten Transaktionspfad und im regulären Build. Bestehende Kodex-Verträge bleiben unverändert. Neue Gate-/Test-/Vertragsbytes werden im bestehenden zentralen Trust-Anchor gepinnt; dessen vorhandene Build-/Take-Control-Pins werden aus den finalen Bytes koordiniert.

18 neue adversarielle Gate-Tests prüfen unter anderem Ereignis-/Typbindung, diagnostische Proof-Laundering-Versuche, fehlende/mismatched Proofs, bool-/Cast-/Comparison-Adapter, produktive Referenzen, Alias/Wildcard/geschachtelte Include-Umgehungen, Sink-Rückgaben/Callbacks/mutierbare Authority, Parent/Class-Bindung, monotone Mengen und verfrühte Runtime-/Retirement-/Exit-Claims.

Validierung wird vor Auslieferung mit Cargo Check, Strict Clippy, vollständiger Strict-All-Targets-Suite sowie Parent- und Candidate-Admission ausgeführt. Windows-Korpus und Acceptance sind im abschließenden realen Take auszuführen.
