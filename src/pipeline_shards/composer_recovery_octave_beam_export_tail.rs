
/// Result of the octave notation policy: the BLOCK #50 receipts of its bound
/// measure replacements, in application order.
#[derive(Debug)]
struct OctaveNotationPolish {
    block_receipts: master_state_identity::OwnerReceipts,
}

#[derive(Debug)]
struct LowerStaffClefPolish {
    done_fragment: String,
    /// BLOCK #49 receipts of the measure replacements, in application order.
    block_receipts: master_state_identity::RecordedClefReceipts,
    rhythmic_transport: anyhow::Result<crate::arrangement::representation_transport::RhythmicRepresentationTransport>,
}

fn count_existing_lower_staff_clef_switches(xml: &str) -> usize {
    xml.matches("<clef number=\"2\">").count()
}

fn find_next_measure_bounds(xml: &str, from: usize) -> Option<(usize, usize, usize)> {
    let marker = "<measure number=\"";
    let start = from + xml[from..].find(marker)?;
    let number_start = start + marker.len();
    let number_end = number_start + xml[number_start..].find('"')?;
    let measure_number = xml[number_start..number_end].trim().parse::<usize>().ok()?;
    let close_rel = xml[start..].find("</measure>")?;
    Some((start, start + close_rel + "</measure>".len(), measure_number))
}

const LOWER_STAFF_TREBLE_CLEF_FROM_MIDI: i32 = 69; // eingestrichenes a / A4

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RegisternotationsGrenzart {
    Stop,
    Continue,
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RegisternotationsRichtung {
    teil: String,
    takt_ordinal: usize,
    system: String,
    einsatz: i64,
    art: RegisternotationsGrenzart,
    darstellung: crate::registernotation::RegisternotationsDarstellung,
    linie: usize,
    origin: Option<String>,
}

fn apply_octave_notation_policy(xml: &mut String) -> OctaveNotationPolish {
    (|| -> anyhow::Result<OctaveNotationPolish> {
        if xml.contains("<octave-shift") {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: Vor-K00-Quelle enthaelt bereits produktive Oktavrichtungen"
            );
        }

        let auftrag = crate::registernotation::sichtmaterialisierungsauftrag()
            .map_err(|fehler| anyhow!("RegisternotationsSichtmaterialisierung: {fehler}"))?;
        let erwartete_sichtlinien = auftrag
            .semantische_plaene
            .saturating_sub(auftrag.kontaktfusionen)
            .saturating_sub(auftrag.stille_bruecken);
        if erwartete_sichtlinien != auftrag.plaene.len() {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: Sichtzustandspfad ist nicht geschlossen"
            );
        }
        let sichtwiederholungen = auftrag
            .plaene
            .iter()
            .map(|plan| plan.sichtwiederholungen.len())
            .sum::<usize>();
        if sichtwiederholungen != auftrag.stille_bruecken {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: stille Bruecken und Continue-Cues weichen ab"
            );
        }
        let mut richtungen = registernotationsrichtungen_aus_sichtauftrag(xml, &auftrag.plaene)?;
        let births = richtungen.iter().map(|r| crate::registernotation::reader_policy::Birth {
            part: r.teil.clone(), staff: r.system.clone(), slot: r.takt_ordinal, onset: r.einsatz,
            kind: match r.art { RegisternotationsGrenzart::Stop => "stop", RegisternotationsGrenzart::Continue => "continue",
                RegisternotationsGrenzart::Start => match r.darstellung {
                    crate::registernotation::RegisternotationsDarstellung::Achtvb | crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb => "up", _ => "down" } }.to_owned(),
            display: r.darstellung, line: r.linie,
        }).collect::<Vec<_>>();
        let origins = crate::registernotation::reader_policy::issue_origins(&auftrag, &births);
        for (direction, origin) in richtungen.iter_mut().zip(origins) {direction.origin=Some(origin);}

        let vorher = xml.clone();
        let mut nach_takt = BTreeMap::<(String, usize), Vec<RegisternotationsRichtung>>::new();
        for richtung in &richtungen {
            nach_takt
                .entry((richtung.teil.clone(), richtung.takt_ordinal))
                .or_default()
                .push(richtung.clone());
        }

        let dokument = roxmltree::Document::parse(xml)
            .map_err(|fehler| anyhow!("RegisternotationsSichtmaterialisierung: XML ungueltig: {fehler}"))?;
        let mut edits = Vec::<(usize, usize, String, crate::arrangement::representation_transport::RepresentationStep)>::new();
        let mut gefundene_takte = BTreeSet::<(String, usize)>::new();
        for teil in dokument
            .root_element()
            .children()
            .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "part")
        {
            let teilkennung = teil
                .attribute("id")
                .ok_or_else(|| anyhow!("RegisternotationsSichtmaterialisierung: Teilkennung fehlt"))?
                .to_string();
            for (takt_index, takt) in teil
                .children()
                .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "measure")
                .enumerate()
            {
                let takt_ordinal = takt_index.saturating_add(1);
                // Die gedruckte MusicXML-Taktkennung bleibt ausschliesslich als
                // diagnostischer FeedbackIdentityChannel erhalten. Die produktive
                // Registernotationsentscheidung ist nur an den strukturellen
                // Taktordinal und die bewiesenen Ereignisanker gebunden.
                let measure_number = takt.attribute("number").unwrap_or("?");
                let schluessel = (teilkennung.clone(), takt_ordinal);
                let Some(taktrichtungen) = nach_takt.get(&schluessel) else {
                    continue;
                };
                let bereich = takt.range();
                let original = xml
                    .get(bereich.clone())
                    .ok_or_else(|| anyhow!("RegisternotationsSichtmaterialisierung: Taktbereich ungueltig"))?;
                if original.contains("<octave-shift") {
                    anyhow::bail!(
                        "RegisternotationsSichtmaterialisierung: konkurrierende Oktavrichtung in Teil {} Takt {} (Ordinal {})",
                        teilkennung,
                        measure_number,
                        takt_ordinal
                    );
                }
                let (ersatz, journal) = registernotationsrichtungen_in_takt(original, taktrichtungen)?;
                if ersatz == original {
                    anyhow::bail!(
                        "RegisternotationsSichtmaterialisierung: Materialisierung erzeugte keinen Taktunterschied in Teil {} Takt {} (Ordinal {})",
                        teilkennung,
                        measure_number,
                        takt_ordinal
                    );
                }
                edits.push((bereich.start, bereich.end, ersatz, journal));
                gefundene_takte.insert(schluessel);
            }
        }
        drop(dokument);

        if gefundene_takte.len() != nach_takt.len()
            || nach_takt.keys().any(|schluessel| !gefundene_takte.contains(schluessel))
        {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: mindestens eine Sichtlinie verweist auf keinen gebundenen Takt"
            );
        }

        edits.sort_by_key(|links| std::cmp::Reverse(links.0));
        let mut block_receipts = master_state_identity::OwnerReceipts::default();
        for (start, end, ersatz, journal) in edits {
            let before = xml.clone();
            let (replaced, receipt) =
                crate::structural_write_authority::replace_bound_octave_measure_authorized(
                    xml.as_str(),
                    start,
                    end,
                    ersatz,
                );
            *xml = replaced.ok_or_else(|| {
                anyhow!(
                    "RegisternotationsSichtmaterialisierung: StructuralWriteAuthority lehnte gebundenen Oktavtakt ab"
                )
            })?;
            block_receipts.push_octave_block_with_transport(receipt,
                journal.enclosed(&before, xml, start..end));
        }

        pruefe_registernotationsmaterialisierung(&vorher, xml, &richtungen)?;

        crate::registernotation::beobachte_materialisierung(
            crate::registernotation::RegisternotationsMaterialisierungsZeugnis {
                semantische_plaene: auftrag.semantische_plaene,
                sichtlinien: auftrag.plaene.len(),
                starts: auftrag.plaene.len(),
                continues: richtungen
                    .iter()
                    .filter(|richtung| richtung.art == RegisternotationsGrenzart::Continue)
                    .count(),
                stops: auftrag.plaene.len(),
                semantischer_plan_fingerabdruck: auftrag.semantischer_plan_fingerabdruck,
                sicht_fingerabdruck: auftrag.sicht_fingerabdruck,
            },
            richtungen.iter()
                .filter(|richtung| richtung.art == RegisternotationsGrenzart::Continue)
                .map(|richtung| crate::registernotation::OttavaCarrier {
                    part: richtung.teil.clone(),
                    staff: richtung.system.clone(),
                    measure: richtung.takt_ordinal,
                    onset: richtung.einsatz,
                    display: richtung.darstellung,
                    line: richtung.linie,
                })
                .collect(),
        )
        .map_err(|fehler| anyhow!("RegisternotationsSichtmaterialisierung: {fehler}"))?;

        crate::registernotation::reader_policy::confirm_materialization(xml)?;
        crate::audit_only::destructive_edit_attribution::observe_selected_state(xml);
        Ok(OctaveNotationPolish { block_receipts })
    })()
    .unwrap_or_else(|fehler| {
        panic!("RegisternotationsSichtmaterialisierung fehlgeschlagen: {fehler}")
    })
}

fn registernotationsrichtungen_aus_sichtauftrag(
    xml: &str,
    plaene: &[crate::registernotation::RegisternotationsMaterialisierungsPlan],
) -> anyhow::Result<Vec<RegisternotationsRichtung>> {
    let dokument = roxmltree::Document::parse(xml).map_err(|fehler| {
        anyhow!(
            "RegisternotationsSichtmaterialisierung: Dokumentordnung nicht lesbar: {fehler}"
        )
    })?;
    let mut nach_teil = BTreeMap::<
        String,
        Vec<(
            &crate::registernotation::RegisternotationsMaterialisierungsPlan,
            usize,
            usize,
        )>,
    >::new();
    for plan in plaene {
        let (erste_grenze, zweite_grenze) =
            registernotations_dokumentintervall(xml, &dokument, plan)?;
        nach_teil
            .entry(plan.teil.clone())
            .or_default()
            .push((plan, erste_grenze, zweite_grenze));
    }

    let mut richtungen = Vec::with_capacity(plaene.len().saturating_mul(2));
    for (_teil, mut teilplaene) in nach_teil {
        teilplaene.sort_by_key(|(plan, erste_grenze, zweite_grenze)| {
            (
                *erste_grenze,
                *zweite_grenze,
                plan.system.clone(),
                plan.darstellung,
            )
        });
        let mut aktiv = Vec::<(usize, usize)>::new();
        let mut belegungen = Vec::<(usize, usize, usize)>::new();
        for (plan, erste_grenze, zweite_grenze) in teilplaene {
            aktiv.retain(|(dokumentende, _linie)| *dokumentende > erste_grenze);
            let belegt = aktiv
                .iter()
                .map(|(_dokumentende, linie)| *linie)
                .collect::<BTreeSet<_>>();
            let linie = (1_usize..=16)
                .find(|nummer| !belegt.contains(nummer))
                .ok_or_else(|| {
                    anyhow!(
                        "RegisternotationsSichtmaterialisierung: mehr als 16 in MusicXML-Dokumentordnung ueberlappende Oktavlinien"
                    )
                })?;
            aktiv.push((zweite_grenze, linie));
            belegungen.push((erste_grenze, zweite_grenze, linie));

            richtungen.push(RegisternotationsRichtung {
                teil: plan.teil.clone(),
                takt_ordinal: plan.beginn_takt_ordinal,
                system: plan.system.clone(),
                einsatz: plan.beginn_einsatz,
                art: RegisternotationsGrenzart::Start,
                darstellung: plan.darstellung,
                linie,
                origin: None,
            });
            for wiederholung in &plan.sichtwiederholungen {
                richtungen.push(RegisternotationsRichtung {
                    teil: plan.teil.clone(),
                    takt_ordinal: wiederholung.takt_ordinal,
                    system: plan.system.clone(),
                    einsatz: wiederholung.einsatz,
                    art: RegisternotationsGrenzart::Continue,
                    darstellung: plan.darstellung,
                    linie,
                    origin: None,
                });
            }
            richtungen.push(RegisternotationsRichtung {
                teil: plan.teil.clone(),
                takt_ordinal: plan.ende_takt_ordinal,
                system: plan.system.clone(),
                einsatz: plan.ende_einsatz,
                art: RegisternotationsGrenzart::Stop,
                darstellung: plan.darstellung,
                linie,
                origin: None,
            });
        }
        for links in 0..belegungen.len() {
            for rechts in links.saturating_add(1)..belegungen.len() {
                let (links_von, links_bis, links_linie) = belegungen[links];
                let (rechts_von, rechts_bis, rechts_linie) = belegungen[rechts];
                if links_linie == rechts_linie
                    && links_von < rechts_bis
                    && rechts_von < links_bis
                {
                    anyhow::bail!(
                        "RegisternotationsSichtmaterialisierung: Liniennummer kollidiert in MusicXML-Dokumentordnung"
                    );
                }
            }
        }
    }
    richtungen.sort();
    Ok(richtungen)
}

fn registernotations_dokumentintervall(
    xml: &str,
    dokument: &roxmltree::Document<'_>,
    plan: &crate::registernotation::RegisternotationsMaterialisierungsPlan,
) -> anyhow::Result<(usize, usize)> {
    let teil = dokument
        .root_element()
        .children()
        .find(|knoten| {
            knoten.is_element()
                && knoten.tag_name().name() == "part"
                && knoten.attribute("id") == Some(plan.teil.as_str())
        })
        .ok_or_else(|| {
            anyhow!(
                "RegisternotationsSichtmaterialisierung: Teil {} fuer Dokumentordnung fehlt",
                plan.teil
            )
        })?;
    let takte = teil
        .children()
        .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "measure")
        .collect::<Vec<_>>();
    let starttakt = takte
        .get(plan.beginn_takt_ordinal.saturating_sub(1))
        .copied()
        .ok_or_else(|| {
            anyhow!(
                "RegisternotationsSichtmaterialisierung: Starttaktordinal {} fehlt",
                plan.beginn_takt_ordinal
            )
        })?;
    let stoptakt = takte
        .get(plan.ende_takt_ordinal.saturating_sub(1))
        .copied()
        .ok_or_else(|| {
            anyhow!(
                "RegisternotationsSichtmaterialisierung: Stoptaktordinal {} fehlt",
                plan.ende_takt_ordinal
            )
        })?;

    let startbereich = starttakt.range();
    let stopbereich = stoptakt.range();
    let startanker = registernotations_tonanker(&xml[startbereich.clone()])?;
    let stopanker = if startbereich == stopbereich {
        startanker.clone()
    } else {
        registernotations_tonanker(&xml[stopbereich.clone()])?
    };

    let startkandidaten = startanker
        .iter()
        .filter(|anker| {
            anker.system == plan.system && anker.beginn == plan.beginn_einsatz
        })
        .collect::<Vec<_>>();
    if startkandidaten.is_empty() || !startkandidaten.iter().any(|anker| anker.sichtbar) {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: kein sichtbarer Dokument-Startanker fuer System {} bei {}",
            plan.system,
            plan.beginn_einsatz
        );
    }
    let stopkandidaten = stopanker
        .iter()
        .filter(|anker| anker.system == plan.system && anker.ende == plan.ende_einsatz)
        .collect::<Vec<_>>();
    if stopkandidaten.is_empty() || !stopkandidaten.iter().any(|anker| anker.sichtbar) {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: kein sichtbarer Dokument-Stopanker fuer System {} bei {}",
            plan.system,
            plan.ende_einsatz
        );
    }

    let startposition = startbereich.start
        + startkandidaten
            .iter()
            .map(|anker| anker.bereich.start)
            .min()
            .ok_or_else(|| {
                anyhow!("RegisternotationsSichtmaterialisierung: Dokument-Startanker fehlt")
            })?;
    let stopposition = stopbereich.start
        + stopkandidaten
            .iter()
            .map(|anker| anker.bereich.end)
            .max()
            .ok_or_else(|| {
                anyhow!("RegisternotationsSichtmaterialisierung: Dokument-Stopanker fehlt")
            })?;
    Ok((
        startposition.min(stopposition),
        startposition.max(stopposition),
    ))
}

#[derive(Debug, Clone)]
struct RegisternotationsTonanker {
    system: String,
    beginn: i64,
    ende: i64,
    bereich: std::ops::Range<usize>,
    sichtbar: bool,
}

fn registernotationsrichtungen_in_takt(
    takt: &str,
    richtungen: &[RegisternotationsRichtung],
) -> anyhow::Result<(String, crate::arrangement::representation_transport::RepresentationStep)> {
    let anker = registernotations_tonanker(takt)?;
    let mut einfuegungen = BTreeMap::<usize, Vec<&RegisternotationsRichtung>>::new();

    for richtung in richtungen {
        if richtung.einsatz < 0 {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: negativer Ereignisanker"
            );
        }
        let kandidaten = anker
            .iter()
            .filter(|anker| anker.system == richtung.system)
            .filter(|anker| match richtung.art {
                RegisternotationsGrenzart::Start | RegisternotationsGrenzart::Continue => {
                    anker.beginn == richtung.einsatz
                }
                RegisternotationsGrenzart::Stop => anker.ende == richtung.einsatz,
            })
            .collect::<Vec<_>>();
        if kandidaten.is_empty() || !kandidaten.iter().any(|anker| anker.sichtbar) {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: kein sichtbarer Tonanker fuer System {} bei {}",
                richtung.system,
                richtung.einsatz
            );
        }
        let position = match richtung.art {
            RegisternotationsGrenzart::Start | RegisternotationsGrenzart::Continue => kandidaten
                .iter()
                .map(|anker| anker.bereich.start)
                .min()
                .ok_or_else(|| anyhow!("RegisternotationsSichtmaterialisierung: Start-/Continue-Anker fehlt"))?,
            RegisternotationsGrenzart::Stop => kandidaten
                .iter()
                .map(|anker| anker.bereich.end)
                .max()
                .ok_or_else(|| anyhow!("RegisternotationsSichtmaterialisierung: Stopanker fehlt"))?,
        };
        einfuegungen.entry(position).or_default().push(richtung);
    }

    let zusaetzlich = richtungen.len().saturating_mul(128);
    let mut ersatz = String::with_capacity(takt.len().saturating_add(zusaetzlich));
    let mut operations = Vec::new();
    let mut letzter = 0usize;
    for (position, mut an_position) in einfuegungen {
        if position < letzter || position > takt.len() {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: ungueltige Dokumentanker-Reihenfolge"
            );
        }
        an_position.sort_by_key(|richtung| {
            (
                richtung.art,
                richtung.linie,
                richtung.system.clone(),
                richtung.darstellung,
            )
        });
        ersatz.push_str(&takt[letzter..position]);
        for richtung in an_position {
            let text = registernotationsrichtung_xml(richtung)?;
            ersatz.push_str(&text);
            operations.push((position..position, text));
        }
        letzter = position;
    }
    ersatz.push_str(&takt[letzter..]);
    let journal = crate::arrangement::representation_transport::RepresentationStep::recorded(takt, &ersatz, operations);
    Ok((ersatz, journal))
}

