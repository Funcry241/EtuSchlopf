//! Local comparison against the complete, frozen baseline. No writer authority.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    AlternativeSchluesselAutoritaetUngeklaert,
    KontextOffen,
    KeinGemeinsamerPlan,
    AbdeckungVerloren,
    GrenzeUnsicher,
    NichtVerbessert,
    Uebernommen,
}

#[derive(Debug, Clone)]
pub(super) struct Vergleich {
    kontext: String,
    basis: String,
    gemeinsam: Option<String>,
    kosten: Option<(i64, i64)>,
    status: Status,
}

fn nur_registerbruch(grenze: &RegisternotationsGrenzenEvidenz) -> bool {
    grenze.staerke == RegisternotationsGrenzenStaerke::Tragend
        && grenze.tonhoehenabstand >= 12
        && !grenze.stille_dazwischen
        && !(grenze.taktgrenze && grenze.dauerveraenderung)
}

fn gesamtkosten(
    kontext: &[&RegisternotationsEreignis],
    plaene: &[&RegisterNotationPlan],
    system: &str,
) -> i64 {
    let abbildungen = plaene.iter().flat_map(|plan| {
        plan.betroffene_ereignisse.iter().map(move |id| (*id, plan.darstellung))
    }).collect::<BTreeMap<_, _>>();
    let noten = kontext.iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .map(|ereignis| {
            let oktaven = abbildungen.get(&ereignis.id)
                .map_or(0, |darstellung| darstellung.darstellungsoktaven());
            i64::from(lesbarkeitswert([ereignis.tonhoehe_midi + 12 * oktaven], system))
        }).sum::<i64>();
    noten + plaene.iter().map(|plan| i64::from(plan.darstellung.bezeichnungsstrafe())).sum::<i64>()
}

fn geschlossen(pruefung: &TopologiePruefung) -> bool {
    pruefung.leere_betroffene_mengen == 0
        && pruefung.nullspannen == 0
        && pruefung.ereignisse_ausserhalb_spanne == 0
        && pruefung.systemfremde_betroffene_ereignisse == 0
        && pruefung.widerspruechliche_ueberlappungen == 0
}

