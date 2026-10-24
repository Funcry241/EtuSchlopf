# r1240 Evidence → Proof → Shadow Permit Runtime Closure

## Grundlage und Grenze

Parent ist accepted r1239 (accepted-r1239-911f029fe8593011f74937677cadebbdea53aa826b08e466467795c2b4dabb79). Dieser Take implementiert dessen eingefrorene Runtime-Formen. Keine normative Gate-/Kodex-Änderung. Version bleibt 0.10.1839; RefactorOnly.

## Ereignisbindung und Lebensdauer

Die privaten Original-Mint-Punkte erfassen den exakten Pre-State und prüfen konstruktive Zertifikate am ursprünglichen Pre-/Post-Paar. Die vorhandene MintedIdentity wird weder rekonstruiert noch neu gemintet. Bei erfolgreicher Adoption wird das originale move-only Receipt im Beobachtungsspeicher behalten. ADDITIVE nutzt eine separate originäre Snapshot-Zuordnung anhand derselben opaque MintedIdentity; die eingefrorenen Receipt-Verifier bleiben unverändert. Unadoptierte ADDITIVE-Receipts räumen ihren Snapshot beim Drop ab.

Große Pre-States werden verlustfrei komprimiert. Die Rückgewinnung liefert exakt dieselben Bytes, keine XML-Normalisierung und keine Fingerprint-Identität. Im Report-Visit teilen TransformationEvidence, InvariantProof und NecessityWitness ein einziges lokales borrowed EventBinding. Der höher-rangige Callback liefert unit und lässt dessen Referenzen nicht entkommen. Die Identität wird im Report nicht offengelegt.

## Konstruktive positive Teilmenge

| Familie | Positiver Fall | Konservative Grenze |
| --- | --- | --- |
| F7 | Bestehender hidden rest wird sichtbar, übrige Struktur bleibt erhalten, seine strikte Voice-/Staff-Zeitspanne war nicht bereits sichtbar gedeckt | Chord/grace, ungültiger Cursor oder Überlappung liefern Unknown |
| F5 | Originaler gebundener Owner ergänzt fehlende Staff-Serialisierung; übrige Struktur bleibt erhalten | Umbelegung vorhandener Lane-Felder bleibt Unknown |
| F9 | Originaler Cursor-/Serialisierungs-Owner ergänzt fehlende Voice-/Staff-Serialisierung; übrige Struktur bleibt erhalten | Änderungen vorhandener Felder und andere Strukturänderungen bleiben Unknown |
| ADDITIVE | Originaler E004-Owner ergänzt genau eine fehlende finale rechte light-heavy-Barline; vorhandene Struktur bleibt erhalten | Andere additive Transformationen einschließlich WAV-Akzenten bleiben Unknown |

Die Zertifikate beweisen diese konkreten strukturellen Eigenschaften und den lokalen, bereits am Ursprung vorhandenen Serialisierungs-/Coverage-/Boundary-Defekt. Sie beweisen keine allgemeine musikalische Optimalität eines vom existierenden Owner gewählten Ziels. Familienzugehörigkeit allein erteilt keinen positiven Proof. WAV-Salience, Validator-Zähler, AuditOnly, ProofMath-Labels und Reporttext sind keine Zertifikatsquellen.

## Beobachtung und vollständige Zählbasis

Master-Adoption und Render-Adoption registrieren am gemeinsamen erfolgreichen Commit sämtliche Original-Owner-Receipts, einschließlich verschachtelter Render-Schritte. Verlierer, verworfene Trials und nicht adoptierte Receipts erzeugen keine adoptierte Beobachtung. Genesis und Render FromMaster sind Provenienz-Wurzeln ohne TransformationReceipt; sie werden ausdrücklich nicht als Owner-Transformationen ausgegeben oder künstlich mit Receipts versehen.

AdoptedOwnerEvents ist der unabhängige Nenner aller behaltenen erfolgreich committed Owner-Receipts. MasterOwnerEvents, RenderOwnerEvents und MissingOriginCapture werden unabhängig von visitierbaren Proofs gezählt. ObservedOwnerEvents sowie WouldPermit, WouldDeny und Unknown werden sichtbar ausgegeben. Ein fehlender oder nicht dekodierbarer Snapshot kann daher keine verdeckte Coverage-Vollständigkeit erzeugen: der Nenner bleibt stehen. 100 Prozent behauptet der Report nicht durch bloßes Vorhandensein von Proofs.

Der geschlossene Beobachter entspricht der r1239-Vorlage. WouldPermit entsteht ausschließlich bei gemeinsamem Binding; unvollständige Zertifikate ergeben Unknown. WouldDeny bleibt beobachtend und verändert keinerlei Release-Entscheidung. Erwartung für den realen 10er-Korpus: WouldDeny = 0 und ObservedOwnerEvents = AdoptedOwnerEvents; diese Ergebnisse werden hier nicht erfunden.

## Authority-Abschottung

Nur final_report konstruiert die private SscObservationReadAccess. Beide erlaubten unit-Report-/Measurement-Sinks benötigen ihre unveränderliche Referenz; auch der Proof-Visitor und Coverage-Leser erfordern diese Capability. Weder Writer noch Selector können diese Capability konstruieren. Es gibt keinen Verdict-bool-Accessor, Reexport, Verdict-Callback oder autorisierenden Rückwert. Die Verdict-Labels werden nur aus der geschlossenen internen unit-Serialisierung für Messzahlen gelesen, nie für Identität oder Proof-Erteilung.

Der Report wird nach bestehender Vertragsvalidierung ergänzt. Musikbytes, produktive Mutationen, Owner-Reihenfolge, Adoption, Release-Vetos und bestehende Auswahlentscheidungen werden nicht aus Shadow abgeleitet. feedback_proof_transport bleibt byteidentisch und NonLinkable; keine WriterEligible-Aufwertung. Alle 15 offenen N_A-Gruppen samt Ausgangsbilanz bleiben byteidentisch; kein Retirement und kein N_A = ∅.

## Prüfung und verbleibende Acceptance

Neue Tests umfassen echte F5-/F7-/E004-Writer-Ursprünge mit Master-Adoption, den echten privaten F9-Receipt-Mint mit Adoption, Render-Adoption und verworfene Trials, exakte Codec-Rückgewinnung sowie adversariale Proof-Grenzen. F9 testet den privaten originären Mint und dessen Adoption, nicht einen neuen alternativen Serialisierungsplaner. Bestehende Tests laufen vollständig mit.

Die reale Windows-Admission, Strict-Ausführung und das 10er-Korpus sind Bestandteil der anschließenden Take-Acceptance. Diese Umgebung enthält deren Original-FLP-/WAV-/Render-Inputs nicht. Lokale Prüfungen ersetzen diese Acceptance nicht; kein vorweggenommener 70-Prozent- oder Accepted-Claim.

## Zeilenbilanz

Rust-Source gegenüber accepted r1239: +1333 / −9 Zeilen, netto +1324. Die zusätzlichen Zeilen implementieren Beobachtung und Tests; dieser Take baut keine N_A-Gruppe zurück.