fn registernotations_tonanker(takt: &str) -> anyhow::Result<Vec<RegisternotationsTonanker>> {
    let dokument = roxmltree::Document::parse(takt).map_err(|fehler| {
        anyhow!(
            "RegisternotationsSichtmaterialisierung: Takt fuer Ereignisanker ungueltig: {fehler}"
        )
    })?;
    let wurzel = dokument.root_element();
    if wurzel.tag_name().name() != "measure" {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: Ereignisanker ist kein MusicXML-Takt"
        );
    }

    let mut gruppen = Vec::<RegisternotationsTonanker>::new();
    let mut aktuell = None::<RegisternotationsTonanker>;
    let mut cursor = 0_i64;
    let mut letzter_notenbeginn = None::<i64>;

    let flush = |aktuell: &mut Option<RegisternotationsTonanker>,
                 gruppen: &mut Vec<RegisternotationsTonanker>| {
        if let Some(gruppe) = aktuell.take() {
            gruppen.push(gruppe);
        }
    };

    for kind in wurzel.children().filter(|knoten| knoten.is_element()) {
        match kind.tag_name().name() {
            "note" => {
                let akkordton = kind
                    .children()
                    .any(|knoten| knoten.is_element() && knoten.tag_name().name() == "chord");
                let vorschlag = kind
                    .children()
                    .any(|knoten| knoten.is_element() && knoten.tag_name().name() == "grace");
                let hat_tonhoehe = kind
                    .children()
                    .any(|knoten| knoten.is_element() && knoten.tag_name().name() == "pitch");
                let system = kind
                    .children()
                    .find(|knoten| knoten.is_element() && knoten.tag_name().name() == "staff")
                    .and_then(|knoten| knoten.text())
                    .unwrap_or("1")
                    .trim()
                    .to_string();
                let sichtbar = hat_tonhoehe
                    && !kind
                        .attribute("print-object")
                        .is_some_and(|wert| wert.trim().eq_ignore_ascii_case("no"));
                let dauer = xml_kind_dauer(kind);
                let onset = if akkordton {
                    letzter_notenbeginn.unwrap_or(cursor)
                } else {
                    cursor
                };

                if !akkordton {
                    flush(&mut aktuell, &mut gruppen);
                    letzter_notenbeginn = Some(onset);
                    if hat_tonhoehe {
                        aktuell = Some(RegisternotationsTonanker {
                            system,
                            beginn: onset,
                            ende: cursor.saturating_add(dauer),
                            bereich: kind.range(),
                            sichtbar,
                        });
                    }
                    if !vorschlag {
                        cursor = cursor.saturating_add(dauer);
                    }
                } else if hat_tonhoehe {
                    if let Some(gruppe) = aktuell.as_mut()
                        && gruppe.system == system
                        && gruppe.beginn == onset
                    {
                        gruppe.bereich.end = kind.range().end;
                        gruppe.sichtbar |= sichtbar;
                        gruppe.ende = cursor;
                    } else {
                        flush(&mut aktuell, &mut gruppen);
                        aktuell = Some(RegisternotationsTonanker {
                            system,
                            beginn: onset,
                            ende: cursor,
                            bereich: kind.range(),
                            sichtbar,
                        });
                    }
                }
            }
            "backup" => {
                flush(&mut aktuell, &mut gruppen);
                cursor = cursor.saturating_sub(xml_kind_dauer(kind));
                letzter_notenbeginn = None;
            }
            "forward" => {
                flush(&mut aktuell, &mut gruppen);
                cursor = cursor.saturating_add(xml_kind_dauer(kind));
                letzter_notenbeginn = None;
            }
            _ => {
                flush(&mut aktuell, &mut gruppen);
            }
        }
    }
    flush(&mut aktuell, &mut gruppen);
    Ok(gruppen)
}

fn registernotationsrichtung_xml(
    richtung: &RegisternotationsRichtung,
) -> anyhow::Result<String> {
    let (art, groesse) = match (richtung.art, richtung.darstellung) {
        (
            RegisternotationsGrenzart::Start,
            crate::registernotation::RegisternotationsDarstellung::Achtva,
        ) => ("down", 8),
        (
            RegisternotationsGrenzart::Start,
            crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma,
        ) => ("down", 15),
        (
            RegisternotationsGrenzart::Start,
            crate::registernotation::RegisternotationsDarstellung::Achtvb,
        ) => ("up", 8),
        (
            RegisternotationsGrenzart::Start,
            crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb,
        ) => ("up", 15),
        (RegisternotationsGrenzart::Continue, darstellung) => {
            ("continue", darstellung.groesse())
        }
        (RegisternotationsGrenzart::Stop, darstellung) => ("stop", darstellung.groesse()),
        (_, crate::registernotation::RegisternotationsDarstellung::Keine) => {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: Darstellung Keine darf nicht materialisiert werden"
            );
        }
    };
    if richtung.linie == 0 || richtung.linie > 16 {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: ungueltige Ueberlappungsidentitaet"
        );
    }
    let placement = match richtung.darstellung {
        crate::registernotation::RegisternotationsDarstellung::Achtvb
        | crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb => "below",
        crate::registernotation::RegisternotationsDarstellung::Achtva
        | crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma => "above",
        crate::registernotation::RegisternotationsDarstellung::Keine => {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: Darstellung Keine besitzt keine Sichtseite"
            );
        }
    };
    let sichtcue = if richtung.art == RegisternotationsGrenzart::Continue {
        format!(
            "<direction-type><words font-style=\"italic\">{}</words></direction-type>",
            richtung.darstellung.bezeichnung()
        )
    } else {
        String::new()
    };
    let origin = richtung.origin.as_ref().map(|id|format!("<!--{id}-->")).unwrap_or_default();
    Ok(format!(
        "<direction placement=\"{}\">{}{}<direction-type><octave-shift type=\"{}\" size=\"{}\" number=\"{}\"/></direction-type><staff>{}</staff></direction>",
        placement, origin, sichtcue, art, groesse, richtung.linie, richtung.system
    ))
}

fn pruefe_registernotationsmaterialisierung(
    vorher: &str,
    nachher: &str,
    erwartet: &[RegisternotationsRichtung],
) -> anyhow::Result<()> {
    let dokument = roxmltree::Document::parse(nachher).map_err(|fehler| {
        anyhow!("RegisternotationsSichtmaterialisierung: materialisiertes XML ungueltig: {fehler}")
    })?;
    let mut beobachtet = Vec::<RegisternotationsRichtung>::new();
    let mut richtungsbereiche = Vec::<std::ops::Range<usize>>::new();

    for teil in dokument
        .root_element()
        .children()
        .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "part")
    {
        let teilkennung = teil
            .attribute("id")
            .ok_or_else(|| anyhow!("RegisternotationsSichtmaterialisierung: Teilkennung fehlt"))?
            .to_string();
        for (takt_index, takt) in teil
            .children()
            .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "measure")
            .enumerate()
        {
            let takt_ordinal = takt_index.saturating_add(1);
            let mut zeit = 0_i64;
            let mut letzter_notenbeginn = None::<i64>;
            for kind in takt.children().filter(|knoten| knoten.is_element()) {
                match kind.tag_name().name() {
                    "note" => {
                        let akkordton = kind
                            .children()
                            .any(|knoten| knoten.is_element() && knoten.tag_name().name() == "chord");
                        let onset = if akkordton {
                            letzter_notenbeginn.unwrap_or(zeit)
                        } else {
                            zeit
                        };
                        if !akkordton {
                            letzter_notenbeginn = Some(onset);
                            let ist_vorschlag = kind
                                .children()
                                .any(|knoten| knoten.is_element() && knoten.tag_name().name() == "grace");
                            if !ist_vorschlag {
                                zeit = zeit.saturating_add(xml_kind_dauer(kind));
                            }
                        }
                    }
                    "backup" => {
                        zeit = zeit.saturating_sub(xml_kind_dauer(kind));
                        letzter_notenbeginn = None;
                    }
                    "forward" => {
                        zeit = zeit.saturating_add(xml_kind_dauer(kind));
                        letzter_notenbeginn = None;
                    }
                    "direction" => {
                        letzter_notenbeginn = None;
                        let oktav = kind.descendants().find(|knoten| {
                            knoten.is_element() && knoten.tag_name().name() == "octave-shift"
                        });
                        let Some(oktav) = oktav else {
                            continue;
                        };
                        let system = kind
                            .children()
                            .find(|knoten| knoten.is_element() && knoten.tag_name().name() == "staff")
                            .and_then(|knoten| knoten.text())
                            .unwrap_or("1")
                            .trim()
                            .to_string();
                        if kind.children().any(|knoten| {
                            knoten.is_element() && knoten.tag_name().name() == "offset"
                        }) {
                            anyhow::bail!(
                                "RegisternotationsSichtmaterialisierung: Ereignisanker darf keinen Offset tragen"
                            );
                        }
                        if oktav.attribute("default-y").is_some() {
                            anyhow::bail!(
                                "RegisternotationsSichtmaterialisierung: Sichtzielbild darf kein default-y vorentscheiden"
                            );
                        }
                        let art_text = oktav.attribute("type").unwrap_or_default();
                        let groesse = oktav
                            .attribute("size")
                            .and_then(|text| text.parse::<i32>().ok())
                            .unwrap_or(8);
                        let linie = oktav
                            .attribute("number")
                            .and_then(|text| text.parse::<usize>().ok())
                            .filter(|nummer| (1..=16).contains(nummer))
                            .ok_or_else(|| {
                                anyhow!(
                                    "RegisternotationsSichtmaterialisierung: ungueltige Linienidentitaet"
                                )
                            })?;
                        let placement = kind.attribute("placement").unwrap_or("above");
                        let art = match art_text {
                            "stop" => RegisternotationsGrenzart::Stop,
                            "continue" => RegisternotationsGrenzart::Continue,
                            _ => RegisternotationsGrenzart::Start,
                        };
                        let darstellung = match (art, art_text, groesse, placement) {
                            (RegisternotationsGrenzart::Start, "down", 8, "above") => {
                                crate::registernotation::RegisternotationsDarstellung::Achtva
                            }
                            (RegisternotationsGrenzart::Start, "down", 15, "above") => {
                                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma
                            }
                            (RegisternotationsGrenzart::Start, "up", 8, "below") => {
                                crate::registernotation::RegisternotationsDarstellung::Achtvb
                            }
                            (RegisternotationsGrenzart::Start, "up", 15, "below") => {
                                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb
                            }
                            (RegisternotationsGrenzart::Continue, "continue", 8, "above")
                            | (RegisternotationsGrenzart::Stop, _, 8, "above") => {
                                crate::registernotation::RegisternotationsDarstellung::Achtva
                            }
                            (RegisternotationsGrenzart::Continue, "continue", 15, "above")
                            | (RegisternotationsGrenzart::Stop, _, 15, "above") => {
                                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma
                            }
                            (RegisternotationsGrenzart::Continue, "continue", 8, "below")
                            | (RegisternotationsGrenzart::Stop, _, 8, "below") => {
                                crate::registernotation::RegisternotationsDarstellung::Achtvb
                            }
                            (RegisternotationsGrenzart::Continue, "continue", 15, "below")
                            | (RegisternotationsGrenzart::Stop, _, 15, "below") => {
                                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb
                            }
                            _ => {
                                anyhow::bail!(
                                    "RegisternotationsSichtmaterialisierung: unbekannte Oktavrichtung"
                                );
                            }
                        };
                        let sichtcue_texte = kind
                            .descendants()
                            .filter(|knoten| {
                                knoten.is_element() && knoten.tag_name().name() == "words"
                            })
                            .filter_map(|knoten| knoten.text())
                            .map(str::trim)
                            .filter(|text| !text.is_empty())
                            .collect::<Vec<_>>();
                        if art == RegisternotationsGrenzart::Continue {
                            if sichtcue_texte.len() != 1
                                || sichtcue_texte.first().copied()
                                    != Some(darstellung.bezeichnung())
                            {
                                anyhow::bail!(
                                    "RegisternotationsSichtmaterialisierung: Continue besitzt keinen exakt passenden sichtbaren Ottava-Re-Cue"
                                );
                            }
                        } else if !sichtcue_texte.is_empty() {
                            anyhow::bail!(
                                "RegisternotationsSichtmaterialisierung: Start/Stop darf keinen zusaetzlichen Re-Cue-Text tragen"
                            );
                        }
                        beobachtet.push(RegisternotationsRichtung {
                            teil: teilkennung.clone(),
                            takt_ordinal,
                            system,
                            einsatz: zeit,
                            art,
                            darstellung,
                            linie,
                            origin: crate::registernotation::reader_policy::carried_origin(kind).map(str::to_owned),
                        });
                        richtungsbereiche.push(kind.range());
                    }
                    _ => {}
                }
            }
        }
    }
    drop(dokument);


    let mut erwartet = erwartet.to_vec();
    erwartet.sort();
    beobachtet.sort();
    if beobachtet != erwartet {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: materialisierte Oktavgrenzen weichen vom bewiesenen Sichtzielbild ab"
        );
    }

    richtungsbereiche.sort_by_key(|bereich| bereich.start);
    let mut ohne_oktavrichtungen = String::with_capacity(vorher.len());
    let mut letzter = 0usize;
    for bereich in richtungsbereiche {
        if bereich.start < letzter || bereich.end > nachher.len() {
            anyhow::bail!(
                "RegisternotationsSichtmaterialisierung: Oktavrichtungsbereiche ueberlappen oder sind ungueltig"
            );
        }
        ohne_oktavrichtungen.push_str(&nachher[letzter..bereich.start]);
        letzter = bereich.end;
    }
    ohne_oktavrichtungen.push_str(&nachher[letzter..]);
    if ohne_oktavrichtungen != vorher {
        anyhow::bail!(
            "RegisternotationsSichtmaterialisierung: ausserhalb der Oktavrichtungen wurde XML veraendert"
        );
    }
    Ok(())
}