pub(super) fn select_closed_common_spans(
    folgen: &BTreeMap<SystemFolgeSchluessel, Vec<&RegisternotationsEreignis>>,
    grenzen: &[RegisternotationsGrenzenEvidenz],
    ereignisse: &[RegisternotationsEreignis],
    basis: Vec<RegisterNotationPlan>,
) -> (Vec<RegisterNotationPlan>, Vec<Vergleich>) {
    let reine_anker = grenzen.iter().filter(|grenze| nur_registerbruch(grenze))
        .map(|grenze| grenze.rechts).collect::<BTreeSet<_>>();
    let harte_anker = grenzen.iter()
        .filter(|grenze| grenze.staerke == RegisternotationsGrenzenStaerke::Tragend)
        .filter(|grenze| !reine_anker.contains(&grenze.rechts))
        .map(|grenze| grenze.rechts).collect::<BTreeSet<_>>();
    let mut ersetzt = BTreeSet::new();
    let mut neue_plaene = Vec::new();
    let mut zeugnisse = Vec::new();
    for (schluessel, folge) in folgen {
        for kontext in evidenz_segmente(folge, &harte_anker) {
            // Only a removed interior fence can establish this error mechanism.
            if !kontext.iter().skip(1).any(|ereignis| reine_anker.contains(&ereignis.id)) {
                continue;
            }
            let ids = kontext.iter().map(|ereignis| ereignis.id).collect::<BTreeSet<_>>();
            let beginn = kontext.iter().map(|ereignis| ereignis.kanonischer_einsatz).min().unwrap();
            let ende = kontext.iter().map(|ereignis| ereignis.kanonisches_ende).max().unwrap();
            let indizes = basis.iter().enumerate().filter(|(_, plan)| {
                plan.teil == schluessel.teil && plan.system == schluessel.system
                    && plan.beginn < ende && beginn < plan.ende
            }).map(|(index, _)| index).collect::<Vec<_>>();
            let lokal = indizes.iter().map(|index| &basis[*index]).collect::<Vec<_>>();
            let mut zeugnis = Vergleich {
                kontext: crate::sha256::hex(format!("{kontext:?}").as_bytes()),
                basis: plan_fingerabdruck(&lokal.iter().map(|plan| (*plan).clone()).collect::<Vec<_>>()),
                gemeinsam: None,
                kosten: None,
                status: Status::KeinGemeinsamerPlan,
            };
            // Existing lower-domain evaluation may choose a different clef for
            // each window. A shared actual clef context is not in this input model.
            // Preserve that owner instead of minimizing clef costs per note.
            if schluessel.system.trim() == "2" {
                zeugnis.status = Status::AlternativeSchluesselAutoritaetUngeklaert;
            } else if lokal.iter().any(|plan| {
                plan.beginn < beginn || plan.ende > ende
                    || plan.entscheidungskontext.iter().chain(&plan.betroffene_ereignisse)
                        .any(|id| !ids.contains(id))
            }) {
                zeugnis.status = Status::KontextOffen;
            } else if let Some(mut gemeinsam) = plane_segment_mit_pflichtabdeckung(
                schluessel, &kontext, grenzen,
                &lokal.iter().flat_map(|plan| &plan.betroffene_ereignisse).copied().collect(),
            ) {
                gemeinsam.gruende.push(RegisterPlanGrund::GemeinsameAbschnittsAlternative);
                zeugnis.gemeinsam = Some(plan_fingerabdruck(std::slice::from_ref(&gemeinsam)));
                let kosten = (gesamtkosten(&kontext, &lokal, &schluessel.system),
                    gesamtkosten(&kontext, &[&gemeinsam], &schluessel.system));
                zeugnis.kosten = Some(kosten);
                zeugnis.status = if lokal.iter().flat_map(|plan| &plan.betroffene_ereignisse)
                    .any(|id| !gemeinsam.betroffene_ereignisse.contains(id))
                {
                    Status::AbdeckungVerloren
                } else if !oktavspanne_hat_ueberbindungsfreie_grenzen(&gemeinsam, ereignisse)
                    || !geschlossen(&pruefe_topologie(ereignisse, std::slice::from_ref(&gemeinsam)))
                {
                    Status::GrenzeUnsicher
                } else if kosten.1 >= kosten.0 {
                    Status::NichtVerbessert
                } else {
                    ersetzt.extend(indizes);
                    neue_plaene.push(gemeinsam);
                    Status::Uebernommen
                };
            }
            zeugnisse.push(zeugnis);
        }
    }
    let mut plaene = basis.into_iter().enumerate()
        .filter(|(index, _)| !ersetzt.contains(index)).map(|(_, plan)| plan).collect::<Vec<_>>();
    plaene.extend(neue_plaene);
    (plaene, zeugnisse)
}

pub(super) fn fuege_bericht_hinzu(out: &mut String, vergleiche: &[Vergleich]) {
    let _ = writeln!(out, "CommonSpanComparisons: {}; PlannerSelected {}. Schema 1; ExistingRegisterOwner.",
        vergleiche.len(), vergleiche.iter().filter(|vergleich| vergleich.status == Status::Uebernommen).count());
    for vergleich in vergleiche {
        let _ = writeln!(out, "CommonSpan: ContextSha256 {}; BaselineSha256 {}; ProposalSha256 {}; Costs {:?}; Status {:?}.",
            vergleich.kontext, vergleich.basis, vergleich.gemeinsam.as_deref().unwrap_or("NONE"),
            vergleich.kosten, vergleich.status);
    }
}

#[cfg(test)]
#[path = "../../tests/unit/register_common_span.rs"]
mod tests;