fn xml_kind_dauer(kind: roxmltree::Node<'_, '_>) -> i64 {
    kind.children()
        .find(|knoten| knoten.is_element() && knoten.tag_name().name() == "duration")
        .and_then(|knoten| knoten.text())
        .and_then(|text| text.trim().parse::<i64>().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod registernotations_besitzumschaltung_tests {
    use super::*;


    fn richtung(
        einsatz: i64,
        art: RegisternotationsGrenzart,
        darstellung: crate::registernotation::RegisternotationsDarstellung,
    ) -> RegisternotationsRichtung {
        RegisternotationsRichtung {
            teil: "P1".to_string(),
            takt_ordinal: 1,
            system: "1".to_string(),
            einsatz,
            art,
            darstellung,
            linie: 1,
            origin: None,
        }
    }

    #[test]
    fn materialisierung_aendert_ausschliesslich_oktavrichtungen() {
        let takt = "<measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure>";
        let erwartet = vec![
            richtung(
                0,
                RegisternotationsGrenzart::Start,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                24,
                RegisternotationsGrenzart::Stop,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
        ];
        let (ersetzt, _) =
            registernotationsrichtungen_in_takt(takt, &erwartet).expect("materialisierung");
        let vorher = format!(
            "<score-partwise><part id=\"P1\">{takt}</part></score-partwise>"
        );
        let nachher = format!(
            "<score-partwise><part id=\"P1\">{ersetzt}</part></score-partwise>"
        );
        pruefe_registernotationsmaterialisierung(&vorher, &nachher, &erwartet)
            .expect("exakte planmaterialisierung");
        assert!(!ersetzt.contains("<offset>"));
        assert!(!ersetzt.contains("default-y="));
        assert!(ersetzt.find("<direction").unwrap() > ersetzt.find("</attributes>").unwrap());
    }

    #[test]
    fn gemeinsamer_grenzzeitpunkt_schliesst_vor_neuem_start() {
        let takt = "<measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure>";
        let richtungen = vec![
            richtung(
                24,
                RegisternotationsGrenzart::Start,
                crate::registernotation::RegisternotationsDarstellung::Achtvb,
            ),
            richtung(
                24,
                RegisternotationsGrenzart::Stop,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
        ];
        let (ersetzt, _) =
            registernotationsrichtungen_in_takt(takt, &richtungen).expect("materialisierung");
        let stop = ersetzt.find("type=\"stop\"").expect("stop");
        let start = ersetzt.find("type=\"up\"").expect("start");
        assert!(stop < start);
    }

    #[test]
    fn stille_bruecke_materialisiert_continue_ohne_scope_neustart() {
        let takt = "<measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><forward><duration>24</duration></forward><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure>";
        let erwartet = vec![
            richtung(
                0,
                RegisternotationsGrenzart::Start,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                48,
                RegisternotationsGrenzart::Continue,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                72,
                RegisternotationsGrenzart::Stop,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
        ];
        let (ersetzt, _) =
            registernotationsrichtungen_in_takt(takt, &erwartet).expect("materialisierung");
        assert_eq!(ersetzt.matches("type=\"down\"").count(), 1);
        assert_eq!(ersetzt.matches("type=\"continue\"").count(), 1);
        assert_eq!(ersetzt.matches("type=\"stop\"").count(), 1);
        assert_eq!(ersetzt.matches("<octave-shift").count(), 3);
        assert_eq!(ersetzt.matches("number=\"1\"/>").count(), 3);
        assert_eq!(
            ersetzt.matches("<words font-style=\"italic\">8va</words>").count(),
            1
        );
        let vorher = format!(
            "<score-partwise><part id=\"P1\">{takt}</part></score-partwise>"
        );
        let nachher = format!(
            "<score-partwise><part id=\"P1\">{ersetzt}</part></score-partwise>"
        );
        pruefe_registernotationsmaterialisierung(&vorher, &nachher, &erwartet)
            .expect("continue muss denselben Scope in gleicher Linienidentitaet bestaetigen");
    }

    #[test]
    fn continue_sichtcue_folgt_exakt_der_registerdarstellung() {
        let faelle = [
            (
                crate::registernotation::RegisternotationsDarstellung::Achtva,
                "8va",
            ),
            (
                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma,
                "15ma",
            ),
            (
                crate::registernotation::RegisternotationsDarstellung::Achtvb,
                "8vb",
            ),
            (
                crate::registernotation::RegisternotationsDarstellung::Fuenfzehnmb,
                "15mb",
            ),
        ];
        for (darstellung, text) in faelle {
            let xml = registernotationsrichtung_xml(&richtung(
                0,
                RegisternotationsGrenzart::Continue,
                darstellung,
            ))
            .expect("Continue-Re-Cue");
            assert!(xml.contains(&format!(
                "<words font-style=\"italic\">{text}</words>"
            )));
            assert!(xml.contains("type=\"continue\""));
        }
    }

    #[test]
    fn continue_ohne_sichtbaren_recue_wird_fail_closed_abgewiesen() {
        let takt = "<measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><forward><duration>24</duration></forward><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure>";
        let erwartet = vec![
            richtung(
                0,
                RegisternotationsGrenzart::Start,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                48,
                RegisternotationsGrenzart::Continue,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                72,
                RegisternotationsGrenzart::Stop,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
        ];
        let (ersetzt, _) = registernotationsrichtungen_in_takt(takt, &erwartet)
            .expect("Materialisierung mit sichtbarem Re-Cue");
        let ohne_recue = ersetzt.replace(
            "<direction-type><words font-style=\"italic\">8va</words></direction-type>",
            "",
        );
        let vorher = format!(
            "<score-partwise><part id=\"P1\">{takt}</part></score-partwise>"
        );
        let nachher = format!(
            "<score-partwise><part id=\"P1\">{ohne_recue}</part></score-partwise>"
        );
        let fehler = pruefe_registernotationsmaterialisierung(&vorher, &nachher, &erwartet)
            .expect_err("unsichtbarer Continue darf nicht als sichtbarer Re-Cue gelten");
        assert!(fehler
            .to_string()
            .contains("keinen exakt passenden sichtbaren Ottava-Re-Cue"));
    }

    #[test]
    fn sichtauftrag_bindet_continue_an_dieselbe_linienidentitaet() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><forward><duration>24</duration></forward><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let plaene = vec![crate::registernotation::RegisternotationsMaterialisierungsPlan {
            teil: "P1".to_string(),
            beginn_takt_ordinal: 1,
            ende_takt_ordinal: 1,
            system: "1".to_string(),
            beginn_einsatz: 0,
            ende_einsatz: 72,
            darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
            sichtwiederholungen: vec![
                crate::registernotation::RegisternotationsSichtwiederholung {
                    takt_ordinal: 1,
                    einsatz: 48,
                },
            ],
        }];
        let richtungen = registernotationsrichtungen_aus_sichtauftrag(xml, &plaene)
            .expect("Sichtauftrag mit Continue-Cue");
        assert_eq!(richtungen.len(), 3);
        assert_eq!(richtungen[0].art, RegisternotationsGrenzart::Start);
        assert_eq!(richtungen[1].art, RegisternotationsGrenzart::Continue);
        assert_eq!(richtungen[1].einsatz, 48);
        assert_eq!(richtungen[2].art, RegisternotationsGrenzart::Stop);
        assert_eq!(richtungen[0].linie, richtungen[1].linie);
        assert_eq!(richtungen[1].linie, richtungen[2].linie);
    }

    #[test]
    fn continue_am_taktanfang_bleibt_im_fortbestehenden_scope() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>24</divisions></attributes><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure><measure number=\"\
2\"><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let plaene = vec![crate::registernotation::RegisternotationsMaterialisierungsPlan {
            teil: "P1".to_string(),
            beginn_takt_ordinal: 1,
            ende_takt_ordinal: 2,
            system: "1".to_string(),
            beginn_einsatz: 0,
            ende_einsatz: 24,
            darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
            sichtwiederholungen: vec![
                crate::registernotation::RegisternotationsSichtwiederholung {
                    takt_ordinal: 2,
                    einsatz: 0,
                },
            ],
        }];
        let richtungen = registernotationsrichtungen_aus_sichtauftrag(xml, &plaene)
            .expect("taktuebergreifender Sichtauftrag mit Continue-Cue");
        let continue_richtung = richtungen
            .iter()
            .find(|richtung| richtung.art == RegisternotationsGrenzart::Continue)
            .expect("Continue-Cue");
        assert_eq!(continue_richtung.takt_ordinal, 2);
        assert_eq!(continue_richtung.einsatz, 0);
        assert_eq!(continue_richtung.linie, richtungen[0].linie);
        assert_eq!(continue_richtung.linie, richtungen[2].linie);
    }

    #[test]
    fn mittige_attributes_verschieben_den_registernotations_zeitnullpunkt_nicht() {
        let takt = "<measure number=\"\
30\"><note><pitch><step>C</step><octave>5</octave></pitch><duration>192</duration><voice>1</voice><staff>1</staff></note><attributes><clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>D</step><octave>5</octave></pitch><duration>192</duration><voice>1</voice><staff>1</staff></note></measure>";
        let erwartet = vec![
            richtung(
                0,
                RegisternotationsGrenzart::Start,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
            richtung(
                384,
                RegisternotationsGrenzart::Stop,
                crate::registernotation::RegisternotationsDarstellung::Achtva,
            ),
        ];
        let (ersetzt, _) =
            registernotationsrichtungen_in_takt(takt, &erwartet).expect("materialisierung");
        let erste_note = ersetzt.find("<note>").expect("erste note");
        let oktavrichtung = ersetzt.find("<direction placement=\"above\">").expect("oktavrichtung");
        let mittige_attributes = ersetzt.find("<attributes>").expect("mittige attributes");
        assert!(oktavrichtung < erste_note);
        assert!(erste_note < mittige_attributes);

        let vorher = format!(
            "<score-partwise><part id=\"P1\">{takt}</part></score-partwise>"
        );
        let nachher = format!(
            "<score-partwise><part id=\"P1\">{ersetzt}</part></score-partwise>"
        );
        pruefe_registernotationsmaterialisierung(&vorher, &nachher, &erwartet)
            .expect("mittige attributes duerfen den Richtungszeitnullpunkt nicht verschieben");
    }


    #[test]
    fn liniennummer_entsteht_aus_ueberlappung_und_nicht_aus_systemnummer() {
        let plaene = vec![
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 2,
                system: "1".to_string(),
                beginn_einsatz: 0,
                ende_einsatz: 96,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
                sichtwiederholungen: Vec::new(),
            },
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 1,
                system: "2".to_string(),
                beginn_einsatz: 48,
                ende_einsatz: 144,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtvb,
                sichtwiederholungen: Vec::new(),
            },
        ];
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><note><pitch><step>C</step><octave>6</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note><backup><duration>96</duration></backup><forward><duration>48</duration></forward><note><pitch><step>C</step><octave>3</octave></pitch><duration>96</duration><voice>2</voice><staff>2</staff></note></measure><measure number=\"\
2\"><note><pitch><step>D</step><octave>6</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let richtungen =
            registernotationsrichtungen_aus_sichtauftrag(xml, &plaene).expect("linienplanung");
        let starts = richtungen
            .iter()
            .filter(|richtung| richtung.art == RegisternotationsGrenzart::Start)
            .collect::<Vec<_>>();
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0].linie, 1);
        assert_eq!(starts[1].linie, 2);
    }

    #[test]
    fn nicht_ueberlappende_sichtlinien_duerfen_nummer_wiederverwenden() {
        let plaene = vec![
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 1,
                system: "1".to_string(),
                beginn_einsatz: 0,
                ende_einsatz: 24,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
                sichtwiederholungen: Vec::new(),
            },
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 1,
                system: "2".to_string(),
                beginn_einsatz: 24,
                ende_einsatz: 48,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtvb,
                sichtwiederholungen: Vec::new(),
            },
        ];
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><backup><duration>24</duration></backup><forward><duration>24</duration></forward><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>";
        let richtungen =
            registernotationsrichtungen_aus_sichtauftrag(xml, &plaene).expect("linienplanung");
        let starts = richtungen
            .iter()
            .filter(|richtung| richtung.art == RegisternotationsGrenzart::Start)
            .collect::<Vec<_>>();
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0].linie, 1);
        assert_eq!(starts[1].linie, 1);
    }

    #[test]
    fn dokumentordnung_schuetzt_zeitlich_getrennte_polyphone_linien_vor_identitaetskollision() {
        let plaene = vec![
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 3,
                system: "1".to_string(),
                beginn_einsatz: 120,
                ende_einsatz: 240,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
                sichtwiederholungen: Vec::new(),
            },
            crate::registernotation::RegisternotationsMaterialisierungsPlan {
                teil: "P1".to_string(),
                beginn_takt_ordinal: 1,
                ende_takt_ordinal: 1,
                system: "2".to_string(),
                beginn_einsatz: 72,
                ende_einsatz: 96,
                darstellung: crate::registernotation::RegisternotationsDarstellung::Achtva,
                sichtwiederholungen: Vec::new(),
            },
        ];
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><forward><duration>120</duration></forward><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><backup><duration>144</duration></backup><forward><duration>72</duration></forward><note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note></measure><measure number=\"\
2\"><note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note></measure><measure number=\"\
3\"><forward><duration>216</duration></forward><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let richtungen =
            registernotationsrichtungen_aus_sichtauftrag(xml, &plaene).expect("linienplanung");
        let starts = richtungen
            .iter()
            .filter(|richtung| richtung.art == RegisternotationsGrenzart::Start)
            .collect::<Vec<_>>();
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0].linie, 1);
        assert_eq!(starts[1].linie, 2);
    }

    #[test]
    fn untere_oktavlinie_wird_ohne_offset_und_default_y_unten_projiziert() {
        let richtung = RegisternotationsRichtung {
            teil: "P1".to_string(),
            takt_ordinal: 1,
            system: "2".to_string(),
            einsatz: 0,
            art: RegisternotationsGrenzart::Start,
            darstellung: crate::registernotation::RegisternotationsDarstellung::Achtvb,
            linie: 1,
            origin: None,
        };
        let xml = registernotationsrichtung_xml(&richtung).expect("richtung");
        assert!(xml.contains("placement=\"below\""));
        assert!(xml.contains("number=\"1\""));
        assert!(!xml.contains("<offset>"));
        assert!(!xml.contains("default-y="));
    }

    #[test]
    fn unbekannte_oktavdarstellung_wird_nicht_materialisiert() {
        let richtung = richtung(
            0,
            RegisternotationsGrenzart::Start,
            crate::registernotation::RegisternotationsDarstellung::Keine,
        );
        assert!(registernotationsrichtung_xml(&richtung).is_err());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LowerStaffClefState {
    Bass,
    Treble,
}
fn apply_lower_staff_clef_readability_policy(xml: &mut String) -> LowerStaffClefPolish {
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};
    let input = xml.clone();
    let mut steps = Vec::new();
    let mut state = LowerStaffClefState::Bass;
    let mut total_switches = 0_usize;
    let mut total_high_notes = 0_usize;
    let mut measures = Vec::new();
    let mut block_receipts = Vec::new();
    let mut scan = 0_usize;
    while let Some((start, end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let original = xml[start..end].to_string();
        let LowerStaffClefMeasure { xml: updated, switches, high_notes, state: new_state, insertions } =
            lower_staff_clef_measure(&original, state);
        state = new_state;
        if switches > 0 || updated != original {
            let before = xml.clone();
            let (replaced, receipt) =
                crate::structural_write_authority::replace_bound_clef_measure_authorized(
                    xml.as_str(),
                    start,
                    end,
                    updated.clone(),
                );
            *xml = replaced.expect("clef measure capability must match the selected source range");
            steps.push(RepresentationStep::recorded(&before, xml,
                insertions.into_iter().map(|(range, text)|
                    (start + range.start..start + range.end, text)).collect()));
            block_receipts.extend(receipt);
            measures.push(measure_number);
            total_switches += switches;
            total_high_notes += high_notes;
            scan = start + updated.len();
        } else {
            total_high_notes += high_notes;
            scan = end;
        }
    }
    if steps.is_empty() {
        steps.push(RepresentationStep::recorded(&input, xml, Vec::new()));
    }
    let rhythmic_transport = RhythmicRepresentationTransport::clef_readability(steps, &block_receipts);
    if total_switches == 0 {
        let existing_switches = count_existing_lower_staff_clef_switches(xml);
        return LowerStaffClefPolish {
            done_fragment: if existing_switches > 0 {
                format!(
                    "Basssystem-Schlüsselwechsel final vorhanden: {existing_switches} Schlüsselangabe(n)"
                )
            } else {
                "Basssystem-Schlüsselwechsel nicht nötig".to_string()
            },
            block_receipts: master_state_identity::RecordedClefReceipts::new(block_receipts, &rhythmic_transport),
            rhythmic_transport,
        };
    }
    let _ = (measures, total_high_notes);
    crate::audit_only::destructive_edit_attribution::observe_selected_state(xml);
    LowerStaffClefPolish {
        done_fragment: format!(
            "unteres System: {total_switches} temporäre Violinschlüssel-Wechsel ab A4 gesetzt"
        ),
        block_receipts: master_state_identity::RecordedClefReceipts::new(block_receipts, &rhythmic_transport),
        rhythmic_transport,
    }
}
/// Pure preparation helper; the existing BLOCK owner commits the result.
struct LowerStaffClefMeasure {
    xml: String,
    switches: usize,
    high_notes: usize,
    state: LowerStaffClefState,
    insertions: Vec<(std::ops::Range<usize>, String)>,
}
fn lower_staff_clef_measure(measure: &str, mut state: LowerStaffClefState) -> LowerStaffClefMeasure {
    let mut insertions = Vec::new();
    let mut out = String::with_capacity(measure.len() + 256);
    let mut last = 0_usize;
    let mut scan = 0_usize;
    let mut switches = 0_usize;
    let mut high_notes = 0_usize;
    while scan < measure.len() {
        let next_note = measure[scan..].find("<note").map(|rel| scan + rel);
        let next_attr = measure[scan..].find("<attributes").map(|rel| scan + rel);
        match (next_attr, next_note) {
            (Some(attr_start), Some(note_start)) if attr_start < note_start => {
                let Some(close_rel) = measure[attr_start..].find("</attributes>") else { break; };
                let attr_end = attr_start + close_rel + "</attributes>".len();
                out.push_str(&measure[last..attr_start]);
                let attr = &measure[attr_start..attr_end];
                state = lower_staff_clef_state_after_attributes(attr, state);
                out.push_str(attr);
                last = attr_end;
                scan = attr_end;
            }
            (Some(attr_start), None) => {
                let Some(close_rel) = measure[attr_start..].find("</attributes>") else { break; };
                let attr_end = attr_start + close_rel + "</attributes>".len();
                out.push_str(&measure[last..attr_start]);
                let attr = &measure[attr_start..attr_end];
                state = lower_staff_clef_state_after_attributes(attr, state);
                out.push_str(attr);
                last = attr_end;
                scan = attr_end;
            }
            (_, Some(note_start)) => {
                let Some(close_rel) = measure[note_start..].find("</note>") else { break; };
                let note_end = note_start + close_rel + "</note>".len();
                let note = &measure[note_start..note_end];
                out.push_str(&measure[last..note_start]);
                let note_staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
                if note_staff.trim() == "2" && !note.contains("<rest") && note.contains("<pitch>")
                    && let Some(midi) = midi_from_note_fragment(note) {
                        let wanted = if midi >= LOWER_STAFF_TREBLE_CLEF_FROM_MIDI {
                            high_notes += 1;
                            LowerStaffClefState::Treble
                        } else {
                            LowerStaffClefState::Bass
                        };
                        if wanted != state {
                            let attributes = lower_staff_clef_attributes(wanted);
                            insertions.push((note_start..note_start, attributes.to_owned()));
                            out.push_str(attributes);
                            state = wanted;
                            switches += 1;
                        }
                    }
                out.push_str(note);
                last = note_end;
                scan = note_end;
            }
            (None, None) => break,
        }
    }
    out.push_str(&measure[last..]);
    LowerStaffClefMeasure { xml: out, switches, high_notes, state, insertions }
}
fn lower_staff_clef_attributes(state: LowerStaffClefState) -> &'static str {
    match state {
        LowerStaffClefState::Treble => "\n      <attributes>\n        <clef number=\"2\"><sign>G</sign><line>2</line></clef>\n      </attributes>",
        LowerStaffClefState::Bass => "\n      <attributes>\n        <clef number=\"2\"><sign>F</sign><line>4</line></clef>\n      </attributes>",
    }
}
#[derive(Debug, Clone, Copy)]
struct OctaveExportStats {
    checked_shifts: usize,
    above_shifts: usize,
    below_shifts: usize,
    continue_shifts: usize,
    visible_continue_recues: usize,
    layout_restart_continues: usize,
}
#[derive(Debug, Default)]
struct PartialBeatPostRestBeamPatchStats {
    representation_edits: Vec<(std::ops::Range<usize>, String)>,
    groups: usize,
    notes: usize,
    beam_tags_added: usize,
    /// PRESERVING receipt (E053) of the whole-document owner call, minted only
    /// if the conserved musical content of input and output is identical.
    preserving_receipt: Option<crate::master_state_identity::PreservingTransformationReceipt>,
}

/// Sealed mint capability of `enforce_partial_beat_post_rest_beams_in_xml`
/// (PRESERVING E053).
pub(crate) struct PartialBeatBeamPreservingMint {
    _sealed: (),
}
fn enforce_partial_beat_post_rest_beams_in_xml(xml: &mut String) -> PartialBeatPostRestBeamPatchStats {
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        return PartialBeatPostRestBeamPatchStats::default();
    }
    let mut out = String::with_capacity(xml.len() + 1024);
    let mut last = 0_usize;
    let mut inherited_divisions = 1_i64;
    let mut total = PartialBeatPostRestBeamPatchStats::default();
    for span in spans {
        out.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let (patched, stats) = patch_partial_beat_post_rest_beams_in_measure(measure_text, inherited_divisions);
        total.groups += stats.groups;
        total.notes += stats.notes;
        total.beam_tags_added += stats.beam_tags_added;
        total.representation_edits.extend(stats.representation_edits.into_iter().map(|(range, text)|
            (range.start + span.start..range.end + span.start, text)));
        out.push_str(&patched);
        last = span.end;
    }
    out.push_str(&xml[last..]);
    total.preserving_receipt =
        crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
            PartialBeatBeamPreservingMint { _sealed: () },
            xml,
            &out,
        );
    let journal = crate::arrangement::representation_transport::RepresentationStep::recorded(
        xml, &out, std::mem::take(&mut total.representation_edits));
    let transport = crate::arrangement::representation_transport::RhythmicRepresentationTransport::partial_beam(
        &PartialBeatBeamPreservingMint { _sealed: () }, journal);
    total.preserving_receipt = total.preserving_receipt.map(|receipt|
        receipt.with_recorded_representation(transport));
    *xml = out;
    total
}
#[derive(Debug, Clone, Default)]
struct BeatScopedBeamPatchStats {
    actual_note_map: ActualBeamNoteMap,
    actual_beam_emissions: Vec<ActualBeamEmission>,
    groups: usize,
    notes: usize,
    affected_measures: usize,
}

#[derive(Debug, Clone, Default)]
struct BeatScopedRestPatchStats {
    actual_note_map: ActualBeamNoteMap,
    rest_notes_inserted: usize,
    rests_rewritten: usize,
    rest_only_aux_rests_hidden: usize,
    affected_measures: usize,
}

fn enforce_beat_scoped_beaming_and_rest_completion_in_xml(xml: &mut String) -> crate::master_state_identity::CapturedBlockReceiptInput {
    let lineage_pre_xml = xml.clone();
    let beams = enforce_beat_scoped_beaming_in_xml(xml);
    let rests = enforce_beat_scoped_rest_completion_in_xml(xml);
    let receipt = BlockTransformationReceipt::mint_block_receipt(
        BeatScopedBeamingBlockMint { _sealed: () }, &lineage_pre_xml, xml, None,
    );
    match receipt {
        Some(receipt) => {
            let capture = ActualCanonicalBeamCapture::at_owner(&receipt, &lineage_pre_xml, xml,
                &beams.actual_note_map, &rests.actual_note_map, &beams.actual_beam_emissions);
            crate::master_state_identity::CapturedBlockReceiptInput::observed_beam(receipt,capture)
        },
        None => None.into(),
    }
}

/// Sealed mint capability of
/// `enforce_beat_scoped_beaming_and_rest_completion_in_xml` (BLOCK #53).
pub(crate) struct BeatScopedBeamingBlockMint {
    _sealed: (),
}

fn enforce_beat_scoped_beaming_in_xml(xml: &mut String) -> BeatScopedBeamPatchStats {
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        return BeatScopedBeamPatchStats { actual_note_map: beat_scoped_unchanged_note_map(xml, 0, 0), ..Default::default() };
    }
    let mut out = String::with_capacity(xml.len() + 2048);
    let mut last = 0_usize;
    let mut inherited_divisions = 1_i64;
    let mut total = BeatScopedBeamPatchStats::default();
    for span in spans {
        total.actual_note_map.0.extend(beat_scoped_unchanged_note_map(&xml[last..span.start], last, out.len()).0);
        out.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let (patched, mut stats) = patch_beat_scoped_beaming_in_measure(
            measure_text,
            inherited_divisions,
        );
        if stats.groups > 0 {
            total.affected_measures += 1;
        }
        total.groups += stats.groups;
        total.notes += stats.notes;
        stats.actual_note_map.lift(span.start, out.len());
        total.actual_note_map.0.extend(stats.actual_note_map.0);
        for emission in &mut stats.actual_beam_emissions { emission.lift(span.start); }
        total.actual_beam_emissions.extend(stats.actual_beam_emissions);
        out.push_str(&patched);
        last = span.end;
    }
    total.actual_note_map.0.extend(beat_scoped_unchanged_note_map(&xml[last..], last, out.len()).0);
    out.push_str(&xml[last..]);
    *xml = out;
    total
}

/// Emitter-side identity carry of complete notes outside the patched spans.
fn beat_scoped_unchanged_note_map(text: &str, source: usize, output: usize) -> ActualBeamNoteMap {
    let mut map = ActualBeamNoteMap::unchanged(&partial_beat_raw_notes(text));
    map.lift(source, output);
    map
}

fn patch_beat_scoped_beaming_in_measure(
    measure_text: &str,
    divisions: i64,
) -> (String, BeatScopedBeamPatchStats) {
    let raw_notes = partial_beat_raw_notes(measure_text);
    if raw_notes.is_empty() {
        return (measure_text.to_string(), BeatScopedBeamPatchStats::default());
    }
    let note_infos = partial_beat_note_infos(measure_text, &raw_notes);
    let beat_unit = divisions.max(1);
    let mut states_by_note: BTreeMap<usize, Vec<(usize, &'static str)>> = BTreeMap::new();
    let mut explicit_negative_notes = BTreeSet::new();
    let mut stats = BeatScopedBeamPatchStats { actual_note_map: ActualBeamNoteMap::unchanged(&raw_notes), ..Default::default() };
    add_beat_scoped_beam_groups(
        &note_infos,
        beat_unit,
        &mut states_by_note,
        &mut explicit_negative_notes,
        &mut stats,
    );
    if states_by_note.is_empty() && explicit_negative_notes.is_empty() {
        return (measure_text.to_string(), stats);
    }
    stats.actual_note_map.0.clear();
    let mut rebuilt = String::with_capacity(measure_text.len() + stats.notes * 40);
    let mut last = 0_usize;
    for note in &raw_notes {
        rebuilt.push_str(&measure_text[last..note.start]);
        let emitted_start = rebuilt.len();
        if let Some(states) = states_by_note.get(&note.start) {
            let cleaned = partial_beat_strip_beam_tags(&note.text);
            rebuilt.push_str(&partial_beat_insert_beam_tags(&cleaned, states));
        } else if explicit_negative_notes.contains(&note.start) {
            rebuilt.push_str(&partial_beat_strip_beam_tags(&note.text));
        } else {
            rebuilt.push_str(&note.text);
        }
        stats.actual_note_map.0.push((note.start..note.end, emitted_start..rebuilt.len()));
        last = note.end;
    }
    rebuilt.push_str(&measure_text[last..]);
    (rebuilt, stats)
}

#[cfg(test)]
mod beat_scoped_beam_decision_scope_materialization_tests {
    use super::patch_beat_scoped_beaming_in_measure;

    fn note(step: &str, duration: i64, note_type: &str, beams: &str) -> String {
        format!(
            "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff>{beams}</note>"
        )
    }

    fn rest(duration: i64, note_type: &str) -> String {
        format!(
            "<note><rest/><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff></note>"
        )
    }

    #[test]
    fn accepted_subrun_clears_stale_prefix_inside_the_same_decided_beat_group() {
        let stale_prefix = note(
            "B",
            24,
            "16th",
            r#"<beam number="1">begin</beam><beam number="2">begin</beam>"#,
        );
        let old_middle = note(
            "F",
            24,
            "16th",
            r#"<beam number="1">continue</beam><beam number="2">continue</beam>"#,
        );
        let old_end = note(
            "G",
            24,
            "16th",
            r#"<beam number="1">end</beam><beam number="2">end</beam>"#,
        );
        let measure = format!(
            "<measure number=\"\
17\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward>{stale_prefix}{}{old_middle}{old_end}</measure>",
            rest(24, "16th"),
        );

        let (after, stats) = patch_beat_scoped_beaming_in_measure(&measure, 96);
        assert_eq!(stats.groups, 1);

        let prefix_start = after.find("<step>B</step>").expect("prefix");
        let prefix_end = after[prefix_start..]
            .find("</note>")
            .map(|relative| prefix_start + relative + "</note>".len())
            .expect("prefix end");
        let prefix_note = &after[prefix_start..prefix_end];
        assert!(!prefix_note.contains("<beam"));

        let middle_start = after.find("<step>F</step>").expect("middle");
        let middle_end = after[middle_start..]
            .find("</note>")
            .map(|relative| middle_start + relative + "</note>".len())
            .expect("middle end");
        let middle_note = &after[middle_start..middle_end];
        assert!(middle_note.contains(r#"<beam number="1">begin</beam>"#));
        assert!(middle_note.contains(r#"<beam number="2">begin</beam>"#));

        let end_start = after.find("<step>G</step>").expect("end");
        let end_end = after[end_start..]
            .find("</note>")
            .map(|relative| end_start + relative + "</note>".len())
            .expect("end note end");
        let end_note = &after[end_start..end_end];
        assert!(end_note.contains(r#"<beam number="1">end</beam>"#));
        assert!(end_note.contains(r#"<beam number="2">end</beam>"#));
    }

    #[test]
    fn beat_without_a_positive_run_keeps_foreign_beam_bytes_untouched() {
        let foreign = note(
            "E",
            24,
            "16th",
            r#"<beam number="1">forward hook</beam><beam number="2">forward hook</beam>"#,
        );
        let measure = format!(
            "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes>{foreign}<forward><duration>72</duration></forward></measure>"
        );
        let (after, stats) = patch_beat_scoped_beaming_in_measure(&measure, 96);
        assert_eq!(stats.groups, 0);
        assert_eq!(after, measure);
    }
}

fn enforce_beat_scoped_rest_completion_in_xml(xml: &mut String) -> BeatScopedRestPatchStats {
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        return BeatScopedRestPatchStats { actual_note_map: beat_scoped_unchanged_note_map(xml, 0, 0), ..Default::default() };
    }
    let mut out = String::with_capacity(xml.len() + 2048);
    let mut last = 0_usize;
    let mut inherited_divisions = 1_i64;
    let mut total = BeatScopedRestPatchStats::default();
    for span in spans {
        total.actual_note_map.0.extend(beat_scoped_unchanged_note_map(&xml[last..span.start], last, out.len()).0);
        out.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let (patched, mut stats) = patch_beat_scoped_rest_completion_in_measure(
            measure_text,
            inherited_divisions,
        );
        if stats.rest_notes_inserted > 0
            || stats.rests_rewritten > 0
            || stats.rest_only_aux_rests_hidden > 0
        {
            total.affected_measures += 1;
        }
        total.rest_notes_inserted += stats.rest_notes_inserted;
        total.rests_rewritten += stats.rests_rewritten;
        total.rest_only_aux_rests_hidden += stats.rest_only_aux_rests_hidden;
        stats.actual_note_map.lift(span.start, out.len());
        total.actual_note_map.0.extend(stats.actual_note_map.0);
        out.push_str(&patched);
        last = span.end;
    }
    total.actual_note_map.0.extend(beat_scoped_unchanged_note_map(&xml[last..], last, out.len()).0);
    out.push_str(&xml[last..]);
    *xml = out;
    total
}

fn patch_beat_scoped_rest_completion_in_measure(
    measure_text: &str,
    divisions: i64,
) -> (String, BeatScopedRestPatchStats) {
    let raw_notes = partial_beat_raw_notes(measure_text);
    if raw_notes.is_empty() {
        return (measure_text.to_string(), BeatScopedRestPatchStats { actual_note_map:ActualBeamNoteMap::unchanged(&raw_notes), ..Default::default() });
    }
    let note_infos = partial_beat_note_infos(measure_text, &raw_notes);
    let beat_unit = divisions.max(1);
    let measure_duration = beat_unit.saturating_mul(4);
    let mut stats = BeatScopedRestPatchStats { actual_note_map: ActualBeamNoteMap::unchanged(&raw_notes), ..Default::default() };
    let mut replacements: BTreeMap<usize, String> = BTreeMap::new();
    let mut inserts_before: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut inserted_rest_staffs: BTreeSet<String> = BTreeSet::new();
    let mut retained_replacements = BTreeSet::new();

    let (rest_completion_plans, cross_staff_suppressed_completion_staffs) =
        plan_visible_voice_rest_completion_gaps(&note_infos, measure_duration);
    add_visible_voice_rest_completion(
        &rest_completion_plans,
        measure_duration,
        beat_unit,
        &mut inserts_before,
        &mut inserted_rest_staffs,
        &mut stats,
    );

    for info in &note_infos {
        if !info.is_rest || info.is_hidden || info.is_chord || info.duration <= 0 {
            continue;
        }
        let Some(durations) = plan_exact_metric_visible_rest_durations(
            info.start,
            info.duration,
            measure_duration,
            beat_unit,
        ) else {
            continue;
        };
        if durations.len() == 1 && durations[0] == info.duration {
            continue;
        }
        replacements.insert(
            info.index,
            durations
                .into_iter()
                .map(|duration| {
                    beat_scoped_visible_rest_note(duration, &info.staff, &info.voice)
                })
                .collect::<Vec<_>>()
                .join(""),
        );
        stats.rests_rewritten += 1;
    }

    let visible_pitched_staffs = note_infos
        .iter()
        .filter(|info| !info.is_hidden && !info.is_rest && !info.is_chord)
        .map(|info| info.staff.clone())
        .collect::<BTreeSet<_>>();
    let visible_pitched_by_voice = note_infos
        .iter()
        .filter(|info| !info.is_hidden && !info.is_rest && !info.is_chord)
        .map(|info| (info.staff.clone(), info.voice.clone()))
        .collect::<BTreeSet<_>>();
    for info in &note_infos {
        if !info.is_rest || info.is_hidden || info.is_chord || info.duration <= 0 {
            continue;
        }
        if (inserted_rest_staffs.contains(&info.staff)
            || cross_staff_suppressed_completion_staffs.contains(&info.staff))
            && visible_pitched_staffs.contains(&info.staff)
            && !visible_pitched_by_voice.contains(&(info.staff.clone(), info.voice.clone()))
            && !replacements.contains_key(&info.index)
        {
            replacements.insert(
                info.index,
                beat_scoped_mark_note_hidden(&raw_note_text(&raw_notes, info.index)).0,
            );
            // This original visibility operation retains the same complete
            // physical rest. It is not a metric split or a new occurrence.
            retained_replacements.insert(info.index);
            stats.rest_only_aux_rests_hidden += 1;
        }
    }
    if replacements.is_empty() && inserts_before.is_empty() {
        return (measure_text.to_string(), stats);
    }

    let mut assembled_map=ActualBeamNoteMap::default();
    let mut rebuilt = String::with_capacity(
        measure_text.len() + stats.rest_notes_inserted * 160,
    );
    let mut last = 0_usize;
    for note in &raw_notes {
        rebuilt.push_str(&measure_text[last..note.start]);
        if let Some(inserts) = inserts_before.get(&note.start) {
            for insert in inserts {
                rebuilt.push_str(insert);
            }
        }
        let emitted_start=rebuilt.len();
        if let Some(replacement) = replacements.get(&note.start) {
            rebuilt.push_str(replacement);
            if retained_replacements.contains(&note.start) {
                assembled_map.0.push((note.start..note.end, emitted_start..rebuilt.len()));
            }
        } else {
            rebuilt.push_str(&note.text);
            assembled_map.0.push((note.start..note.end,emitted_start..rebuilt.len()));
        }
        last = note.end;
    }
    rebuilt.push_str(&measure_text[last..]);

    // Rest-Completion kann die reale Cursorposition spaeterer Pausen verschieben.
    // Diese zweite Lesung besitzt keinerlei Gap-, Beam- oder Silence-Entscheidungsrecht:
    // sie normalisiert ausschliesslich bereits materialisierte sichtbare Restdauern
    // ueber denselben zentralen MetricRestNotation-Owner.
    let (normalized, rewritten_after_materialization, normalized_map) =
        normalize_metric_visible_rests_with_actual_carry(
            &rebuilt,
            measure_duration,
            beat_unit,
        );

    // Rest completion owns visible silence only. It must not move pitched events
    // or turn a metrically green MusicXML cursor into an overflowing lane. The
    // checks are read-only postconditions on this existing owner; failure keeps
    // the pre-owner measure unchanged instead of delegating to a later repair.
    let pitched_timeline_stable = beat_scoped_rest_completion_pitched_timeline(measure_text)
        == beat_scoped_rest_completion_pitched_timeline(&normalized);
    let before_cursor =
        beat_scoped_rest_completion_cursor_state(measure_text, measure_duration);
    let after_cursor =
        beat_scoped_rest_completion_cursor_state(&normalized, measure_duration);
    if !pitched_timeline_stable
        || (before_cursor.is_green(measure_duration)
            && !after_cursor.is_green(measure_duration))
    {
        return (measure_text.to_string(), BeatScopedRestPatchStats { actual_note_map:ActualBeamNoteMap::unchanged(&raw_notes), ..Default::default() });
    }

    stats.actual_note_map=assembled_map.then(&normalized_map);
    stats.rests_rewritten += rewritten_after_materialization;
    (normalized, stats)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BeatScopedRestCompletionCursorState {
    cursor_min: i64,
    cursor_max: i64,
    cursor_final: i64,
    partial_backup_count: usize,
    overflow_events: usize,
    post_expected_events: usize,
}

impl BeatScopedRestCompletionCursorState {
    fn is_green(self, expected_duration: i64) -> bool {
        self.cursor_min >= 0
            && self.cursor_max <= expected_duration
            && self.cursor_final == expected_duration
            && self.partial_backup_count == 0
            && self.overflow_events == 0
            && self.post_expected_events == 0
    }
}

type BeatScopedRestCompletionPitchedKey = (
    i64,
    i64,
    String,
    String,
    bool,
    bool,
    String,
    String,
    String,
);

fn beat_scoped_rest_completion_pitched_timeline(
    measure_text: &str,
) -> Vec<BeatScopedRestCompletionPitchedKey> {
    let raw_notes = partial_beat_raw_notes(measure_text);
    let note_infos = partial_beat_note_infos(measure_text, &raw_notes);
    let mut pitched = note_infos
        .iter()
        .zip(raw_notes.iter())
        .filter_map(|(info, raw)| {
            if info.is_rest || !raw.text.contains("<pitch") {
                return None;
            }
            Some((
                info.start,
                info.duration,
                info.staff.clone(),
                info.voice.clone(),
                info.is_chord,
                info.is_hidden,
                partial_beat_first_text(&raw.text, "step").unwrap_or_default(),
                partial_beat_first_text(&raw.text, "alter").unwrap_or_default(),
                partial_beat_first_text(&raw.text, "octave").unwrap_or_default(),
            ))
        })
        .collect::<Vec<_>>();
    pitched.sort();
    pitched
}

fn beat_scoped_rest_completion_cursor_state(
    measure_text: &str,
    expected_duration: i64,
) -> BeatScopedRestCompletionCursorState {
    let mut state = BeatScopedRestCompletionCursorState::default();
    let mut cursor = 0_i64;
    let mut scan = 0_usize;
    while scan < measure_text.len() {
        let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
        let Some((pos, kind)) = [
            next_note.map(|p| (p, "note")),
            next_backup.map(|p| (p, "backup")),
            next_forward.map(|p| (p, "forward")),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(p, _)| *p)
        else {
            break;
        };
        match kind {
            "note" => {
                let Some(close_rel) = measure_text[pos..].find("</note>") else {
                    break;
                };
                let end = pos + close_rel + "</note>".len();
                let block = &measure_text[pos..end];
                let duration = partial_beat_first_i64(block, "duration")
                    .unwrap_or(0)
                    .max(0);
                if !block.contains("<chord") {
                    let before = cursor;
                    let after = cursor.saturating_add(duration);
                    if after > expected_duration || before >= expected_duration {
                        if after > expected_duration {
                            state.overflow_events = state.overflow_events.saturating_add(1);
                        }
                        state.post_expected_events = state.post_expected_events.saturating_add(1);
                    }
                    cursor = after;
                    state.cursor_min = state.cursor_min.min(cursor);
                    state.cursor_max = state.cursor_max.max(cursor);
                } else if cursor > expected_duration {
                    state.post_expected_events = state.post_expected_events.saturating_add(1);
                }
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = measure_text[pos..].find("</backup>") else {
                    break;
                };
                let end = pos + close_rel + "</backup>".len();
                let duration = partial_beat_first_i64(&measure_text[pos..end], "duration")
                    .unwrap_or(0)
                    .max(0);
                cursor = cursor.saturating_sub(duration);
                if cursor != 0 {
                    state.partial_backup_count = state.partial_backup_count.saturating_add(1);
                }
                state.cursor_min = state.cursor_min.min(cursor);
                state.cursor_max = state.cursor_max.max(cursor);
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = measure_text[pos..].find("</forward>") else {
                    break;
                };
                let end = pos + close_rel + "</forward>".len();
                let duration = partial_beat_first_i64(&measure_text[pos..end], "duration")
                    .unwrap_or(0)
                    .max(0);
                let before = cursor;
                let after = cursor.saturating_add(duration);
                if after > expected_duration || before >= expected_duration {
                    if after > expected_duration {
                        state.overflow_events = state.overflow_events.saturating_add(1);
                    }
                    state.post_expected_events = state.post_expected_events.saturating_add(1);
                }
                cursor = after;
                state.cursor_min = state.cursor_min.min(cursor);
                state.cursor_max = state.cursor_max.max(cursor);
                scan = end;
            }
            _ => break,
        }
    }
    state.cursor_final = cursor;
    state
}

fn beat_scoped_plain_visible_rest(block: &str) -> bool {
    block.contains("<rest")
        && !block.contains("<chord")
        && !block.contains("print-object=\"no\"")
        && !block.contains("print-object='no'")
        && !block.contains("print-spacing=\"no\"")
        && !block.contains("print-spacing='no'")
        && !block.contains("<pitch")
        && !block.contains("<beam")
        && !block.contains("<tie")
        && !block.contains("<notations")
        && !block.contains("<stem")
        && !block.contains("default-x=")
        && !block.contains("default-y=")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeatScopedOpenRestRun {
    start: usize,
    last_end: usize,
    cursor_start: i64,
    staff: String,
    voice: String,
    count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeatScopedCrossingRestSuffix {
    run_start: usize,
    last_rest_end: usize,
    backup_end: usize,
    rest_start: i64,
    old_lane_end: i64,
    staff: String,
    voice: String,
    rewritten_rests: usize,
}

fn beat_scoped_rest_completion_crossing_rest_suffix(
    measure_text: &str,
    expected_duration: i64,
) -> Option<BeatScopedCrossingRestSuffix> {
    let mut cursor = 0_i64;
    let mut scan = 0_usize;
    let mut rest_run: Option<BeatScopedOpenRestRun> = None;
    while scan < measure_text.len() {
        let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
        let Some((pos, kind)) = [
            next_note.map(|p| (p, "note")),
            next_backup.map(|p| (p, "backup")),
            next_forward.map(|p| (p, "forward")),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(p, _)| *p)
        else {
            break;
        };
        match kind {
            "note" => {
                let Some(close_rel) = measure_text[pos..].find("</note>") else {
                    break;
                };
                let end = pos + close_rel + "</note>".len();
                let block = &measure_text[pos..end];
                let duration = partial_beat_first_i64(block, "duration")
                    .unwrap_or(0)
                    .max(0);
                let before = cursor;
                if !block.contains("<chord") {
                    cursor = cursor.saturating_add(duration);
                }
                if duration > 0 && beat_scoped_plain_visible_rest(block) {
                    let staff = partial_beat_first_text(block, "staff")
                        .unwrap_or_else(|| "1".to_string());
                    let voice = partial_beat_first_text(block, "voice")
                        .unwrap_or_else(|| "1".to_string());
                    rest_run = match rest_run.take() {
                        Some(mut run)
                            if run.staff == staff
                                && run.voice == voice
                                && run.last_end <= pos
                                && measure_text[run.last_end..pos].trim().is_empty() =>
                        {
                            run.last_end = end;
                            run.count = run.count.saturating_add(1);
                            Some(run)
                        }
                        _ => Some(BeatScopedOpenRestRun {
                            start: pos,
                            last_end: end,
                            cursor_start: before,
                            staff,
                            voice,
                            count: 1,
                        }),
                    };
                } else {
                    rest_run = None;
                }
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = measure_text[pos..].find("</backup>") else {
                    break;
                };
                let end = pos + close_rel + "</backup>".len();
                let duration = partial_beat_first_i64(&measure_text[pos..end], "duration")
                    .unwrap_or(0)
                    .max(0);
                if duration == expected_duration
                    && cursor > expected_duration
                    && let Some(run) = rest_run.take()
                    && run.cursor_start >= 0
                    && run.cursor_start < expected_duration
                    && run.start < run.last_end
                    && run.last_end <= pos
                    && measure_text[run.last_end..pos].trim().is_empty()
                {
                    return Some(BeatScopedCrossingRestSuffix {
                        run_start: run.start,
                        last_rest_end: run.last_end,
                        backup_end: end,
                        rest_start: run.cursor_start,
                        old_lane_end: cursor,
                        staff: run.staff,
                        voice: run.voice,
                        rewritten_rests: run.count,
                    });
                }
                cursor = cursor.saturating_sub(duration);
                rest_run = None;
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = measure_text[pos..].find("</forward>") else {
                    break;
                };
                let end = pos + close_rel + "</forward>".len();
                let duration = partial_beat_first_i64(&measure_text[pos..end], "duration")
                    .unwrap_or(0)
                    .max(0);
                cursor = cursor.saturating_add(duration);
                rest_run = None;
                scan = end;
            }
            _ => break,
        }
    }
    None
}

fn close_beat_scoped_rest_completion_crossing_rest_suffix(
    measure_text: &str,
    expected_duration: i64,
    beat_unit: i64,
) -> Option<(String, usize)> {
    let suffix =
        beat_scoped_rest_completion_crossing_rest_suffix(measure_text, expected_duration)?;
    let overflow = suffix.old_lane_end.checked_sub(expected_duration)?;
    let retained_duration = expected_duration.checked_sub(suffix.rest_start)?;
    if overflow <= 0 || retained_duration <= 0 {
        return None;
    }
    let parts = plan_exact_metric_visible_rest_durations(
        suffix.rest_start,
        retained_duration,
        expected_duration,
        beat_unit,
    )?;
    if parts.is_empty() || parts.iter().copied().sum::<i64>() != retained_duration {
        return None;
    }
    let rest_replacement = parts
        .into_iter()
        .map(|duration| {
            beat_scoped_visible_rest_note(duration, &suffix.staff, &suffix.voice)
        })
        .collect::<Vec<_>>()
        .join("");
    let mut candidate = String::with_capacity(measure_text.len() + 128);
    candidate.push_str(&measure_text[..suffix.run_start]);
    candidate.push_str(&rest_replacement);
    candidate.push_str(&measure_text[suffix.last_rest_end..suffix.backup_end]);
    candidate.push_str(&format!(
        "\n        <forward>\n          <duration>{overflow}</duration>\n        </forward>"
    ));
    candidate.push_str(&measure_text[suffix.backup_end..]);

    if beat_scoped_rest_completion_pitched_timeline(measure_text)
        != beat_scoped_rest_completion_pitched_timeline(&candidate)
    {
        return None;
    }
    if !beat_scoped_rest_completion_cursor_state(&candidate, expected_duration)
        .is_green(expected_duration)
    {
        return None;
    }
    if beat_scoped_rest_completion_crossing_rest_suffix(&candidate, expected_duration).is_some() {
        return None;
    }
    Some((candidate, suffix.rewritten_rests))
}


fn close_beat_scoped_rest_completion_crossing_rests_in_xml(
    xml: &str,
) -> ((String, usize), Option<BlockTransformationReceipt>) {
    
    
    let output = {
    let mut out = String::with_capacity(xml.len() + 256);
    let mut scan = 0_usize;
    let mut divisions = 1_i64;
    let mut beats = 4_i64;
    let mut beat_type = 4_i64;
    let mut closed_measures = 0_usize;

    while let Some((start, end, _)) = find_next_measure_bounds(xml, scan) {
        out.push_str(&xml[scan..start]);
        let measure_xml = &xml[start..end];
        let (next_divisions, next_beats, next_beat_type, expected_duration) =
            parse_musicxml_current_meter(
                measure_xml,
                divisions,
                beats,
                beat_type,
            );
        divisions = next_divisions;
        beats = next_beats;
        beat_type = next_beat_type;

        let beat_unit = divisions.max(1);
        if let Some((closed, _rewritten_rests)) =
            close_beat_scoped_rest_completion_crossing_rest_suffix(
                measure_xml,
                expected_duration,
                beat_unit,
            )
        {
            out.push_str(&closed);
            closed_measures = closed_measures.saturating_add(1);
        } else {
            out.push_str(measure_xml);
        }
        scan = end;
    }
    out.push_str(&xml[scan..]);
    (out, closed_measures)
};
    
    let receipt = Some(output.0.as_str()).filter(|post| *post != xml).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow54BlockMint { _private: () }, xml, post, Some((0, xml.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

fn normalize_metric_visible_rests_with_actual_carry(
    measure_text: &str,
    measure_duration: i64,
    beat_unit: i64,
) -> (String, usize, ActualBeamNoteMap) {
    let raw_notes = partial_beat_raw_notes(measure_text);
    if raw_notes.is_empty() {
        return (measure_text.to_string(), 0, ActualBeamNoteMap::unchanged(&raw_notes));
    }
    let note_infos = partial_beat_note_infos(measure_text, &raw_notes);
    let mut replacements: BTreeMap<usize, String> = BTreeMap::new();
    let mut rewritten = 0_usize;
    for info in &note_infos {
        if !info.is_rest || info.is_hidden || info.is_chord || info.duration <= 0 {
            continue;
        }
        let Some(durations) = plan_exact_metric_visible_rest_durations(
            info.start,
            info.duration,
            measure_duration,
            beat_unit,
        ) else {
            continue;
        };
        if durations.len() == 1 && durations[0] == info.duration {
            continue;
        }
        let replacement = durations
            .into_iter()
            .map(|duration| {
                beat_scoped_visible_rest_note(duration, &info.staff, &info.voice)
            })
            .collect::<Vec<_>>()
            .join("");
        replacements.insert(info.index, replacement);
        rewritten += 1;
    }
    if replacements.is_empty() {
        return (measure_text.to_string(), 0, ActualBeamNoteMap::unchanged(&raw_notes));
    }
    let mut actual_note_map=ActualBeamNoteMap::default();
    let mut rebuilt = String::with_capacity(measure_text.len() + rewritten * 160);
    let mut last = 0_usize;
    for note in &raw_notes {
        rebuilt.push_str(&measure_text[last..note.start]);
        let emitted_start=rebuilt.len();
        if let Some(replacement) = replacements.get(&note.start) {
            rebuilt.push_str(replacement);
        } else {
            rebuilt.push_str(&note.text);
            actual_note_map.0.push((note.start..note.end,emitted_start..rebuilt.len()));
        }
        last = note.end;
    }
    rebuilt.push_str(&measure_text[last..]);
    (rebuilt, rewritten, actual_note_map)
}

fn add_visible_voice_rest_completion(
    plans: &[BeatScopedRestCompletionGapPlan],
    measure_duration: i64,
    beat_unit: i64,
    inserts_before: &mut BTreeMap<usize, Vec<String>>,
    inserted_rest_staffs: &mut BTreeSet<String>,
    stats: &mut BeatScopedRestPatchStats,
) {
    for plan in plans {
        let Some(durations) = plan_exact_metric_visible_rest_durations(
            plan.gap_start,
            plan.gap_end - plan.gap_start,
            measure_duration,
            beat_unit,
        ) else {
            continue;
        };
        let mut created = Vec::new();
        for duration in durations {
            created.push(beat_scoped_visible_rest_note(
                duration,
                &plan.staff,
                &plan.voice,
            ));
            stats.rest_notes_inserted += 1;
        }
        if !created.is_empty() {
            inserts_before
                .entry(plan.insert_before)
                .or_default()
                .extend(created);
            inserted_rest_staffs.insert(plan.staff.clone());
        }
    }
}
fn beat_scoped_visible_rest_note(duration: i64, staff: &str, voice: &str) -> String {
    let (note_type, dots) = beat_scoped_rest_type_and_dots(duration);
    let dots_text = (0..dots).map(|_| "        <dot/>\n").collect::<String>();
    format!(
        "      <note>\n        <rest/>\n        <duration>{}</duration>\n        <type>{}</type>\n{}        <voice>{}</voice>\n        <staff>{}</staff>\n      </note>\n",
        duration.max(1),
        note_type,
        dots_text,
        voice,
        staff
    )
}
fn beat_scoped_rest_type_and_dots(duration: i64) -> (&'static str, usize) {
    match duration {
        384 => ("whole", 0),
        288 => ("half", 1),
        192 => ("half", 0),
        144 => ("quarter", 1),
        96 => ("quarter", 0),
        72 => ("eighth", 1),
        48 => ("eighth", 0),
        36 => ("16th", 1),
        24 => ("16th", 0),
        18 => ("32nd", 1),
        12 => ("32nd", 0),
        _ if duration >= 192 => ("half", 0),
        _ if duration >= 96 => ("quarter", 0),
        _ if duration >= 48 => ("eighth", 0),
        _ if duration >= 24 => ("16th", 0),
        _ => ("32nd", 0),
    }
}
fn beat_scoped_mark_note_hidden(note_text: &str) -> (String, Option<BlockTransformationReceipt>) {
    
    
    let output = (|| {
    if note_text.contains("print-object=\"no\"") {
        return note_text.to_string();
    }
    if let Some(open_end) = note_text.find('>') {
        let mut out = String::with_capacity(note_text.len() + 40);
        out.push_str(&note_text[..open_end]);
        out.push_str(" print-object=\"no\" print-spacing=\"no\"");
        out.push_str(&note_text[open_end..]);
        out
    } else {
        note_text.to_string()
    }
})();
    
    let receipt = Some(output.as_str()).filter(|post| *post != note_text).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow19BlockMint { _private: () }, note_text, post, Some((0, note_text.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}
fn patch_partial_beat_post_rest_beams_in_measure(measure_text: &str, divisions: i64) -> (String, PartialBeatPostRestBeamPatchStats) {
    let raw_notes = partial_beat_raw_notes(measure_text);
    if raw_notes.is_empty() {
        return (measure_text.to_string(), PartialBeatPostRestBeamPatchStats::default());
    }
    let note_infos = partial_beat_note_infos(measure_text, &raw_notes);
    let beat_unit = divisions.max(1);
    let mut groups: BTreeMap<(String, String, i64), Vec<usize>> = BTreeMap::new();
    for info in &note_infos {
        if partial_beat_is_beam_candidate(info) {
            groups
                .entry((info.staff.clone(), info.voice.clone(), info.start.div_euclid(beat_unit)))
                .or_default()
                .push(info.index);
        }
    }
    let mut states_by_note: BTreeMap<usize, Vec<(usize, &'static str)>> = BTreeMap::new();
    let mut stats = PartialBeatPostRestBeamPatchStats::default();
    for (_key, mut indices) in groups {
        indices.sort_by_key(|idx| {
            note_infos
                .iter()
                .find(|info| info.index == *idx)
                .map_or((0, *idx), |info| (info.start, info.index))
        });
        let group = indices
            .iter()
            .filter_map(|idx| note_infos.iter().find(|info| info.index == *idx))
            .collect::<Vec<_>>();
        if !partial_beat_group_needs_post_rest_beam(&note_infos, &group, beat_unit) {
            continue;
        }
        if group.iter().all(|info| info.existing_beams > 0) {
            continue;
        }
        let Some(beam_tags_added) =
            partial_beat_extend_states_from_plan(&group, &mut states_by_note)
        else {
            continue;
        };
        stats.groups += 1;
        stats.notes += group.len();
        stats.beam_tags_added += beam_tags_added;
    }
    if states_by_note.is_empty() {
        return (measure_text.to_string(), stats);
    }
    let mut rebuilt = String::with_capacity(measure_text.len() + stats.beam_tags_added * 40);
    let mut last = 0_usize;
    for note in &raw_notes {
        rebuilt.push_str(&measure_text[last..note.start]);
        if let Some(states) = states_by_note.get(&note.start) {
            let cleaned = partial_beat_strip_beam_tags(&note.text);
            let replacement = partial_beat_insert_beam_tags(&cleaned, states);
            stats.representation_edits.push((note.start..note.end, replacement.clone()));
            rebuilt.push_str(&replacement);
        } else {
            rebuilt.push_str(&note.text);
        }
        last = note.end;
    }
    rebuilt.push_str(&measure_text[last..]);
    (rebuilt, stats)
}
// v0.10.1278: Behind-Bars authority is structural and scorewide.
// No measure role or external-feedback locator is permitted in productive beaming.
pub(crate) struct StructuralRestBridgeVariantAPreservingMint {
    pub(crate) _sealed: (),
}

#[derive(Debug, Clone, Default)]
struct SisterFeedbackBeamVariantATinyRenderWriteAudit {
    source_label: String,
    output_path: String,
    attempted: bool,
    written: bool,
    target_measure: String,
    target_staff: String,
    target_voice: String,
    target_rows: usize,
    written_rows: usize,
    beam_level1_rows: usize,
    beam_level2_rows: usize,
    variant_b_blocked: bool,
    write_footprint_outside_decision_scope_rows: usize,
    write_footprint_within_decision_scope: bool,
    parse_after_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_after_ok: bool,
    sounding_stable: bool,
    verdict: String,
    done_fragment: String,
    pipeline_line: String,
    frozen_voice_split_groups: usize,
    // r1485: withheld split groups whose rewrite would not change any owned beam
    // state; reported separately so `split_groups` counts only real withholdings.
    frozen_voice_unchanged_split_groups: usize,
    frozen_voice_unbound_groups: usize,
}
fn apply_structural_rest_bridge_beam_variant_a_render_write( xml: &str, source_label: &str, output_path: &std::path::Path, voice_guard: &structural_write_authority::CanonicalFinalChordStructuralPlan, ) -> ((String, SisterFeedbackBeamVariantATinyRenderWriteAudit), Option<PreservingTransformationReceipt>, Option<structural_write_authority::RestBridgeExtensionOccurrenceEvidence>) {
    let mut authored_groups = Vec::<Vec<(usize,usize)>>::new();
    // r1484: parallel to authored_groups; true marks a group split in the sealed
    // plan that is written only provisionally, with its own bound evidence.
    let mut provisional_flags = Vec::<bool>::new();
    let withdrawal = voice_guard.rest_bridge_withdrawal_rows(xml).expect("authentic exact bridge withdrawal input");
    let finish = |post: String, audit: SisterFeedbackBeamVariantATinyRenderWriteAudit, groups: &[Vec<(usize,usize)>], flags: &[bool]| {
        let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
            StructuralRestBridgeVariantAPreservingMint { _sealed: () },
            xml,
            &post,
        );
        assert!(
            post == xml || receipt.is_some(),
            "E050 changed bytes require its PRESERVING receipt"
        );
        let (evidence, bound) = match (withdrawal.is_none(), receipt.as_ref()) {
            (true, Some(receipt)) => voice_guard
                .capture_rest_bridge_extension_output_offered(xml, &post, groups, flags, receipt)
                .expect("E050 writer-authored extension binding"),
            _ => (None, std::collections::BTreeSet::new()),
        };
        let offered = flags.iter().enumerate().filter(|(_, provisional)| **provisional)
            .map(|(index, _)| index).collect::<std::collections::BTreeSet<_>>();
        if !offered.is_subset(&bound) {
            // Provisional groups without complete evidence are never written:
            // discard this pass and repeat with only the evidenced ones offered.
            let mut chain = crate::master_state_identity::RenderTransitionReceipts::new();
            chain.push_preserving(receipt, &post);
            chain.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            let kept = offered.intersection(&bound).map(|index| groups[*index].clone()).collect();
            return apply_structural_rest_bridge_beam_variant_a_render_write(
                xml, &audit.source_label, output_path, &voice_guard.with_bound_provisional_bridges(kept));
        }
        eprintln!("BridgeFrozenVoiceCompatibility: file={}; split_groups={}; unchanged_split_groups={}; unbound_groups={}; provisional_bound={}; phase=proposal_evaluated_not_adoption_evidence", audit.output_path, audit.frozen_voice_split_groups, audit.frozen_voice_unchanged_split_groups, audit.frozen_voice_unbound_groups, offered.len());
        ((post, audit), receipt, evidence)
    };
    let mut audit = SisterFeedbackBeamVariantATinyRenderWriteAudit {
        source_label: source_label.to_string(),
        output_path: output_path.display().to_string(),
        attempted: true,
        variant_b_blocked: true,
        write_footprint_outside_decision_scope_rows: 0,
        write_footprint_within_decision_scope: true,
        parse_after_ok: true,
        backup_forward_after_ok: true,
        raw_cursor_after_ok: true,
        sounding_stable: true,
        verdict: "external_notation_feedback_beam_variant_a_no_matching_cell".to_string(),
        ..SisterFeedbackBeamVariantATinyRenderWriteAudit::default()
    };
    // v0.10.442: Dieser Write materialisiert nur den freigegebenen Variante-A-Vertrag.
    // Er setzt ausschliesslich Beam-Tags auf vorhandene Nachbar-Noten in einer dynamisch
    // ausgelesenen lower-staff/voice-Zelle. Rest-Bridge bleibt Kontext, nicht produktiver
    // MusicXML-Rest-Beam-Tag. Der Write-Footprint ist exakt die Vereinigung der
    // BeamPlan-Zielnoten im entschiedenen Beat; Folgebeats bleiben byteidentisch.
    // Pitch, Onset, Dauer und Pausen bleiben unveraendert.
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        audit.done_fragment = "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback: keine Measure-Spans; RenderXmlWrite nein; MusicMutation nein".to_string();
        audit.pipeline_line = format!(
            "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback v{}: Source {}; Output {}; attempted {}; written nein; VariantBBlocked {}; keine Measure-Spans; RenderXmlWrite nein; MusicMutation nein.",
            env!("CARGO_PKG_VERSION"),
            audit.source_label,
            audit.output_path,
            yes_no(audit.attempted),
            yes_no(audit.variant_b_blocked)
        );
        return finish(xml.to_string(), audit, &authored_groups, &provisional_flags);
    }
    let mut out = String::with_capacity(xml.len() + 256);
    let mut last = 0_usize;
    let mut inherited_divisions = 1_i64;
    let mut affected_measures = Vec::<String>::new();
    for span in spans {
        out.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let measure_no = partial_beat_measure_number(measure_text).unwrap_or_default();
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let beat_unit = inherited_divisions.max(1);
        let restore_rows = withdrawal.into_iter().flat_map(|rows| rows.iter())
            .filter(|(begin,end,_)| *begin >= span.start && *end <= span.end).collect::<Vec<_>>();
        let candidate_groups = if withdrawal.is_some() { Vec::new() } else { choose_structural_rest_bridge_candidate_groups(&infos, beat_unit) };
        if candidate_groups.is_empty() && restore_rows.is_empty() {
            out.push_str(measure_text);
            last = span.end;
            continue;
        }
        let target_staff = candidate_groups
            .first()
            .map_or_else(|| "2".to_string(), |(_, staff, _, _, _, _)| staff.clone());
        let target_voice = candidate_groups
            .first()
            .map_or_else(|| "2".to_string(), |(_, _, voice, _, _, _)| voice.clone());
        let mut states: BTreeMap<usize, Vec<(usize, &'static str)>> = restore_rows.iter()
            .map(|(begin,_,rows)| (begin - span.start, rows.clone())).collect();
        let mut target_rows = states.len();
        for (
            _target_beat,
            _staff,
            _voice,
            mut targets,
            _has_rest_bridge,
            _has_mixed_hierarchy,
        ) in candidate_groups
        {
            targets.sort_by_key(|info| (info.start, info.index));
            let members = targets.iter().filter_map(|info| raw_notes.iter().find(|raw| raw.start == info.index)
                .map(|raw| (span.start + raw.start, span.start + raw.end))).collect::<Vec<_>>();
            let admission = if members.len() == targets.len() {
                voice_guard.bridge_voice_admission(xml, &members)
            } else { None };
            match admission {
                Some(true) => provisional_flags.push(false),
                Some(false) if voice_guard.provisional_bridge_offered(&members) => provisional_flags.push(true),
                Some(false) => {
                    let unchanged = structural_rest_bridge_beam_states(&targets).iter().all(|(idx, rows)| {
                        raw_notes.iter().find(|raw| raw.start == *idx).and_then(|raw| partial_beat_observed_beam_states(&raw.text))
                            .is_some_and(|observed| {
                                let mut owned = observed.into_iter().filter(|(level, _)| *level <= 2).collect::<Vec<_>>();
                                let mut planned = rows.iter().map(|(level, state)| (*level, (*state).to_owned())).collect::<Vec<_>>();
                                owned.sort();
                                planned.sort();
                                owned == planned
                            })
                    });
                    if unchanged {
                        audit.frozen_voice_unchanged_split_groups += 1;
                    } else {
                        audit.frozen_voice_split_groups += 1;
                    }
                    continue;
                },
                None => {
                    audit.frozen_voice_unbound_groups += 1;
                    continue;
                },
            }
            authored_groups.push(members);
            target_rows += targets.len();
            for (idx, rows) in structural_rest_bridge_beam_states(&targets) {
                states.entry(idx).or_default().extend(rows);
            }
        }
        if states.is_empty() {
            out.push_str(measure_text);
            last = span.end;
            continue;
        }
        let mut rebuilt = String::with_capacity(measure_text.len() + 256);
        let mut note_last = 0_usize;
        let mut written_rows = 0_usize;
        let mut beam_level1_rows = 0_usize;
        let mut beam_level2_rows = 0_usize;
        for raw in &raw_notes {
            rebuilt.push_str(&measure_text[note_last..raw.start]);
            if let Some(state_rows) = states.get(&raw.start) {
                let rewritten = partial_beat_rewrite_owned_beam_levels(&raw.text, state_rows, 2)
                    .unwrap_or_else(|| raw.text.clone());
                rebuilt.push_str(&rewritten);
                written_rows += 1;
                if state_rows.iter().any(|(level, _)| *level == 1) { beam_level1_rows += 1; }
                if state_rows.iter().any(|(level, _)| *level == 2) { beam_level2_rows += 1; }
            } else {
                rebuilt.push_str(&raw.text);
            }
            note_last = raw.end;
        }
        rebuilt.push_str(&measure_text[note_last..]);
        let rebuilt_raw_notes = partial_beat_raw_notes(&rebuilt);
        let mut outside_scope_rows = raw_notes.len().abs_diff(rebuilt_raw_notes.len());
        for (ordinal, raw) in raw_notes.iter().enumerate() {
            if states.contains_key(&raw.start) {
                continue;
            }
            if rebuilt_raw_notes
                .get(ordinal)
                .is_none_or(|after_raw| after_raw.text != raw.text)
            {
                outside_scope_rows += 1;
            }
        }
        audit.write_footprint_outside_decision_scope_rows = audit
            .write_footprint_outside_decision_scope_rows
            .saturating_add(outside_scope_rows);
        out.push_str(&rebuilt);
        last = span.end;
        let changed = written_rows > 0 && rebuilt != measure_text;
        if changed {
            audit.written = true;
            if !affected_measures.contains(&measure_no) {
                affected_measures.push(measure_no.clone());
            }
            audit.target_measure = affected_measures.join(",");
            audit.target_staff = target_staff;
            audit.target_voice = target_voice;
            audit.target_rows = audit.target_rows.saturating_add(target_rows);
            audit.written_rows = audit.written_rows.saturating_add(written_rows);
            audit.beam_level1_rows = audit.beam_level1_rows.saturating_add(beam_level1_rows);
            audit.beam_level2_rows = audit.beam_level2_rows.saturating_add(beam_level2_rows);
            audit.verdict = "scorewide_behind_bars_variant_a_beaming_written_render_xml_only".to_string();
        }
    }
    out.push_str(&xml[last..]);
    audit.write_footprint_within_decision_scope =
        audit.write_footprint_outside_decision_scope_rows == 0;
    if !audit.write_footprint_within_decision_scope {
        audit.written = false;
        audit.verdict = "variant_a_write_footprint_outside_decision_scope_blocked".to_string();
        audit.done_fragment = format!(
            "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback: Write-Footprint ausserhalb Decision-Scope erkannt; rows {}; RenderXmlWrite nein; fail-closed",
            audit.write_footprint_outside_decision_scope_rows
        );
        audit.pipeline_line = format!(
            "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback v{}: Source {}; Output {}; attempted {}; WriteFootprintOutsideDecisionScopeRows {}; DecisionScopeWriteFootprintGreen nein; VariantBBlocked {}; RenderXmlWrite nein; fail-closed; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            audit.source_label,
            audit.output_path,
            yes_no(audit.attempted),
            audit.write_footprint_outside_decision_scope_rows,
            yes_no(audit.variant_b_blocked),
            audit.verdict
        );
        return finish(xml.to_string(), audit, &authored_groups, &provisional_flags);
    }
    if !audit.written {
        audit.done_fragment = "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback: kein sicherer Variante-A-Schreibkandidat; RenderXmlWrite nein; MusicMutation nein".to_string();
        audit.pipeline_line = format!(
            "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback v{}: Source {}; Output {}; attempted {}; written nein; VariantBBlocked {}; RenderXmlWrite nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            audit.source_label,
            audit.output_path,
            yes_no(audit.attempted),
            yes_no(audit.variant_b_blocked),
            audit.verdict
        );
        return finish(xml.to_string(), audit, &authored_groups, &provisional_flags);
    }
    audit.done_fragment = format!(
        "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback: Variante A geschrieben; Scope {} staff {} voice {}; rows {}; Beam1 {}; Beam2 {}; WriteFootprintOutsideDecisionScopeRows {}; DecisionScopeWriteFootprintGreen {}; VariantBBlocked ja; RenderXmlWrite ja; MusicMutation nein",
        audit.target_measure, audit.target_staff, audit.target_voice, audit.written_rows, audit.beam_level1_rows, audit.beam_level2_rows, audit.write_footprint_outside_decision_scope_rows, yes_no(audit.write_footprint_within_decision_scope)
    );
    audit.pipeline_line = format!(
        "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback v{}: Source {}; Output {}; attempted {}; target {} staff {} voice {}; target_rows {}; written_rows {}; BeamLevel1Rows {}; BeamLevel2Rows {}; WriteFootprintOutsideDecisionScopeRows {}; DecisionScopeWriteFootprintGreen {}; VariantAOnly ja; VariantBBlocked {}; no_crossbeat_merge ja; no_duration_change ja; no_pitch_change ja; parse/back-forward/raw {}/{}/{}; SoundingStable {}; RenderXmlWrite ja; MusicXmlWrite nein; PdfWrite source-selected; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.output_path,
        yes_no(audit.attempted),
        audit.target_measure,
        audit.target_staff,
        audit.target_voice,
        audit.target_rows,
        audit.written_rows,
        audit.beam_level1_rows,
        audit.beam_level2_rows,
        audit.write_footprint_outside_decision_scope_rows,
        yes_no(audit.write_footprint_within_decision_scope),
        yes_no(audit.variant_b_blocked),
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        if audit.raw_cursor_after_ok { "ok" } else { "fail" },
        yes_no(audit.sounding_stable),
        audit.verdict
    );
    finish(out, audit, &authored_groups, &provisional_flags)
}

#[cfg(test)]
mod variant_a_decision_scope_write_footprint_tests {
    use super::StructuralRestBridgeVariantAPreservingMint;
    use super::{
        apply_structural_rest_bridge_beam_variant_a_render_write,
        close_committed_cross_staff_beam_gestures,
    };
    use std::path::Path;

    #[test]
    fn mixed_hierarchy_write_keeps_following_beat_byte_identical() {
        let following_beat = r#"<note><pitch><step>F</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">begin</beam><beam number="2">begin</beam></note>
<note><pitch><step>G</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">continue</beam><beam number="2">continue</beam></note>
<note><pitch><step>A</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">continue</beam><beam number="2">continue</beam></note>
<note><pitch><step>B</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">end</beam><beam number="2">end</beam></note>"#;
        let xml = format!(
            r#"<score-partwise><part id="P1"><measure number="scope"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
<note><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
{following_beat}</measure></part></score-partwise>"#
        );
        let ((after, audit), receipt, _pending_extension) = apply_structural_rest_bridge_beam_variant_a_render_write(
            &xml,
            "synthetic decision-scope proof",
            Path::new("test.render.musicxml"),
        
            &crate::structural_write_authority::CanonicalFinalChordStructuralPlan::synthetic_bridge_voice_guard(&xml),
        );

        assert!(audit.written);
        let receipt = receipt.expect("E050 changed post must carry preserving receipt");
        assert!(receipt
            .validate_preserving_transition::<StructuralRestBridgeVariantAPreservingMint>(&xml, &after)
            .is_ok());
        assert_eq!(audit.write_footprint_outside_decision_scope_rows, 0);
        assert!(audit.write_footprint_within_decision_scope);
        assert!(after.contains(following_beat));
        let target_e_start = after.find("<step>E</step>").expect("target E note");
        let target_e_end = after[target_e_start..]
            .find("</note>")
            .map(|relative| target_e_start + relative + "</note>".len())
            .expect("target E note end");
        let target_e_note = &after[target_e_start..target_e_end];
        assert!(target_e_note.contains(r#"<beam number="1">end</beam>"#));
        assert!(target_e_note.contains(r#"<beam number="2">end</beam>"#));

        // Historical counterexample A: a visible rest carrier must not become a BeamPlan event.
        {
            let xml = r#"<score-partwise><part id="P1"><measure number="scope"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
<note><rest/><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
</measure></part></score-partwise>"#;
            let ((after_early, early_audit), early_receipt, _pending_extension) = apply_structural_rest_bridge_beam_variant_a_render_write(
                xml,
                "synthetic rest-carrier counterexample proof",
                Path::new("test.render.musicxml"),
            
            &crate::structural_write_authority::CanonicalFinalChordStructuralPlan::synthetic_bridge_voice_guard(xml),
        );
            assert!(early_audit.written);
            let early_receipt = early_receipt.expect("E050 changed rest-carrier post must carry receipt");
            assert!(early_receipt
                .validate_preserving_transition::<StructuralRestBridgeVariantAPreservingMint>(xml, &after_early)
                .is_ok());
            let mut early_probe = crate::master_state_identity::RenderTransitionReceipts::new();
            early_probe.push_preserving(Some(early_receipt), &after_early);
            early_probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            let rest_start = after_early.find("<rest/>").expect("rest");
            let rest_note_start = after_early[..rest_start].rfind("<note").expect("rest note start");
            let rest_note_end = after_early[rest_start..]
                .find("</note>")
                .map(|relative| rest_start + relative + "</note>".len())
                .expect("rest note end");
            assert!(!after_early[rest_note_start..rest_note_end].contains("<beam"));

            // Historical counterexample B: an early begin must survive the late checkpoint.
            let first_note_end = after_early.find("</note>").expect("first note end") + "</note>".len();
            let first_note = &after_early[..first_note_end];
            assert!(first_note.contains(r#"<beam number="1">begin</beam>"#));
            assert!(first_note.contains(r#"<beam number="2">begin</beam>"#));

            let mut after_late = after_early.clone();
            let (_late_stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after_late);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after_late);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            let first_note_end = after_late.find("</note>").expect("final first note end") + "</note>".len();
            let first_note = &after_late[..first_note_end];
            assert!(first_note.contains(r#"<beam number="1">begin</beam>"#));
            assert!(first_note.contains(r#"<beam number="2">begin</beam>"#));
            let rest_start = after_late.find("<rest/>").expect("final rest");
            let rest_note_start = after_late[..rest_start].rfind("<note").expect("final rest note start");
            let rest_note_end = after_late[rest_start..]
                .find("</note>")
                .map(|relative| rest_start + relative + "</note>".len())
                .expect("final rest note end");
            assert!(!after_late[rest_note_start..rest_note_end].contains("<beam"));
        }

        // Historical counterexample C: a silent four-quarter-rest voice keeps full visible coverage.
        {
            let xml = r#"<score-partwise><part id="P1"><measure number="scope"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>
<note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
<note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
<note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
<note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
</measure></part></score-partwise>"#;
            let ((after_early, early_audit), early_receipt, _pending_extension) = apply_structural_rest_bridge_beam_variant_a_render_write(
                xml,
                "synthetic silent-voice coverage counterexample proof",
                Path::new("test.render.musicxml"),
            
            &crate::structural_write_authority::CanonicalFinalChordStructuralPlan::synthetic_bridge_voice_guard(xml),
        );
            assert!(!early_audit.written);
            assert!(early_receipt.is_none());
            assert_eq!(after_early, xml);
            let mut after_late = after_early;
            let (_late_stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after_late);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after_late);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(after_late, xml);
            assert_eq!(after_late.matches("<rest/>").count(), 4);
        }

        // The L1/L2 convergence must preserve pre-existing higher beam authority as read-through.
        {
            let xml = r#"<score-partwise><part id="P1"><measure number="scope"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>2</voice><type>32nd</type><staff>2</staff><beam number="3">begin</beam></note>
<note><rest/><duration>12</duration><voice>2</voice><type>32nd</type><staff>2</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>2</voice><type>32nd</type><staff>2</staff><beam number="3">end</beam></note>
</measure></part></score-partwise>"#;
            let ((after, audit), receipt, _pending_extension) = apply_structural_rest_bridge_beam_variant_a_render_write(
                xml,
                "synthetic higher-level read-through proof",
                Path::new("test.render.musicxml"),
            
            &crate::structural_write_authority::CanonicalFinalChordStructuralPlan::synthetic_bridge_voice_guard(xml),
        );
            assert!(audit.written);
            let receipt = receipt.expect("E050 higher-level changed post must carry receipt");
            assert!(receipt
                .validate_preserving_transition::<StructuralRestBridgeVariantAPreservingMint>(xml, &after)
                .is_ok());
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(Some(receipt), &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert!(after.contains(r#"<beam number="3">begin</beam>"#));
            assert!(after.contains(r#"<beam number="3">end</beam>"#));
        }

        // CrossStaffHandProjectionReadability: add this scenario to the already
        // writer-reaching owner test so no new Recovery-shard writer root is created.
        {
            let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
5\"><attributes><divisions>96</divisions></attributes>\n<note><pitch><step>B</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff><beam number=\"1\">begin</beam><beam number=\"2\">begin</beam></note>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff><beam number=\"1\">end</beam><beam number=\"2\">end</beam></note>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n<note><pitch><step>G</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>\n</measure></part></score-partwise>";
            let mut after = xml.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 1);
            assert_eq!(after.matches(r#"<beam number="1">begin</beam>"#).count(), 1);
            assert_eq!(after.matches(r#"<beam number="1">continue</beam>"#).count(), 2);
            assert_eq!(after.matches(r#"<beam number="1">end</beam>"#).count(), 1);
            assert_eq!(after.matches(r#"<beam number="2">begin</beam>"#).count(), 1);
            assert_eq!(after.matches(r#"<beam number="2">continue</beam>"#).count(), 2);
            assert_eq!(after.matches(r#"<beam number="2">end</beam>"#).count(), 1);
        }

        // CrossStaffDottedEighthSixteenthBeamRule: a complete beat remains one
        // metric beam unit when the musical voice changes staff. The shared plan
        // owns only L1 here; the renderer derives the isolated sixteenth hook.
        {
            let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
19\"><attributes><divisions>96</divisions></attributes>\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>72</duration><voice>1</voice><type>eighth</type><dot/><staff>2</staff></note>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>\n</measure></part></score-partwise>";
            let mut after = xml.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 1);
            assert_eq!(stats.level1_rows, 2);
            assert_eq!(stats.level2_rows, 0);
            assert_eq!(after.matches(r#"<beam number="1">begin</beam>"#).count(), 1);
            assert_eq!(after.matches(r#"<beam number="1">end</beam>"#).count(), 1);
            assert!(!after.contains(r#"<beam number="2">"#));

            let once = after.clone();
            let (second_stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(second_stats.cross_staff_groups, 0);
            assert_eq!(after, once);
        }

        // The inverse 1/4 + 3/4 ordering is the same complete-beat rule.
        {
            let xml = r#"<score-partwise><part id="P1"><measure number="mirror"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>72</duration><voice>1</voice><type>eighth</type><dot/><staff>2</staff></note>
</measure></part></score-partwise>"#;
            let mut after = xml.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 1);
            assert_eq!(after.matches(r#"<beam number="1">begin</beam>"#).count(), 1);
            assert_eq!(after.matches(r#"<beam number="1">end</beam>"#).count(), 1);
            assert!(!after.contains(r#"<beam number="2">"#));
        }

        // Same-staff and non-3/4+1/4 pairs remain outside this Cross-Staff owner.
        {
            let same_staff = r#"<score-partwise><part id="P1"><measure number="same"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>72</duration><voice>1</voice><type>eighth</type><dot/><staff>2</staff></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>
</measure></part></score-partwise>"#;
            let mut after = same_staff.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 0);
            assert_eq!(after, same_staff);

            let wrong_ratio = r#"<score-partwise><part id="P1"><measure number="ratio"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>60</duration><voice>1</voice><type>eighth</type><staff>2</staff></note>
<note><pitch><step>D</step><octave>5</octave></pitch><duration>36</duration><voice>1</voice><type>16th</type><staff>1</staff></note>
</measure></part></score-partwise>"#;
            let mut after = wrong_ratio.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 0);
            assert_eq!(after, wrong_ratio);

            let incomplete_beat = r#"<score-partwise><part id="P1"><measure number="incomplete"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>72</duration><voice>1</voice><type>eighth</type><dot/><staff>2</staff></note>
<note><pitch><step>D</step><octave>5</octave></pitch><duration>12</duration><voice>1</voice><type>16th</type><staff>1</staff></note>
</measure></part></score-partwise>"#;
            let mut after = incomplete_beat.to_string();
            let (stats, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
            probe.push_preserving(receipt, &after);
            probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
            assert_eq!(stats.cross_staff_groups, 0);
            assert_eq!(after, incomplete_beat);
        }
    }
}
#[derive(Debug, Clone, Default)]
struct ExternalFeedbackClosureRenderWriteAudit {
    attempted: bool,
    written: bool,
    cross_staff_beam_groups: usize,
    beam_rows_written: usize,
    beam_level1_rows: usize,
    beam_level2_rows: usize,
    note_count_before: usize,
    note_count_after: usize,
    parse_after_ok: bool,
    sounding_stable: bool,
    verdict: String,
    done_fragment: String,
    pipeline_line: String,
}






fn lower_staff_ledger_pressure(midi: i64, clef: &str) -> i64 {
    match clef {
        "G" => {
            if midi < 64 {
                (64 - midi + 1) / 2
            } else if midi > 77 {
                (midi - 77 + 1) / 2
            } else {
                0
            }
        }
        "F" => {
            if midi < 43 {
                (43 - midi + 1) / 2
            } else if midi > 57 {
                (midi - 57 + 1) / 2
            } else {
                0
            }
        }
        _ => 0,
    }
}
/// Final export normalization. A wrapper only: it returns the PRESERVING E055
/// receipt of its inner owner on every path, including the validation error.
fn normalize_final_export_xml(
    xml: &mut String,
) -> (anyhow::Result<String>, Option<master_state_identity::PreservingTransformationReceipt>) {
    let (removed_words, receipt) = remove_generated_octave_word_labels(xml);
    let stats = match validate_octave_shift_direction_semantics(xml)
        .context("OctaveShiftExportGuard: finale Oktavlinien sind semantisch widersprüchlich")
    {
        Ok(stats) => stats,
        Err(error) => return (Err(error), receipt),
    };
    (Ok(format!(
        "OctaveShiftExportGuard: kanonische Final-XML geprüft; {} alte Oktav-Textlabel entfernt; Registernotations-Placement unverändert; {} Oktavrichtungen geprüft (above={}, below={}, continue={}).",
        removed_words,
        stats.checked_shifts,
        stats.above_shifts,
        stats.below_shifts,
        stats.continue_shifts
    )), receipt)
}
fn purge_previous_report_outputs(source: &source::SourcePlan) -> anyhow::Result<()> {
    let outputs: Vec<PathBuf> = vec![
        source.musicxml_out.clone(),
        source.render_musicxml_out.clone(),
        source.pdf_out.clone(),
        source.report_out.clone(),
    ];
    for path in outputs {
        match std::fs::remove_file(&path) {
            Ok(_) => {},
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {},
            Err(err) => {
                return Err(anyhow!("Konnte alte Ausgabedatei nicht entfernen: {}: {}", path.display(), err));
            }
        }
    }
    Ok(())
}
fn validate_musicxml_export_semantics(musicxml_out: &Path) -> anyhow::Result<String> {
    let musicxml = std::fs::read_to_string(musicxml_out)
        .with_context(|| format!("Konnte MusicXML für ExportGuard nicht lesen: {}", musicxml_out.display()))?;
    validate_musicxml_export_semantics_from_xml(musicxml_out, &musicxml)
}
fn validate_musicxml_export_semantics_from_xml(musicxml_out: &Path, musicxml: &str) -> anyhow::Result<String> {
    let stats = validate_final_layout_aware_octave_shift_direction_semantics(musicxml)
        .context("EXPORT_INCONSISTENT: Oktavlinien-Semantik in der geschriebenen Final-XML ist ungültig")?;
    Ok(format!(
        "MusicXmlExportGuard: XML-Datei exportfähig geschrieben; {} Oktavlinien semantisch geprüft; Continues {} davon inner-systemisch sichtbar {} und an finalem Layout-Neustart {}; Quelle Speicher-XML statt Datei-ReRead; Ziel {} ; MXL-Standardausgabe entfällt.",
        stats.checked_shifts,
        stats.continue_shifts,
        stats.visible_continue_recues,
        stats.layout_restart_continues,
        musicxml_out.display()
    ))
}
/// Sealed mint capability of `remove_generated_octave_word_labels`
/// (PRESERVING E055).
pub(crate) struct OctaveWordLabelPreservingMint {
    _sealed: (),
}

fn remove_generated_octave_word_labels(xml: &mut String) -> (usize, Option<PreservingTransformationReceipt>) {
    let mut removed = 0_usize;
    let mut out = String::with_capacity(xml.len());
    let mut scan = 0_usize;

    while let Some(rel_start) = xml[scan..].find("<direction") {
        let start = scan + rel_start;
        let Some(close_rel) = xml[start..].find("</direction>") else { break; };
        let end = start + close_rel + "</direction>".len();
        out.push_str(&xml[scan..start]);
        let mut block = xml[start..end].to_string();
        let (cleaned, count) = remove_octave_words_from_direction_block(&block);
        block = cleaned;
        removed += count;
        if count == 0 || !all_direction_types_empty(&block) {
            out.push_str(&block);
        }
        scan = end;
    }
    out.push_str(&xml[scan..]);
    // PRESERVING receipt E055, minted by this owner only if the conserved
    // musical content projection is unchanged (directions are presentation).
    let receipt = master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
        OctaveWordLabelPreservingMint { _sealed: () },
        xml,
        &out,
    );
    *xml = out;
    (removed, receipt)
}

fn all_direction_types_empty(block: &str) -> bool {
    let mut found = false;
    let mut scan = 0_usize;
    while let Some(rel_start) = block[scan..].find("<direction-type>") {
        found = true;
        let start = scan + rel_start + "<direction-type>".len();
        let Some(close_rel) = block[start..].find("</direction-type>") else { return false; };
        let end = start + close_rel;
        if !block[start..end].trim().is_empty() {
            return false;
        }
        scan = end + "</direction-type>".len();
    }
    found
}

fn direction_block_contains_continue_octave_shift(block: &str) -> bool {
    let mut scan = 0_usize;
    while let Some(rel_start) = block[scan..].find("<octave-shift") {
        let start = scan + rel_start;
        let Some(tag_end_rel) = block[start..].find('>') else {
            return false;
        };
        let tag_end = start + tag_end_rel + 1;
        let shift_tag = &block[start..tag_end];
        if extract_attr_from_opening_tag(shift_tag, "type").as_deref() == Some("continue") {
            return true;
        }
        scan = tag_end;
    }
    false
}

fn direction_words_texts(block: &str) -> Vec<String> {
    let mut texts = Vec::new();
    let mut scan = 0_usize;
    while let Some(rel_start) = block[scan..].find("<words") {
        let start = scan + rel_start;
        let Some(tag_end_rel) = block[start..].find('>') else {
            break;
        };
        let text_start = start + tag_end_rel + 1;
        let Some(close_rel) = block[text_start..].find("</words>") else {
            break;
        };
        let text_end = text_start + close_rel;
        let end = text_end + "</words>".len();
        let text = block[text_start..text_end].trim();
        if !text.is_empty() {
            texts.push(text.to_string());
        }
        scan = end;
    }
    texts
}

fn remove_octave_words_from_direction_block(block: &str) -> (String, usize) {
    let preserve_continue_recue = direction_block_contains_continue_octave_shift(block);
    let mut removed = 0_usize;
    let mut out = String::with_capacity(block.len());
    let mut scan = 0_usize;

    while let Some(rel_start) = block[scan..].find("<words") {
        let start = scan + rel_start;
        let Some(tag_end_rel) = block[start..].find('>') else { break; };
        let text_start = start + tag_end_rel + 1;
        let Some(close_rel) = block[text_start..].find("</words>") else { break; };
        let text_end = text_start + close_rel;
        let end = text_end + "</words>".len();
        let text = block[text_start..text_end].trim();
        if matches!(text, "8va" | "15ma" | "8vb" | "15mb") && !preserve_continue_recue {
            out.push_str(&block[scan..start]);
            removed += 1;
        } else {
            out.push_str(&block[scan..end]);
        }
        scan = end;
    }
    out.push_str(&block[scan..]);
    (out, removed)
}

fn validate_octave_shift_direction_semantics(xml: &str) -> anyhow::Result<OctaveExportStats> {
    validate_octave_shift_direction_semantics_with_layout(xml, false)
}


fn validate_final_layout_aware_octave_shift_direction_semantics(
    xml: &str,
) -> anyhow::Result<OctaveExportStats> {
    validate_octave_shift_direction_semantics_with_layout(xml, true)
}

fn validate_octave_shift_direction_semantics_with_layout(
    xml: &str,
    final_layout_aware: bool,
) -> anyhow::Result<OctaveExportStats> {
    for word in ["<words>8va</words>", "<words>15ma</words>", "<words>8vb</words>", "<words>15mb</words>"] {
        if xml.contains(word) {
            return Err(anyhow!("alte Oktavierungs-Textangabe gefunden: {word}"));
        }
    }

    let mut stats = OctaveExportStats {
        checked_shifts: 0,
        above_shifts: 0,
        below_shifts: 0,
        continue_shifts: 0,
        visible_continue_recues: 0,
        layout_restart_continues: 0,
    };
    let mut layout_restart_direction_starts = BTreeSet::<usize>::new();
    if final_layout_aware {
        let dokument = roxmltree::Document::parse(xml)
            .map_err(|fehler| anyhow!("OctaveShiftExportGuard: finale XML ungueltig: {fehler}"))?;
        for takt in dokument
            .descendants()
            .filter(|knoten| knoten.is_element() && knoten.tag_name().name() == "measure")
        {
            let layout_neustart = takt.children().any(|knoten| {
                knoten.is_element()
                    && knoten.tag_name().name() == "print"
                    && (knoten.attribute("new-system") == Some("yes")
                        || knoten.attribute("new-page") == Some("yes"))
            });
            if layout_neustart {
                for richtung in takt.children().filter(|knoten| {
                    knoten.is_element() && knoten.tag_name().name() == "direction"
                }) {
                    let staff = richtung
                        .children()
                        .find(|kind| kind.is_element() && kind.tag_name().name() == "staff")
                        .and_then(|kind| kind.text())
                        .map(str::trim);
                    let erstes_gepitchtes_ereignis_dieses_staffs = staff.and_then(|staff| {
                        takt.children()
                            .filter(|knoten| {
                                knoten.is_element() && knoten.tag_name().name() == "note"
                            })
                            .find(|note| {
                                let hat_pitch = note.children().any(|kind| {
                                    kind.is_element() && kind.tag_name().name() == "pitch"
                                });
                                let note_staff = note
                                    .children()
                                    .find(|kind| {
                                        kind.is_element() && kind.tag_name().name() == "staff"
                                    })
                                    .and_then(|kind| kind.text())
                                    .map(str::trim);
                                hat_pitch && note_staff == Some(staff)
                            })
                            .map(|note| note.range().start)
                    });
                    if erstes_gepitchtes_ereignis_dieses_staffs
                        .is_some_and(|erste_note| richtung.range().start < erste_note)
                    {
                        layout_restart_direction_starts.insert(richtung.range().start);
                    }
                }
            }
        }
    }
    let mut scan = 0_usize;
    let mut direction_ordinal = 0usize;
    while let Some(rel_start) = xml[scan..].find("<direction") {
        let current_direction_ordinal = direction_ordinal;
        direction_ordinal += 1;
        let start = scan + rel_start;
        let Some(close_rel) = xml[start..].find("</direction>") else { break; };
        let end = start + close_rel + "</direction>".len();
        let block = &xml[start..end];
        let layout_neustart = final_layout_aware
            && layout_restart_direction_starts.contains(&start);
        scan = end;

        if !block.contains("<octave-shift") {
            continue;
        }
        let placement = extract_attr_from_opening_tag(block, "placement");
        let mut shift_scan = 0_usize;
        while let Some(shift_rel) = block[shift_scan..].find("<octave-shift") {
            let shift_start = shift_scan + shift_rel;
            let Some(shift_end_rel) = block[shift_start..].find('>') else { break; };
            let shift_tag = &block[shift_start..shift_start + shift_end_rel + 1];
            shift_scan = shift_start + shift_end_rel + 1;

            let shift_type = extract_attr_from_opening_tag(shift_tag, "type")
                .ok_or_else(|| anyhow!("octave-shift ohne type-Attribut"))?;
            if extract_attr_from_opening_tag(shift_tag, "default-y").is_some() {
                return Err(anyhow!(
                    "OCTAVE_GEOMETRY_AUTHORITY_CONFLICT: Registernotations-Oktavlinie enthaelt unerlaubtes default-y"
                ));
            }
            if shift_type == "stop" {
                continue;
            }
            let size = extract_attr_from_opening_tag(shift_tag, "size")
                .ok_or_else(|| anyhow!("octave-shift ohne size-Attribut"))?;
            if !matches!(size.as_str(), "8" | "15") {
                return Err(anyhow!("octave-shift mit unerwarteter size={size}"));
            }

            if shift_type == "continue" {
                let expected_recue = match (size.as_str(), placement.as_deref()) {
                    ("8", Some("above")) => "8va",
                    ("15", Some("above")) => "15ma",
                    ("8", Some("below")) => "8vb",
                    ("15", Some("below")) => "15mb",
                    _ => {
                        return Err(anyhow!(
                            "octave-shift continue ohne eindeutige Darstellung fuer sichtbaren Re-Cue"
                        ));
                    }
                };
                let words = direction_words_texts(block);
                if layout_neustart {
                    if !words.is_empty() {
                        return Err(anyhow!(
                            "OCTAVE_CONTINUE_RECUE_DUPLICATES_LAYOUT_RESTART: octave-shift continue an finalem System-/Seitenneustart darf keinen zusaetzlichen Words-Re-Cue tragen"
                        ));
                    }
                    stats.layout_restart_continues += 1;
                } else if final_layout_aware && words.is_empty()
                    && crate::registernotation::reader_policy::omission_is_adopted(xml, current_direction_ordinal) {
                    // The unchanged baseline rule is replaced only for this exact
                    // adopted E007 post and its transported continuation ordinal.
                } else {
                    if words.len() != 1
                        || words.first().map(String::as_str) != Some(expected_recue)
                    {
                        return Err(anyhow!(
                            "OCTAVE_CONTINUE_RECUE_MISSING: inner-systemischer octave-shift continue besitzt keinen exakt passenden sichtbaren Re-Cue"
                        ));
                    }
                    stats.visible_continue_recues += 1;
                }
                match placement.as_deref() {
                    Some("above") => stats.above_shifts += 1,
                    Some("below") => stats.below_shifts += 1,
                    _ => {
                        return Err(anyhow!(
                            "octave-shift continue ohne eindeutiges placement above/below"
                        ));
                    }
                }
                stats.checked_shifts += 1;
                stats.continue_shifts += 1;
                continue;
            }
            if !matches!(shift_type.as_str(), "up" | "down") {
                return Err(anyhow!("octave-shift mit unerwartetem type={shift_type}"));
            }
            match (shift_type.as_str(), placement.as_deref()) {
                ("down", Some("above")) => {
                    stats.checked_shifts += 1;
                    stats.above_shifts += 1;
                }
                ("up", Some("below")) => {
                    stats.checked_shifts += 1;
                    stats.below_shifts += 1;
                }
                ("down", Some("below")) | ("up", Some("above")) => {
                    return Err(anyhow!(
                        "OCTAVE_PLACEMENT_AMBIGUOUS: Oktavierungsart und sichtbare Seite widersprechen einander"
                    ));
                }
                _ => return Err(anyhow!("octave-shift ohne eindeutiges placement above/below")),
            }
        }
    }
    Ok(stats)
}

#[cfg(test)]
mod registernotations_export_guard_tests {
    use super::{
        normalize_final_export_xml,
        validate_final_layout_aware_octave_shift_direction_semantics,
        validate_octave_shift_direction_semantics,
    };

    #[test]
    fn untere_registernotation_bleibt_unten_und_ohne_default_y() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><direction placement=\"below\"><direction-type><octave-shift type=\"up\" size=\"8\" number=\"1\"/></direction-type><staff>2</staff></direction></measure></part></score-partwise>".to_string();
        let vorher = xml.clone();
        normalize_final_export_xml(&mut xml).0.expect("untere Registernotation muss exportierbar bleiben");
        assert_eq!(xml, vorher);
        assert!(xml.contains("placement=\"below\""));
        assert!(!xml.contains("default-y="));
    }

    #[test]
    fn exportguard_akzeptiert_musicxml40_continue_mit_sichtbarem_recue() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><direction placement=\"above\"><direction-type><octave-shift type=\"down\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction><note><pitch><step>C</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><forward><duration>24</duration></forward><direction placement=\"above\"><direction-type><words font-style=\"italic\">8va</words></direction-type><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><direction placement=\"above\"><direction-type><octave-shift type=\"stop\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction></measure></part></score-partwise>".to_string();
        let summary = normalize_final_export_xml(&mut xml).0
            .expect("autorisierter Continue-Re-Cue muss den Final-Export-Cleanup ueberleben");
        assert!(summary.contains("0 alte Oktav-Textlabel entfernt"));
        assert!(xml.contains("<words font-style=\"italic\">8va</words>"));
        let stats = validate_octave_shift_direction_semantics(&xml)
            .expect("MusicXML 4.0 continue mit sichtbarem Re-Cue muss exportierbar sein");
        assert_eq!(stats.checked_shifts, 2);
        assert_eq!(stats.continue_shifts, 1);
        assert_eq!(stats.above_shifts, 2);
        assert_eq!(stats.below_shifts, 0);
    }

    #[test]
    fn exportguard_verweigert_continue_wenn_cleanup_keinen_sichtbaren_recue_hinterlaesst() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><direction placement=\"above\"><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction></measure></part></score-partwise>";
        let fehler = validate_octave_shift_direction_semantics(xml)
            .expect_err("Continue ohne sichtbaren Re-Cue muss final fail-closed bleiben");
        assert!(fehler.to_string().contains("OCTAVE_CONTINUE_RECUE_MISSING"));
    }

    #[test]
    fn finaler_exportguard_akzeptiert_layout_neustart_continue_ohne_words_recue() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><print new-system=\"yes\"/><note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><staff>2</staff></note><direction placement=\"above\"><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let stats = validate_final_layout_aware_octave_shift_direction_semantics(xml)
            .expect("finaler Layout-Neustart muss continue ohne zusaetzlichen Words-Re-Cue akzeptieren");
        assert_eq!(stats.continue_shifts, 1);
        assert_eq!(stats.visible_continue_recues, 0);
        assert_eq!(stats.layout_restart_continues, 1);
    }

    #[test]
    fn finaler_exportguard_behaelt_spaeteren_inneren_recue_im_restart_takt() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><print new-system=\"yes\"/><note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><direction placement=\"above\"><direction-type><words font-style=\"italic\">8va</words></direction-type><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction></measure></part></score-partwise>";
        let stats = validate_final_layout_aware_octave_shift_direction_semantics(xml)
            .expect("spaeterer innerer Continue im Restart-Takt braucht weiterhin seinen sichtbaren Re-Cue");
        assert_eq!(stats.continue_shifts, 1);
        assert_eq!(stats.visible_continue_recues, 1);
        assert_eq!(stats.layout_restart_continues, 0);
    }

    #[test]
    fn finaler_exportguard_verweigert_doppelten_recue_am_layout_neustart() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><print new-page=\"yes\"/><direction placement=\"above\"><direction-type><words font-style=\"italic\">8va</words></direction-type><direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction><note><pitch><step>D</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let fehler = validate_final_layout_aware_octave_shift_direction_semantics(xml)
            .expect_err("finaler Layout-Neustart darf keinen zusaetzlichen Words-Re-Cue tragen");
        assert!(fehler
            .to_string()
            .contains("OCTAVE_CONTINUE_RECUE_DUPLICATES_LAYOUT_RESTART"));
    }

    #[test]
    fn exportguard_verweigert_spaete_default_y_oktavgeometrie() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><direction placement=\"above\"><direction-type><octave-shift type=\"down\" size=\"8\" number=\"1\" default-y=\"54\"/></direction-type><staff>1</staff></direction></measure></part></score-partwise>";
        let fehler = validate_octave_shift_direction_semantics(xml)
            .expect_err("spaete Oktav-default-y-Geometrie muss fail-closed bleiben");
        assert!(fehler.to_string().contains("OCTAVE_GEOMETRY_AUTHORITY_CONFLICT"));
    }
}

fn extract_attr_from_opening_tag(fragment: &str, attr: &str) -> Option<String> {
    let end = fragment.find('>').unwrap_or(fragment.len());
    let opening = &fragment[..end];
    let marker = format!("{attr}=\"");
    let start = opening.find(&marker)? + marker.len();
    let stop = start + opening[start..].find('"')?;
    Some(opening[start..stop].to_string())
}

fn collect_measure_note_fragments(measure: &str) -> Vec<String> {
    let mut notes = Vec::new();
    let mut scan = 0_usize;
    while let Some(rel_start) = measure[scan..].find("<note") {
        let start = scan + rel_start;
        let Some(close_rel) = measure[start..].find("</note>") else { break; };
        let end = start + close_rel + "</note>".len();
        notes.push(measure[start..end].to_string());
        scan = end;
    }
    notes
}

fn midi_from_note_fragment(note: &str) -> Option<i32> {
    if note.contains("<rest") || !note.contains("<pitch>") {
        return None;
    }
    let step = extract_first_text_from_fragment(note, "step")?;
    let alter = extract_first_text_from_fragment(note, "alter")
        .and_then(|value| value.trim().parse::<i32>().ok())
        .unwrap_or(0);
    let octave = extract_first_text_from_fragment(note, "octave")?.trim().parse::<i32>().ok()?;
    midi_for_pitch(&step, alter, octave)
}

fn midi_for_pitch(step: &str, alter: i32, octave: i32) -> Option<i32> {
    let base = match step.trim() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return None,
    };
    Some((octave + 1) * 12 + base + alter)
}

fn extract_first_text_from_fragment(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some(text[start..end].trim().to_string())
}

/// Expression polish. A wrapper only: it returns the ADDITIVE E036 receipts
/// of its direction insertions in application order.
fn apply_expression_polish(
    xml: &mut String,
    estimated_bpm: Option<f64>,
) -> (String, master_state_identity::OwnerReceipts) {
    let mut inserted = 0_usize;
    let mut receipts = master_state_identity::OwnerReceipts::default();

    let Some(first_measure) =
        crate::structural_write_authority::first_structural_measure_locator(xml)
    else {
        return ("ExpressionPolish v0.1.162: keine Takte gefunden; keine Tempo-/Spiel-/Dynamikangaben gesetzt; keine Pedalzeichen eingefügt.".to_string(), receipts);
    };

    // v0.1.159: Startseiten-Luxuspolitur vor der Statusprüfung. Tempo und
    // non legato bleiben reine MusicXML-Directions, bekommen aber mehr Abstand
    // zum ersten System. Es ändert keine Noten, Dauern, Stimmen, Staffs,
    // Oktavierungen, Schlüssel oder Solver-Auswahl.
    let tempo_bpm = estimated_bpm
        .map(|bpm| bpm.round().clamp(40.0, 220.0) as u32)
        .filter(|bpm| *bpm > 0);
    if let Some(bpm) = tempo_bpm {
        let (tempo_inserted, tempo_receipts) =
            insert_tempo_words_direction_into_measure(xml, &first_measure, bpm, "1");
        inserted += tempo_inserted;
        receipts.append(tempo_receipts);
    }

    let (performance_inserted, performance_receipt) =
        insert_performance_words_direction_into_measure(xml, &first_measure, "non legato", "1");
    inserted += performance_inserted;
    receipts.push_additive(performance_receipt);

    // Generische, sparsame Dynamikpunkte: Anfang, etwa 40%, etwa 60%, etwa 75%
    // und ein später Schlussbereich. Die Positionen werden aus der Partiturstruktur
    // abgeleitet, nicht aus festen Taktnummern.
    let mut dynamic_targets = vec![(first_measure, "mf")];
    for (fraction, dynamic) in [(0.40, "mp"), (0.60, "f"), (0.75, "mf"), (0.84, "f")] {
        if let Some(locator) =
            crate::structural_write_authority::fractional_structural_measure_locator(xml, fraction)
        {
            dynamic_targets.push((locator, dynamic));
        }
    }

    dynamic_targets.sort_by_key(|(measure, _)| measure.measure_number());
    dynamic_targets.dedup_by(|left, right| {
        left.0.measure_number() == right.0.measure_number() && left.1 == right.1
    });
    for (measure, dyn_mark) in dynamic_targets {
        let (dynamic_inserted, dynamic_receipt) =
            insert_dynamic_direction_into_measure(xml, &measure, dyn_mark, "1");
        inserted += dynamic_inserted;
        receipts.push_additive(dynamic_receipt);
    }

    let tempo_fragment = tempo_bpm
        .map_or_else(
            || "Tempo nicht gesetzt, weil kein plausibler WAV-Tempo-Schätzwert vorlag".to_string(),
            |bpm| format!("Tempo als echtes Metronomzeichen sichtbar: Viertel={bpm} aus WAV-Signatur"),
        );
    (format!(
        "ExpressionPolish v0.1.162: {inserted} portable Tempo-/Spiel-/Dynamikangaben generisch aus der Partiturstruktur gesetzt; {tempo_fragment}; Startseite luxuriöser gesetzt: Tempo und non legato haben mehr Abstand zum ersten System, Editor-/Versionsmetadata bleiben aus dem Staff-Text entfernt; keine festen Taktnummern, keine Pedalzeichen, keine Noten-/Dauer-/Staff-/Oktavierungsänderung."
    ), receipts)
}


fn insert_performance_words_direction_into_measure(
    xml: &mut String,
    measure: &crate::structural_write_authority::StructuralMeasureLocator,
    words: &str,
    staff: &str,
) -> (usize, Option<master_state_identity::AdditiveTransformationReceipt>) {
    let direction = format!(
        "      <direction placement=\"above\">\n        <direction-type>\n          <words default-x=\"0\" default-y=\"58\" font-size=\"10\" font-style=\"italic\">{words}</words>\n        </direction-type>\n        <staff>{staff}</staff>\n      </direction>\n"
    );
    insert_direction_into_measure(xml, measure, &direction)
}

fn insert_tempo_words_direction_into_measure(
    xml: &mut String,
    measure: &crate::structural_write_authority::StructuralMeasureLocator,
    bpm: u32,
    staff: &str,
) -> (usize, master_state_identity::OwnerReceipts) {
    let measure_number = measure.measure_number();
    let (equivalent_visible_tempo_present, equivalent_playback_tempo_present) = {
        let mut visible_found = false;
        let mut playback_found = false;
        let mut scan = 0_usize;
        while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
            scan = end;
            if number != measure_number {
                continue;
            }
            let measure = &xml[start..end];
            let mut direction_scan = 0_usize;
            while let Some(relative_start) = measure[direction_scan..].find("<direction") {
                let direction_start = direction_scan + relative_start;
                let Some(relative_end) = measure[direction_start..].find("</direction>") else {
                    break;
                };
                let direction_end = direction_start + relative_end + "</direction>".len();
                let direction = &measure[direction_start..direction_end];
                direction_scan = direction_end;

                let hidden = direction.contains("print-object=\"no\"");
                let equivalent_metronome = direction.contains("<metronome")
                    && !direction.contains("<beat-unit-dot")
                    && extract_first_text_from_fragment(direction, "beat-unit").as_deref()
                        == Some("quarter")
                    && extract_first_text_from_fragment(direction, "per-minute")
                        .and_then(|value| value.parse::<f64>().ok())
                        .is_some_and(|value| (value - f64::from(bpm)).abs() < 0.001);
                let visible_tempo_candidate = roxmltree::Document::parse(direction)
                    .ok()
                    .and_then(|document| {
                        document.descendants().find(|node| node.has_tag_name("words"))
                            .and_then(|node| node.text()).map(str::to_string)
                    });
                let equivalent_words = visible_tempo_candidate
                    .filter(|value| value.contains("&#x2669;") || value.contains('♩'))
                    .and_then(|value| value.rsplit_once('=').map(|(_, rhs)| rhs.trim().to_string()))
                    .and_then(|value| value.parse::<f64>().ok())
                    .is_some_and(|value| (value - f64::from(bpm)).abs() < 0.001);
                let equivalent_sound = direction
                    .find("<sound")
                    .and_then(|relative| direction.get(relative..))
                    .and_then(|sound| extract_attr_from_opening_tag(sound, "tempo"))
                    .and_then(|value| value.parse::<f64>().ok())
                    .is_some_and(|value| (value - f64::from(bpm)).abs() < 0.001);
                let equivalent_staff = extract_first_text_from_fragment(direction, "staff")
                    .is_none_or(|value| value == staff);
                if equivalent_staff {
                    visible_found |= !hidden && (equivalent_metronome || equivalent_words);
                    playback_found |= equivalent_metronome || equivalent_sound;
                }
                if visible_found && playback_found {
                    break;
                }
            }
            break;
        }
        (visible_found, playback_found)
    };

    let mut receipts = master_state_identity::OwnerReceipts::default();
    if equivalent_visible_tempo_present && equivalent_playback_tempo_present {
        return (0, receipts);
    }
    // The accepted r1268 causal class binds one initial tempo event. When both
    // representations are absent, one ADDITIVE direction owns both. Existing
    // partial representations retain their bytes and gain only the missing half.
    let visible = if equivalent_visible_tempo_present {
        "          <words></words>".to_string()
    } else {
        format!("          <words default-x=\"0\" default-y=\"88\" font-size=\"11\" font-weight=\"bold\">&#x2669; = {bpm}</words>")
    };
    let playback = if equivalent_playback_tempo_present {
        String::new()
    } else {
        format!("        <sound tempo=\"{bpm}\"/>\n")
    };
    let marker = if equivalent_visible_tempo_present {
        "etuschlopf_playback_tempo"
    } else {
        "etuschlopf_visible_tempo"
    };
    let direction = format!(
        "      <!-- {marker} -->\n      <direction placement=\"above\">\n        <direction-type>\n{visible}\n        </direction-type>\n        <staff>{staff}</staff>\n{playback}      </direction>\n"
    );
    let (inserted, receipt) = insert_direction_into_measure(xml, measure, &direction);
    receipts.push_additive(receipt);
    (inserted, receipts)
}

#[cfg(test)]
mod initial_tempo_semantic_identity_tests {
    use super::insert_tempo_words_direction_into_measure;

    fn insert_tempo(xml: &mut String, bpm: u32, staff: &str) -> usize {
        let locator = crate::structural_write_authority::first_structural_measure_locator(xml).unwrap();
        insert_tempo_words_direction_into_measure(xml, &locator, bpm, staff).0
    }

    #[test]
    fn equivalent_visible_metronome_already_owns_visibility_and_playback() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<direction placement=\"above\"><direction-type><metronome default-y=\"72\"><beat-unit>quarter</beat-unit><per-minute>126.0</per-minute></metronome></direction-type><staff>1</staff></direction>\n<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>".to_string();
        let before = xml.clone();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 0);
        assert_eq!(xml, before);
        assert_eq!(xml.matches("<metronome").count(), 1);
    }

    #[test]
    fn generated_start_tempo_uses_visible_staff_text_and_sound_only_playback() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n</measure></part></score-partwise>".to_string();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 1);
        assert_eq!(xml.matches("etuschlopf_visible_tempo").count(), 1);
        assert_eq!(xml.matches("etuschlopf_playback_tempo").count(), 0);
        assert_eq!(xml.matches("<words").count(), 1);
        assert_eq!(xml.matches("<metronome").count(), 0);
        assert_eq!(xml.matches("<sound tempo=\"126\"/>").count(), 1);
        assert_eq!(xml.matches("print-object=\"no\"").count(), 0);
    }

    #[test]
    fn equivalent_generated_pair_is_not_materialized_twice() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<!-- etuschlopf_visible_tempo --><direction placement=\"above\"><direction-type><words>&#x2669; = 126.0</words></direction-type><staff>1</staff></direction>\n<!-- etuschlopf_playback_tempo --><direction placement=\"above\"><direction-type><words></words></direction-type><staff>1</staff><sound tempo=\"126\"/></direction>\n</measure></part></score-partwise>".to_string();
        let before = xml.clone();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 0);
        assert_eq!(xml, before);
    }

    #[test]
    fn visible_words_without_playback_gain_only_sound_carrier() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<direction placement=\"above\"><direction-type><words>&#x2669; = 126</words></direction-type><staff>1</staff></direction>\n</measure></part></score-partwise>".to_string();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 1);
        assert_eq!(xml.matches("&#x2669; = 126").count(), 1);
        assert_eq!(xml.matches("<sound tempo=\"126\"/>").count(), 1);
        assert_eq!(xml.matches("<metronome").count(), 0);
    }

    #[test]
    fn distinct_visible_tempo_remains_legitimate() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<direction placement=\"above\"><direction-type><metronome><beat-unit>quarter</beat-unit><per-minute>121</per-minute></metronome></direction-type><staff>1</staff></direction>\n</measure></part></score-partwise>".to_string();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 1);
        assert_eq!(xml.matches("<metronome").count(), 1);
        assert_eq!(xml.matches("etuschlopf_visible_tempo").count(), 1);
        assert_eq!(xml.matches("<sound tempo=\"126\"/>").count(), 1);
        assert!(xml.contains("<per-minute>121</per-minute>"));
        assert!(xml.contains("&#x2669; = 126"));
    }

    #[test]
    fn dotted_quarter_is_not_the_same_tempo_identity() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<direction placement=\"above\"><direction-type><metronome><beat-unit>quarter</beat-unit><beat-unit-dot/><per-minute>126</per-minute></metronome></direction-type><staff>1</staff></direction>\n</measure></part></score-partwise>".to_string();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 1);
        assert_eq!(xml.matches("<metronome").count(), 1);
        assert_eq!(xml.matches("etuschlopf_visible_tempo").count(), 1);
        assert_eq!(xml.matches("<sound tempo=\"126\"/>").count(), 1);
    }

    #[test]
    fn sound_only_playback_suppresses_only_the_playback_duplicate() {
        let mut xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes></attributes>\n<direction placement=\"above\"><direction-type><words></words></direction-type><staff>1</staff><sound tempo=\"126\"/></direction>\n</measure></part></score-partwise>".to_string();
        assert_eq!(insert_tempo(&mut xml, 126, "1"), 1);
        assert_eq!(xml.matches("<sound tempo=\"126\"/>").count(), 1);
        assert_eq!(xml.matches("etuschlopf_visible_tempo").count(), 1);
        assert_eq!(xml.matches("<metronome").count(), 0);
    }
}

#[cfg(test)]
#[path = "../../tests/fixtures/synthetic/wav_metronome_construction.rs"]
mod wav_metronome_construction_tests;

fn insert_dynamic_direction_into_measure(
    xml: &mut String,
    measure: &crate::structural_write_authority::StructuralMeasureLocator,
    dynamic: &str,
    staff: &str,
) -> (usize, Option<master_state_identity::AdditiveTransformationReceipt>) {
    let direction = format!(
        "      <direction placement=\"below\">\n        <direction-type>\n          <dynamics>\n            <{dynamic}/>\n          </dynamics>\n        </direction-type>\n        <staff>{staff}</staff>\n      </direction>\n"
    );
    insert_direction_into_measure(xml, measure, &direction)
}


fn insert_direction_into_measure(
    xml: &mut String,
    measure: &crate::structural_write_authority::StructuralMeasureLocator,
    direction: &str,
) -> (usize, Option<master_state_identity::AdditiveTransformationReceipt>) {
    crate::structural_write_authority::insert_direction_into_measure_authorized(xml, measure, direction)
}


/// Final notation polish guard. A wrapper only: it returns the receipts of its
/// inner owners (#5 note visibility, #6 allowed range removal) in application
/// order and mints none of its own.
fn apply_final_notation_polish_guard(xml: &mut String) -> master_state_identity::OwnerReceipts {
    let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.append(hide_exact_duplicate_visible_noteheads(xml).1);
    receipts.append(hide_redundant_visible_staff_rests(xml).1);

    for _ in 0..4 {
        let ((removed_slurs, removed_ties), endpoint_receipts) =
            remove_orphan_slur_and_tie_endpoints(xml);
        receipts.append(endpoint_receipts);
        let (rest_only_clefs, rest_only_receipt) = prune_rest_only_staff2_clef_noops(xml);
        receipts.push_exact(rest_only_receipt);
        let ((empty_clefs, trailing_clefs, redundant_clefs, advance_clefs), redundant_receipt) =
            prune_redundant_lower_staff_clefs(xml);
        receipts.push_exact(redundant_receipt);
        if removed_slurs
            + removed_ties
            + rest_only_clefs
            + empty_clefs
            + trailing_clefs
            + redundant_clefs
            + advance_clefs
            == 0
        {
            break;
        }
    }
    receipts
}

pub(crate) struct RenderRow19BlockMint { _private: () }

pub(crate) struct RenderRow54BlockMint { _private: () }

#[cfg(test)]
#[path = "../../tests/unit/selected_master_notation_journal.rs"]
mod selected_master_notation_journal_tests;

#[cfg(test)]
#[path = "../../tests/unit/notation_endpoint_cleanup_adoption.rs"]
mod notation_endpoint_cleanup_adoption_tests;

#[cfg(test)]
#[path = "../../tests/unit/preserving_representation_receipt_custody.rs"]
pub(crate) mod preserving_representation_receipt_custody_tests;

#[cfg(test)]
#[path = "../../tests/unit/register_tie_boundary_owner.rs"]
mod register_tie_boundary_owner;

#[cfg(test)]
mod same_beat_existing_beam_edge_protection_tests {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/same_beat_existing_beam_edge_protection.rs"));
}

#[cfg(test)]
mod bridge_frozen_voice_compatibility_tests {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/bridge_frozen_voice_compatibility.rs"));
}

fn append_external_feedback_closure_render_write_section(
    out: &mut String,
    audit: &ExternalFeedbackClosureRenderWriteAudit,
) {
    out.push_static_section_header_v795("\nEXTERNAL-FEEDBACK-CLOSURE-RENDER-WRITE", "----------------------------------------");
    out.push_format_args(format_args!(
        "CommittedCrossStaffBeamProjection: Attempted {}; Written {}; CrossStaffBeamGroups {}; BeamRows {}; BeamLevel1Rows {}; BeamLevel2Rows {}; NoteCountBeforeAfter {}/{}; ParseAfter {}; SoundingStable {}; Urteil {}.\n",
        yes_no(audit.attempted), yes_no(audit.written), audit.cross_staff_beam_groups,
        audit.beam_rows_written, audit.beam_level1_rows, audit.beam_level2_rows,
        audit.note_count_before, audit.note_count_after, yes_no(audit.parse_after_ok),
        yes_no(audit.sounding_stable), audit.verdict,
    ));
}

#[cfg(test)]
mod behind_bars_beam_single_owner_tests {
    use super::{
        structural_metric_beam_hierarchy_plan,
        structural_beam_group_needs_owned_plan_normalization,
        partial_beat_rewrite_owned_beam_levels,
        PartialBeatNoteInfo,
        PartialBeatRawNote,
    };
    fn info(index: usize, start: i64) -> PartialBeatNoteInfo {
        PartialBeatNoteInfo {
            index,
            start,
            duration: 24,
            staff: "1".to_string(),
            voice: "1".to_string(),
            note_type: "16th".to_string(),
            is_rest: false,
            is_chord: false,
            is_hidden: false,
            existing_beams: 2,
        }
    }

    fn typed_info(
        index: usize,
        start: i64,
        duration: i64,
        note_type: &str,
    ) -> PartialBeatNoteInfo {
        PartialBeatNoteInfo {
            index,
            start,
            duration,
            staff: "1".to_string(),
            voice: "1".to_string(),
            note_type: note_type.to_string(),
            is_rest: false,
            is_chord: false,
            is_hidden: false,
            existing_beams: 0,
        }
    }

    fn upper_plan_rows(
        infos: &[PartialBeatNoteInfo],
    ) -> Vec<(usize, u8, &'static str)> {
        let group = infos.iter().collect::<Vec<_>>();
        structural_metric_beam_hierarchy_plan(&group, 2)
            .expect("upper plan")
            .edits()
            .iter()
            .copied()
            .map(|edit| {
                (
                    edit.event_ordinal(),
                    edit.level(),
                    edit.state().as_musicxml_text(),
                )
            })
            .collect()
    }
    #[test]
    fn already_normalized_pair_is_idempotent() {
        let first = r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number="1">begin</beam><beam number="2">begin</beam></note>"#;
        let second = r#"<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number="1">end</beam><beam number="2">end</beam></note>"#;
        let raw = vec![
            PartialBeatRawNote { start: 0, end: first.len(), text: first.to_string() },
            PartialBeatRawNote { start: 100, end: 100 + second.len(), text: second.to_string() },
        ];
        let infos = [info(0, 0), info(100, 24)];
        let group = vec![&infos[0], &infos[1]];
        let plan = structural_metric_beam_hierarchy_plan(&group, 2).expect("upper plan");
        assert!(!structural_beam_group_needs_owned_plan_normalization(
            &raw, &group, &plan, 2
        ));
    }
    #[test]
    fn missing_beams_require_normalization() {
        let first = r"<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff></note>";
        let second = r"<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff></note>";
        let raw = vec![
            PartialBeatRawNote { start: 0, end: first.len(), text: first.to_string() },
            PartialBeatRawNote { start: 100, end: 100 + second.len(), text: second.to_string() },
        ];
        let infos = [info(0, 0), info(100, 24)];
        let group = vec![&infos[0], &infos[1]];
        let plan = structural_metric_beam_hierarchy_plan(&group, 2).expect("upper plan");
        assert!(structural_beam_group_needs_owned_plan_normalization(
            &raw, &group, &plan, 2
        ));
    }

    #[test]
    fn upper_plan_stops_secondary_before_trailing_eighth() {
        let infos = vec![
            typed_info(0, 0, 24, "16th"),
            typed_info(100, 24, 24, "16th"),
            typed_info(200, 48, 48, "eighth"),
        ];
        assert_eq!(
            upper_plan_rows(&infos),
            vec![
                (0, 1, "begin"),
                (0, 2, "begin"),
                (1, 1, "continue"),
                (1, 2, "end"),
                (2, 1, "end"),
            ]
        );
    }

    #[test]
    fn upper_plan_starts_secondary_after_leading_eighth() {
        let infos = vec![
            typed_info(0, 0, 48, "eighth"),
            typed_info(100, 48, 24, "16th"),
            typed_info(200, 72, 24, "16th"),
        ];
        assert_eq!(
            upper_plan_rows(&infos),
            vec![
                (0, 1, "begin"),
                (1, 1, "continue"),
                (1, 2, "begin"),
                (2, 1, "end"),
                (2, 2, "end"),
            ]
        );
    }

    #[test]
    fn upper_plan_keeps_four_sixteenths_fully_secondary_beamed() {
        let infos = vec![
            typed_info(0, 0, 24, "16th"),
            typed_info(100, 24, 24, "16th"),
            typed_info(200, 48, 24, "16th"),
            typed_info(300, 72, 24, "16th"),
        ];
        let rows = upper_plan_rows(&infos);
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 1).count(), 4);
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 2).count(), 4);
    }

    #[test]
    fn upper_plan_does_not_expand_authority_to_tertiary_or_quaternary_levels() {
        let infos = vec![
            typed_info(0, 0, 12, "32nd"),
            typed_info(100, 12, 12, "32nd"),
            typed_info(200, 24, 6, "64th"),
            typed_info(300, 30, 6, "64th"),
        ];
        let rows = upper_plan_rows(&infos);
        assert!(rows.iter().all(|(_, level, _)| *level <= 2));
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 2).count(), 4);
    }

    #[test]
    fn upper_rewrite_preserves_existing_higher_beam_levels() {
        let raw = r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>6</duration><type>64th</type><voice>1</voice><staff>1</staff><beam number="1">continue</beam><beam number="2">continue</beam><beam number="3">begin</beam><beam number="4">forward hook</beam></note>"#;
        let normalized = partial_beat_rewrite_owned_beam_levels(
            raw,
            &[(1, "begin"), (2, "end")],
            2,
        )
        .expect("owned-level rewrite");
        assert!(normalized.contains(r#"<beam number="1">begin</beam>"#));
        assert!(normalized.contains(r#"<beam number="2">end</beam>"#));
        assert!(normalized.contains(r#"<beam number="3">begin</beam>"#));
        assert!(normalized.contains(r#"<beam number="4">forward hook</beam>"#));
    }

    #[test]
    fn upper_plan_does_not_invent_secondary_beam_for_isolated_short_note() {
        let infos = vec![
            typed_info(0, 0, 48, "eighth"),
            typed_info(100, 48, 24, "16th"),
            typed_info(200, 72, 24, "eighth"),
        ];
        let rows = upper_plan_rows(&infos);
        assert!(rows.iter().all(|(_, level, _)| *level == 1));
    }

    #[test]
    fn shared_metric_hierarchy_plan_preserves_early_four_level_semantics() {
        let infos = [
            typed_info(0, 0, 6, "64th"),
            typed_info(100, 6, 6, "64th"),
            typed_info(200, 12, 12, "32nd"),
            typed_info(300, 24, 24, "16th"),
        ];
        let group = infos.iter().collect::<Vec<_>>();
        let plan = structural_metric_beam_hierarchy_plan(&group, 4).expect("shared plan");
        let rows = plan
            .edits()
            .iter()
            .copied()
            .map(|edit| (edit.event_ordinal(), edit.level(), edit.state().as_musicxml_text()))
            .collect::<Vec<_>>();
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 1).count(), 4);
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 2).count(), 4);
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 3).count(), 3);
        assert_eq!(rows.iter().filter(|(_, level, _)| *level == 4).count(), 2);
    }


    #[test]
    fn mixed_upper_group_uses_same_plan_for_detection() {
        let first = r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number="1">begin</beam><beam number="2">begin</beam></note>"#;
        let second = r#"<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number="1">continue</beam><beam number="2">end</beam></note>"#;
        let third = r#"<note><pitch><step>E</step><octave>5</octave></pitch><duration>48</duration><type>eighth</type><voice>1</voice><staff>1</staff><beam number="1">end</beam></note>"#;
        let raw = vec![
            PartialBeatRawNote { start: 0, end: first.len(), text: first.to_string() },
            PartialBeatRawNote { start: 100, end: 100 + second.len(), text: second.to_string() },
            PartialBeatRawNote { start: 200, end: 200 + third.len(), text: third.to_string() },
        ];
        let infos = [
            typed_info(0, 0, 24, "16th"),
            typed_info(100, 24, 24, "16th"),
            typed_info(200, 48, 48, "eighth"),
        ];
        let group = infos.iter().collect::<Vec<_>>();
        let plan = structural_metric_beam_hierarchy_plan(&group, 2).expect("upper plan");
        assert!(!structural_beam_group_needs_owned_plan_normalization(
            &raw, &group, &plan, 2
        ));
    }
}

#[cfg(test)]
#[path = "../../tests/unit/register_common_span_owner.rs"]
mod register_common_span_owner_tests;
