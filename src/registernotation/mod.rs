//! Fachmodell und Entscheidungsautoritaet der Registernotation.
//!
//! Seit r262 liest der Planer ausschliesslich den bereits semantisch geparsten `model::Score`
//! vor K00. Die Fachplanung besitzt selbst weiterhin weder MusicXML- noch
//! StructuralWriteAuthority-Zugriff. r287 schliesst den in r283 bewiesenen Sichtzustandspfad
//! als typisierten Sichtmaterialisierungsauftrag; die MusicXML-Schreibbefugnis bleibt beim
//! bestehenden zentralen StructuralWriteAuthority-Writer.

mod gemeinsame_spanne;
mod ottava_kontext;
pub(crate) mod reader_policy;
pub(crate) use ottava_kontext::OttavaCarrier;

use crate::model::{EventKind, Score};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::sync::{Mutex, OnceLock};

const MINDEST_TONOBJEKTE: usize = 4;
const VERBESSERUNGSSCHWELLE: i32 = 8;
const DOPPELOKTAV_BEZEICHNUNGSSTRAFE: i32 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RegisternotationsEreignisId {
    pub(crate) teil_ordinal: usize,
    pub(crate) takt_ordinal: usize,
    pub(crate) ereignis_ordinal: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RegisternotationsSichtbarkeit {
    Gedruckt,
    VerborgenStrukturell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsEreignis {
    pub(crate) id: RegisternotationsEreignisId,
    pub(crate) teil: String,
    pub(crate) system: String,
    pub(crate) stimme: String,
    pub(crate) takt_ordinal: usize,
    pub(crate) taktbezeichnung: String,
    pub(crate) einsatz: i64,
    pub(crate) dauer: i64,
    pub(crate) kanonischer_einsatz: i128,
    pub(crate) kanonisches_ende: i128,
    pub(crate) tonhoehe_midi: i32,
    pub(crate) sichtbarkeit: RegisternotationsSichtbarkeit,
    pub(crate) akkordton: bool,
    pub(crate) ueberbindungsfortsetzung: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsGrenzenEvidenz {
    pub(crate) links: RegisternotationsEreignisId,
    pub(crate) rechts: RegisternotationsEreignisId,
    pub(crate) taktgrenze: bool,
    pub(crate) zaehlzeitgrenze: bool,
    pub(crate) stille_dazwischen: bool,
    pub(crate) stille_dauer: i128,
    pub(crate) tonhoehenabstand: i32,
    pub(crate) dauerveraenderung: bool,
    pub(crate) staerke: RegisternotationsGrenzenStaerke,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RegisternotationsGrenzenStaerke {
    Schwach,
    Tragend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RegisternotationsDarstellung {
    Keine,
    Achtva,
    Fuenfzehnma,
    Achtvb,
    Fuenfzehnmb,
}

impl RegisternotationsDarstellung {
    pub(crate) fn bezeichnung(self) -> &'static str {
        match self {
            Self::Keine => "keine",
            Self::Achtva => "8va",
            Self::Fuenfzehnma => "15ma",
            Self::Achtvb => "8vb",
            Self::Fuenfzehnmb => "15mb",
        }
    }

    pub(crate) fn groesse(self) -> i32 {
        match self {
            Self::Keine => 0,
            Self::Achtva | Self::Achtvb => 8,
            Self::Fuenfzehnma | Self::Fuenfzehnmb => 15,
        }
    }

    fn darstellungsoktaven(self) -> i32 {
        match self {
            Self::Keine => 0,
            Self::Achtva => -1,
            Self::Fuenfzehnma => -2,
            Self::Achtvb => 1,
            Self::Fuenfzehnmb => 2,
        }
    }

    fn bezeichnungsstrafe(self) -> i32 {
        if self.groesse() == 15 {
            DOPPELOKTAV_BEZEICHNUNGSSTRAFE
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RegisterPlanGrund {
    BestehendeBewertungsformel,
    SichtbareRegisterlast,
    StrukturelleKontinuitaet,
    TragendeGruppierungsgrenze,
    SchwacheTaktgrenzeUeberbrueckt,
    KlingendeZwischeninselKontinuitaet,
    LokaleLesbarkeitsspanne,
    UnteresSystemSchluesselvorrang,
    GemeinsameAbschnittsAlternative,
}

impl RegisterPlanGrund {
    fn text(self) -> &'static str {
        match self {
            Self::BestehendeBewertungsformel => "BestehendeBewertungsformel",
            Self::SichtbareRegisterlast => "SichtbareRegisterlast",
            Self::StrukturelleKontinuitaet => "StrukturelleKontinuitaet",
            Self::TragendeGruppierungsgrenze => "TragendeGruppierungsgrenze",
            Self::SchwacheTaktgrenzeUeberbrueckt => "SchwacheTaktgrenzeUeberbrueckt",
            Self::KlingendeZwischeninselKontinuitaet => "KlingendeZwischeninselKontinuitaet",
            Self::LokaleLesbarkeitsspanne => "LokaleLesbarkeitsspanne",
            Self::UnteresSystemSchluesselvorrang => "UnteresSystemSchluesselvorrang",
            Self::GemeinsameAbschnittsAlternative => "GemeinsameAbschnittsAlternative",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisterNotationPlan {
    pub(crate) entscheidungskontext: Vec<RegisternotationsEreignisId>,
    pub(crate) betroffene_ereignisse: Vec<RegisternotationsEreignisId>,
    pub(crate) teil: String,
    pub(crate) beginn_takt_ordinal: usize,
    pub(crate) ende_takt_ordinal: usize,
    pub(crate) system: String,
    pub(crate) beginn: i128,
    pub(crate) ende: i128,
    pub(crate) darstellung: RegisternotationsDarstellung,
    pub(crate) gruende: Vec<RegisterPlanGrund>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsSichtwiederholung {
    pub(crate) takt_ordinal: usize,
    pub(crate) einsatz: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsMaterialisierungsPlan {
    pub(crate) teil: String,
    pub(crate) beginn_takt_ordinal: usize,
    pub(crate) ende_takt_ordinal: usize,
    pub(crate) system: String,
    pub(crate) beginn_einsatz: i64,
    pub(crate) ende_einsatz: i64,
    pub(crate) darstellung: RegisternotationsDarstellung,
    pub(crate) sichtwiederholungen: Vec<RegisternotationsSichtwiederholung>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsMaterialisierungsauftrag {
    pub(crate) plaene: Vec<RegisternotationsMaterialisierungsPlan>,
    pub(crate) plan_fingerabdruck: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsSichtmaterialisierungsauftrag {
    pub(crate) plaene: Vec<RegisternotationsMaterialisierungsPlan>,
    pub(crate) semantischer_plan_fingerabdruck: String,
    pub(crate) sicht_fingerabdruck: String,
    pub(crate) semantische_plaene: usize,
    pub(crate) kontaktfusionen: usize,
    pub(crate) stille_bruecken: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisternotationsMaterialisierungsZeugnis {
    pub(crate) semantische_plaene: usize,
    pub(crate) sichtlinien: usize,
    pub(crate) starts: usize,
    pub(crate) continues: usize,
    pub(crate) stops: usize,
    pub(crate) semantischer_plan_fingerabdruck: String,
    pub(crate) sicht_fingerabdruck: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RegisternotationsAbweichungsgrund {
    Sichtbarkeitsmenge,
    Gruppierungsgrenze,
    BetroffeneEreignismenge,
    Spannenzuschnitt,
    Oktavart,
    UnteresSystemSchluesselvorrang,
}

impl RegisternotationsAbweichungsgrund {
    fn text(self) -> &'static str {
        match self {
            Self::Sichtbarkeitsmenge => "Sichtbarkeitsmenge",
            Self::Gruppierungsgrenze => "Gruppierungsgrenze",
            Self::BetroffeneEreignismenge => "BetroffeneEreignismenge",
            Self::Spannenzuschnitt => "Spannenzuschnitt",
            Self::Oktavart => "Oktavart",
            Self::UnteresSystemSchluesselvorrang => "UnteresSystemSchluesselvorrang",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SystemTaktSchluessel {
    teil: String,
    takt_ordinal: usize,
    system: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SystemFolgeSchluessel {
    teil: String,
    system: String,
}

#[derive(Debug, Clone, Copy)]
struct TaktZeit {
    absoluter_beginn: i128,
    teilung: i64,
    zaehlzeit_ticks: i128,
}

#[derive(Debug, Clone)]
struct KanonischeTaktzeitAchse {
    skalierung: i64,
    zeiten: BTreeMap<(String, usize), TaktZeit>,
}

#[derive(Debug, Clone, Default)]
struct RegisternotationsBeweis {
    ereignisse: Vec<RegisternotationsEreignis>,
    grenzen: Vec<RegisternotationsGrenzenEvidenz>,
    system_takt_gruppen: usize,
    abschnitte: usize,
    taktuebergreifende_abschnitte: usize,
    tragende_grenzen: usize,
    schwache_grenzen: usize,
    tragende_taktgrenzen: usize,
    schwache_taktgrenzen: usize,
    gedruckte_ereignisse: usize,
    verborgene_ereignisse: usize,
    altentscheidungen: BTreeMap<SystemTaktSchluessel, RegisternotationsDarstellung>,
    sichtbarkeits_gegenrechnungs_aenderungen: usize,
    plaene: Vec<RegisterNotationPlan>,
    klingende_zwischeninsel_kontinuitaetsplaene: usize,
    kurze_randinsel_kontinuitaetsplaene: usize,
    abweichungsgruende: BTreeMap<RegisternotationsAbweichungsgrund, usize>,
    leere_betroffene_mengen: usize,
    nullspannen: usize,
    ereignisse_ausserhalb_spanne: usize,
    systemfremde_betroffene_ereignisse: usize,
    widerspruechliche_ueberlappungen: usize,
    nicht_klassifizierte_abweichungen: usize,
    plan_fingerabdruck: String,
    basis_plan_fingerabdruck: String,
    gemeinsame_spannen: Vec<gemeinsame_spanne::Vergleich>,
}

#[derive(Debug, Clone, Default)]
struct Sitzung {
    eingang: Option<crate::audit_only::register_planner_input::InputSnapshot>,
    beweis: Option<Result<RegisternotationsBeweis, String>>,
    materialisierung: Option<RegisternotationsMaterialisierungsZeugnis>,
    ottava_kontext: Option<ottava_kontext::MaterializedContext>,
}

static SITZUNG: OnceLock<Mutex<Sitzung>> = OnceLock::new();

fn sitzung() -> &'static Mutex<Sitzung> {
    SITZUNG.get_or_init(|| Mutex::new(Sitzung::default()))
}

#[cfg(test)]
pub(crate) fn sperre_fuer_tests() -> std::sync::MutexGuard<'static, ()> {
    static SPERRE: Mutex<()> = Mutex::new(());
    SPERRE.lock().unwrap_or_else(|vergiftet| vergiftet.into_inner())
}

pub(crate) fn loesche() {
    reader_policy::reset();
    let mut zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    *zustand = Sitzung::default();
}

pub(crate) fn beobachte_vor_k00(xml: &str) {
    reader_policy::reset();
    let eingang = crate::audit_only::register_planner_input::InputSnapshot::capture(xml);
    let beweis = crate::musicxml::parse_musicxml(xml)
        .map_err(|fehler| format!("Score-Projektion vor K00 fehlgeschlagen: {fehler:#}"))
        .and_then(|score| analysiere_score(&score));
    let mut zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    zustand.eingang = Some(eingang);
    zustand.beweis = Some(beweis);
}

fn analysiere_score(score: &Score) -> Result<RegisternotationsBeweis, String> {
    let zeitachse = kanonische_taktzeiten(score)?;
    let ereignisse = beobachte_ereignisse(score, zeitachse.skalierung, &zeitachse.zeiten)?;
    let taktgruppen = gruppiere_system_takt(&ereignisse);
    let systemfolgen = gruppiere_systemfolge(&ereignisse);
    let grenzen = beobachte_grenzen(&systemfolgen, &zeitachse.zeiten);

    let mut altentscheidungen = BTreeMap::new();
    let mut sichtbare_entscheidungen = BTreeMap::new();
    let mut sichtbarkeits_gegenrechnungs_aenderungen = 0usize;

    for (schluessel, gruppe) in &taktgruppen {
        let alle_midis = gruppe
            .iter()
            .map(|ereignis| ereignis.tonhoehe_midi)
            .collect::<Vec<_>>();
        let sichtbare_midis = gruppe
            .iter()
            .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
            .map(|ereignis| ereignis.tonhoehe_midi)
            .collect::<Vec<_>>();
        let alt = entscheide_oktavdarstellung(&alle_midis, alle_midis.len(), &schluessel.system);
        let nur_sichtbar = entscheide_oktavdarstellung(
            &sichtbare_midis,
            sichtbare_midis.len(),
            &schluessel.system,
        );
        if alt != nur_sichtbar {
            sichtbarkeits_gegenrechnungs_aenderungen =
                sichtbarkeits_gegenrechnungs_aenderungen.saturating_add(1);
        }
        altentscheidungen.insert(schluessel.clone(), alt);
        sichtbare_entscheidungen.insert(schluessel.clone(), nur_sichtbar);
    }

    let tragende_rechte_anker = grenzen
        .iter()
        .filter(|grenze| grenze.staerke == RegisternotationsGrenzenStaerke::Tragend)
        .map(|grenze| grenze.rechts)
        .collect::<BTreeSet<_>>();
    let mut plaene = Vec::new();
    let mut abschnitte = 0usize;
    let mut taktuebergreifende_abschnitte = 0usize;

    for (schluessel, folge) in &systemfolgen {
        let segmente = evidenz_segmente(folge, &tragende_rechte_anker);
        abschnitte = abschnitte.saturating_add(segmente.len());
        for segment in &segmente {
            let erster_takt = segment.first().map(|ereignis| ereignis.takt_ordinal);
            let letzter_takt = segment.last().map(|ereignis| ereignis.takt_ordinal);
            if erster_takt.zip(letzter_takt).is_some_and(|(links, rechts)| links != rechts) {
                taktuebergreifende_abschnitte = taktuebergreifende_abschnitte.saturating_add(1);
            }
            if let Some(plan) = plane_segment_optimal_zusammenhaengend(
                schluessel,
                segment,
                &grenzen,
            ) {
                plaene.push(plan);
            }
        }
    }

    let (mut plaene, _) = ergaenze_klingende_zwischeninseln(&systemfolgen, plaene);
    // This only refuses an already planned display span. It neither joins ties
    // nor invents source identity, a new boundary or an alternative register span.
    plaene.retain(|plan| oktavspanne_hat_ueberbindungsfreie_grenzen(plan, &ereignisse));
    sortiere_plaene(&mut plaene);
    let basis_plan_fingerabdruck = plan_fingerabdruck(&plaene);
    let (mut plaene, gemeinsame_spannen) = gemeinsame_spanne::select_closed_common_spans(
        &systemfolgen, &grenzen, &ereignisse, plaene,
    );
    let klingende_zwischeninsel_kontinuitaetsplaene = plaene.iter()
        .filter(|plan| plan.gruende.contains(&RegisterPlanGrund::KlingendeZwischeninselKontinuitaet))
        .count();
    let kurze_randinsel_kontinuitaetsplaene = 0usize;

    sortiere_plaene(&mut plaene);

    let mut abweichungsgruende = BTreeMap::new();
    let mut nicht_klassifizierte_abweichungen = 0usize;
    for (schluessel, gruppe) in &taktgruppen {
        let alt = altentscheidungen
            .get(schluessel)
            .copied()
            .unwrap_or(RegisternotationsDarstellung::Keine);
        let nur_sichtbar = sichtbare_entscheidungen
            .get(schluessel)
            .copied()
            .unwrap_or(RegisternotationsDarstellung::Keine);
        if klassifiziere_abweichung(
            schluessel,
            gruppe,
            alt,
            nur_sichtbar,
            &grenzen,
            &plaene,
            &mut abweichungsgruende,
        ) {
            nicht_klassifizierte_abweichungen =
                nicht_klassifizierte_abweichungen.saturating_add(1);
        }
    }

    let topologie = pruefe_topologie(&ereignisse, &plaene);
    let plan_fingerabdruck = plan_fingerabdruck(&plaene);
    let system_takt_gruppen = taktgruppen.len();
    let gedruckte_ereignisse = ereignisse
        .iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .count();
    let verborgene_ereignisse = ereignisse.len().saturating_sub(gedruckte_ereignisse);
    let tragende_grenzen = grenzen
        .iter()
        .filter(|grenze| grenze.staerke == RegisternotationsGrenzenStaerke::Tragend)
        .count();
    let schwache_grenzen = grenzen.len().saturating_sub(tragende_grenzen);
    let tragende_taktgrenzen = grenzen
        .iter()
        .filter(|grenze| {
            grenze.taktgrenze && grenze.staerke == RegisternotationsGrenzenStaerke::Tragend
        })
        .count();
    let schwache_taktgrenzen = grenzen
        .iter()
        .filter(|grenze| {
            grenze.taktgrenze && grenze.staerke == RegisternotationsGrenzenStaerke::Schwach
        })
        .count();

    Ok(RegisternotationsBeweis {
        ereignisse,
        grenzen,
        system_takt_gruppen,
        abschnitte,
        taktuebergreifende_abschnitte,
        tragende_grenzen,
        schwache_grenzen,
        tragende_taktgrenzen,
        schwache_taktgrenzen,
        gedruckte_ereignisse,
        verborgene_ereignisse,
        altentscheidungen,
        sichtbarkeits_gegenrechnungs_aenderungen,
        plaene,
        klingende_zwischeninsel_kontinuitaetsplaene,
        kurze_randinsel_kontinuitaetsplaene,
        abweichungsgruende,
        leere_betroffene_mengen: topologie.leere_betroffene_mengen,
        nullspannen: topologie.nullspannen,
        ereignisse_ausserhalb_spanne: topologie.ereignisse_ausserhalb_spanne,
        systemfremde_betroffene_ereignisse: topologie.systemfremde_betroffene_ereignisse,
        widerspruechliche_ueberlappungen: topologie.widerspruechliche_ueberlappungen,
        nicht_klassifizierte_abweichungen,
        plan_fingerabdruck,
        basis_plan_fingerabdruck,
        gemeinsame_spannen,
    })
}

fn sortiere_plaene(plaene: &mut [RegisterNotationPlan]) {
    plaene.sort_by(|links, rechts| {
        (
            &links.teil,
            links.beginn_takt_ordinal,
            links.ende_takt_ordinal,
            &links.system,
            links.beginn,
            links.ende,
            links.darstellung,
        )
            .cmp(&(
                &rechts.teil,
                rechts.beginn_takt_ordinal,
                rechts.ende_takt_ordinal,
                &rechts.system,
                rechts.beginn,
                rechts.ende,
                rechts.darstellung,
            ))
    });
}

fn kanonische_taktzeiten(score: &Score) -> Result<KanonischeTaktzeitAchse, String> {
    let mut skalierung = 1_i64;
    let mut taktanzahl = 0usize;
    for teil in &score.parts {
        for takt in &teil.measures {
            if takt.divisions <= 0 || takt.beats <= 0 || takt.beat_type <= 0 {
                return Err(format!(
                    "ungueltige Metrik in Teil {} Takt {}",
                    teil.id, takt.number
                ));
            }
            skalierung = checked_lcm(skalierung, takt.divisions)
                .ok_or_else(|| "kanonische Zeitskalierung ueberlaeuft".to_string())?;
            skalierung = checked_lcm(skalierung, takt.expected.denominator.max(1))
                .ok_or_else(|| "kanonische Zeitskalierung ueberlaeuft".to_string())?;
            taktanzahl = taktanzahl.saturating_add(1);
        }
    }
    if taktanzahl == 0 {
        return Err("Score besitzt keine Takte".to_string());
    }

    let mut zeiten = BTreeMap::new();
    for teil in &score.parts {
        let mut absoluter_beginn = 0_i128;
        for (index, takt) in teil.measures.iter().enumerate() {
            if takt.cursor_min != 0 || takt.expected.compare_to_actual(takt.cursor_max) != 0 {
                return Err(format!(
                    "rhythmisch unvollstaendiger Takt vor K00: Teil {} Takt {}",
                    teil.id, takt.number
                ));
            }
            let erwartungs_zaehler = i128::from(takt.expected.numerator)
                .checked_mul(i128::from(skalierung))
                .ok_or_else(|| "Taktlaengenberechnung ueberlaeuft".to_string())?;
            let erwartungs_nenner = i128::from(takt.expected.denominator.max(1))
                .checked_mul(i128::from(takt.divisions))
                .ok_or_else(|| "Taktlaengenberechnung ueberlaeuft".to_string())?;
            if erwartungs_nenner <= 0 || erwartungs_zaehler % erwartungs_nenner != 0 {
                return Err(format!(
                    "Taktlaenge nicht exakt kanonisierbar: Teil {} Takt {}",
                    teil.id, takt.number
                ));
            }
            let taktdauer = erwartungs_zaehler / erwartungs_nenner;
            let beat_zaehler = i128::from(skalierung)
                .checked_mul(4)
                .ok_or_else(|| "Zaehlzeitberechnung ueberlaeuft".to_string())?;
            let beat_nenner = i128::from(takt.beat_type.max(1));
            if beat_zaehler % beat_nenner != 0 {
                return Err(format!(
                    "Zaehlzeit nicht exakt kanonisierbar: Teil {} Takt {}",
                    teil.id, takt.number
                ));
            }
            let zaehlzeit_ticks = beat_zaehler / beat_nenner;
            zeiten.insert(
                (teil.id.clone(), index.saturating_add(1)),
                TaktZeit {
                    absoluter_beginn,
                    teilung: takt.divisions,
                    zaehlzeit_ticks,
                },
            );
            absoluter_beginn = absoluter_beginn
                .checked_add(taktdauer)
                .ok_or_else(|| "absolute Scorezeit ueberlaeuft".to_string())?;
        }
    }
    Ok(KanonischeTaktzeitAchse {
        skalierung,
        zeiten,
    })
}

fn beobachte_ereignisse(
    score: &Score,
    skalierung: i64,
    zeiten: &BTreeMap<(String, usize), TaktZeit>,
) -> Result<Vec<RegisternotationsEreignis>, String> {
    let mut ausgabe = Vec::new();
    for (teil_index, teil) in score.parts.iter().enumerate() {
        for (takt_index, takt) in teil.measures.iter().enumerate() {
            let takt_ordinal = takt_index.saturating_add(1);
            let taktzeit = zeiten
                .get(&(teil.id.clone(), takt_ordinal))
                .ok_or_else(|| "kanonische Taktzeit fehlt".to_string())?;
            for (ereignis_index, ereignis) in takt.events.iter().enumerate() {
                if ereignis.kind != EventKind::Note {
                    continue;
                }
                let Some(tonhoehe) = ereignis.pitch.as_ref() else {
                    continue;
                };
                let lokaler_einsatz = skaliere_exakt(ereignis.start, skalierung, taktzeit.teilung)
                    .ok_or_else(|| "Einsatzzeit nicht exakt kanonisierbar".to_string())?;
                let kanonische_dauer =
                    skaliere_exakt(ereignis.duration, skalierung, taktzeit.teilung)
                        .ok_or_else(|| "Dauer nicht exakt kanonisierbar".to_string())?;
                let kanonischer_einsatz = taktzeit
                    .absoluter_beginn
                    .checked_add(lokaler_einsatz)
                    .ok_or_else(|| "Einsatzzeit ueberlaeuft".to_string())?;
                let kanonisches_ende = kanonischer_einsatz
                    .checked_add(kanonische_dauer)
                    .ok_or_else(|| "Endzeit ueberlaeuft".to_string())?;
                ausgabe.push(RegisternotationsEreignis {
                    id: RegisternotationsEreignisId {
                        teil_ordinal: teil_index.saturating_add(1),
                        takt_ordinal,
                        ereignis_ordinal: ereignis_index.saturating_add(1),
                    },
                    teil: teil.id.clone(),
                    system: ereignis.staff.clone(),
                    stimme: ereignis.voice.clone(),
                    takt_ordinal,
                    taktbezeichnung: takt.number.clone(),
                    einsatz: ereignis.start,
                    dauer: ereignis.duration,
                    kanonischer_einsatz,
                    kanonisches_ende,
                    tonhoehe_midi: tonhoehe.midi,
                    sichtbarkeit: if ereignis.visible {
                        RegisternotationsSichtbarkeit::Gedruckt
                    } else {
                        RegisternotationsSichtbarkeit::VerborgenStrukturell
                    },
                    akkordton: ereignis.is_chord,
                    ueberbindungsfortsetzung: ereignis.tie_stop,
                });
            }
        }
    }
    Ok(ausgabe)
}

/// Protected incoming ties exclude an octave-display change on their onset.
/// The complete part/staff scope is examined, including hidden continuations
/// and the next measure. Numeric voices and pitch never select a parent or role.
fn oktavspanne_hat_ueberbindungsfreie_grenzen(
    plan: &RegisterNotationPlan, ereignisse: &[RegisternotationsEreignis],
) -> bool {
    !ereignisse.iter().any(|ereignis| ereignis.ueberbindungsfortsetzung
        && ereignis.teil == plan.teil && ereignis.system == plan.system
        && (ereignis.kanonischer_einsatz == plan.beginn
            || ereignis.kanonischer_einsatz == plan.ende))
}

fn gruppiere_system_takt(
    ereignisse: &[RegisternotationsEreignis],
) -> BTreeMap<SystemTaktSchluessel, Vec<&RegisternotationsEreignis>> {
    let mut gruppen = BTreeMap::<SystemTaktSchluessel, Vec<&RegisternotationsEreignis>>::new();
    for ereignis in ereignisse {
        gruppen
            .entry(SystemTaktSchluessel {
                teil: ereignis.teil.clone(),
                takt_ordinal: ereignis.takt_ordinal,
                system: ereignis.system.clone(),
            })
            .or_default()
            .push(ereignis);
    }
    for gruppe in gruppen.values_mut() {
        gruppe.sort_by_key(|ereignis| {
            (
                ereignis.kanonischer_einsatz,
                ereignis.kanonisches_ende,
                ereignis.id,
            )
        });
    }
    gruppen
}

fn gruppiere_systemfolge(
    ereignisse: &[RegisternotationsEreignis],
) -> BTreeMap<SystemFolgeSchluessel, Vec<&RegisternotationsEreignis>> {
    let mut folgen = BTreeMap::<SystemFolgeSchluessel, Vec<&RegisternotationsEreignis>>::new();
    for ereignis in ereignisse {
        folgen
            .entry(SystemFolgeSchluessel {
                teil: ereignis.teil.clone(),
                system: ereignis.system.clone(),
            })
            .or_default()
            .push(ereignis);
    }
    for folge in folgen.values_mut() {
        folge.sort_by_key(|ereignis| {
            (
                ereignis.kanonischer_einsatz,
                ereignis.kanonisches_ende,
                ereignis.id,
            )
        });
    }
    folgen
}

fn beobachte_grenzen(
    systemfolgen: &BTreeMap<SystemFolgeSchluessel, Vec<&RegisternotationsEreignis>>,
    zeiten: &BTreeMap<(String, usize), TaktZeit>,
) -> Vec<RegisternotationsGrenzenEvidenz> {
    let mut ausgabe = Vec::new();
    for (schluessel, folge) in systemfolgen {
        let angriffe = angriffe(folge);
        for paar in angriffe.windows(2) {
            let links = &paar[0];
            let rechts = &paar[1];
            let taktgrenze = links.erste_id.takt_ordinal != rechts.erste_id.takt_ordinal;
            let Some(rechter_takt) = zeiten.get(&(
                schluessel.teil.clone(),
                rechts.erste_id.takt_ordinal,
            )) else {
                continue;
            };
            ausgabe.push(grenzen_evidenz(
                links,
                rechts,
                taktgrenze,
                rechter_takt,
                &schluessel.system,
            ));
        }
    }
    ausgabe
}

fn grenzen_evidenz(
    links: &Angriff,
    rechts: &Angriff,
    taktgrenze: bool,
    rechter_takt: &TaktZeit,
    system: &str,
) -> RegisternotationsGrenzenEvidenz {
    let stille_dauer = (rechts.start - links.ende).max(0);
    let stille_dazwischen = stille_dauer > 0;
    let lokaler_rechter_einsatz = rechts.start - rechter_takt.absoluter_beginn;
    let zaehlzeitgrenze = rechter_takt.zaehlzeit_ticks > 0
        && lokaler_rechter_einsatz >= 0
        && lokaler_rechter_einsatz % rechter_takt.zaehlzeit_ticks == 0;
    let struktureller_tonhoehenabstand =
        kleinster_tonhoehenabstand(&links.tonhoehen, &rechts.tonhoehen);
    let tonhoehenabstand = if system.trim() == "1"
        && struktureller_tonhoehenabstand < 12
        && !links.sichtbare_tonhoehen.is_empty()
        && !rechts.sichtbare_tonhoehen.is_empty()
    {
        let sichtbarer_tonhoehenabstand = kleinster_tonhoehenabstand(
            &links.sichtbare_tonhoehen,
            &rechts.sichtbare_tonhoehen,
        );
        if sichtbarer_tonhoehenabstand >= 12 {
            sichtbarer_tonhoehenabstand
        } else {
            struktureller_tonhoehenabstand
        }
    } else {
        struktureller_tonhoehenabstand
    };
    let dauerveraenderung = links.dauer != rechts.dauer;
    let staerke = if rechts.start < links.ende {
        RegisternotationsGrenzenStaerke::Schwach
    } else {
        let ganze_zaehlzeit_stille = rechter_takt.zaehlzeit_ticks > 0
            && stille_dauer >= rechter_takt.zaehlzeit_ticks;
        let registerbruch = tonhoehenabstand >= 12;
        let takt_mit_stuetzender_evidenz =
            taktgrenze && (stille_dazwischen || dauerveraenderung);
        let zaehlzeit_mit_stille_und_dauerwechsel =
            zaehlzeitgrenze && stille_dazwischen && dauerveraenderung;
        if ganze_zaehlzeit_stille
            || registerbruch
            || takt_mit_stuetzender_evidenz
            || zaehlzeit_mit_stille_und_dauerwechsel
        {
            RegisternotationsGrenzenStaerke::Tragend
        } else {
            RegisternotationsGrenzenStaerke::Schwach
        }
    };
    RegisternotationsGrenzenEvidenz {
        links: links.erste_id,
        rechts: rechts.erste_id,
        taktgrenze,
        zaehlzeitgrenze,
        stille_dazwischen,
        stille_dauer,
        tonhoehenabstand,
        dauerveraenderung,
        staerke,
    }
}

#[derive(Debug, Clone)]
struct Angriff {
    erste_id: RegisternotationsEreignisId,
    start: i128,
    ende: i128,
    dauer: i128,
    tonhoehen: Vec<i32>,
    sichtbare_tonhoehen: Vec<i32>,
}

fn angriffe(gruppe: &[&RegisternotationsEreignis]) -> Vec<Angriff> {
    let mut ausgabe = Vec::<Angriff>::new();
    for ereignis in gruppe {
        if let Some(letzter) = ausgabe.last_mut()
            && letzter.start == ereignis.kanonischer_einsatz {
                letzter.ende = letzter.ende.max(ereignis.kanonisches_ende);
                letzter.dauer = letzter.dauer.max(
                    ereignis
                        .kanonisches_ende
                        .saturating_sub(ereignis.kanonischer_einsatz),
                );
                letzter.tonhoehen.push(ereignis.tonhoehe_midi);
                if ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt {
                    letzter.sichtbare_tonhoehen.push(ereignis.tonhoehe_midi);
                }
                continue;
            }
        ausgabe.push(Angriff {
            erste_id: ereignis.id,
            start: ereignis.kanonischer_einsatz,
            ende: ereignis.kanonisches_ende,
            dauer: ereignis
                .kanonisches_ende
                .saturating_sub(ereignis.kanonischer_einsatz),
            tonhoehen: vec![ereignis.tonhoehe_midi],
            sichtbare_tonhoehen: if ereignis.sichtbarkeit
                == RegisternotationsSichtbarkeit::Gedruckt
            {
                vec![ereignis.tonhoehe_midi]
            } else {
                Vec::new()
            },
        });
    }
    ausgabe
}

fn kleinster_tonhoehenabstand(links: &[i32], rechts: &[i32]) -> i32 {
    links
        .iter()
        .flat_map(|links| rechts.iter().map(move |rechts| (links - rechts).abs()))
        .min()
        .unwrap_or(0)
}

fn evidenz_segmente<'a>(
    folge: &[&'a RegisternotationsEreignis],
    tragende_rechte_anker: &BTreeSet<RegisternotationsEreignisId>,
) -> Vec<Vec<&'a RegisternotationsEreignis>> {
    let mut segmente = Vec::<Vec<&RegisternotationsEreignis>>::new();
    let mut aktuelles = Vec::<&RegisternotationsEreignis>::new();
    for ereignis in folge {
        if tragende_rechte_anker.contains(&ereignis.id) && !aktuelles.is_empty() {
            segmente.push(std::mem::take(&mut aktuelles));
        }
        aktuelles.push(*ereignis);
    }
    if !aktuelles.is_empty() {
        segmente.push(aktuelles);
    }
    segmente
}

fn plane_segment(
    schluessel: &SystemFolgeSchluessel,
    segment: &[&RegisternotationsEreignis],
    grenzen: &[RegisternotationsGrenzenEvidenz],
) -> Option<RegisterNotationPlan> {
    if segment.len() < MINDEST_TONOBJEKTE {
        return None;
    }
    let sichtbare = segment
        .iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .copied()
        .collect::<Vec<_>>();
    if sichtbare.is_empty() {
        return None;
    }
    let sichtbare_midis = sichtbare
        .iter()
        .map(|ereignis| ereignis.tonhoehe_midi)
        .collect::<Vec<_>>();
    let darstellung = entscheide_oktavdarstellung(
        &sichtbare_midis,
        segment.len(),
        &schluessel.system,
    );
    if darstellung == RegisternotationsDarstellung::Keine {
        return None;
    }

    let sichtbarer_beginn = sichtbare
        .iter()
        .map(|ereignis| ereignis.kanonischer_einsatz)
        .min()?;
    let sichtbares_ende = sichtbare
        .iter()
        .map(|ereignis| ereignis.kanonisches_ende)
        .max()?;
    let betroffene = segment
        .iter()
        .filter(|ereignis| {
            ereignis.kanonischer_einsatz >= sichtbarer_beginn
                && ereignis.kanonisches_ende <= sichtbares_ende
        })
        .copied()
        .collect::<Vec<_>>();
    let beginn = betroffene
        .iter()
        .map(|ereignis| ereignis.kanonischer_einsatz)
        .min()?;
    let ende = betroffene
        .iter()
        .map(|ereignis| ereignis.kanonisches_ende)
        .max()?;
    let beginn_takt_ordinal = betroffene.iter().map(|ereignis| ereignis.takt_ordinal).min()?;
    let ende_takt_ordinal = betroffene.iter().map(|ereignis| ereignis.takt_ordinal).max()?;

    let kontext_ids = segment
        .iter()
        .map(|ereignis| ereignis.id)
        .collect::<BTreeSet<_>>();
    let mut gruende = vec![
        RegisterPlanGrund::BestehendeBewertungsformel,
        RegisterPlanGrund::SichtbareRegisterlast,
        RegisterPlanGrund::StrukturelleKontinuitaet,
    ];
    if grenzen.iter().any(|grenze| {
        grenze.staerke == RegisternotationsGrenzenStaerke::Tragend
            && grenze.rechts == segment[0].id
    }) {
        gruende.push(RegisterPlanGrund::TragendeGruppierungsgrenze);
    }
    if grenzen.iter().any(|grenze| {
        grenze.taktgrenze
            && grenze.staerke == RegisternotationsGrenzenStaerke::Schwach
            && kontext_ids.contains(&grenze.links)
            && kontext_ids.contains(&grenze.rechts)
    }) {
        gruende.push(RegisterPlanGrund::SchwacheTaktgrenzeUeberbrueckt);
    }
    if schluessel.system.trim() == "2"
        && unteres_system_schluesselalternative_ist_lesbarer(&sichtbare_midis)
    {
        gruende.push(RegisterPlanGrund::UnteresSystemSchluesselvorrang);
    }

    Some(RegisterNotationPlan {
        entscheidungskontext: segment.iter().map(|ereignis| ereignis.id).collect(),
        betroffene_ereignisse: betroffene.iter().map(|ereignis| ereignis.id).collect(),
        teil: schluessel.teil.clone(),
        beginn_takt_ordinal,
        ende_takt_ordinal,
        system: schluessel.system.clone(),
        beginn,
        ende,
        darstellung,
        gruende,
    })
}


#[derive(Debug, Clone, Copy)]
struct OptimalesSpanfenster {
    links: usize,
    rechts: usize,
    darstellung: RegisternotationsDarstellung,
    vorteil: i64,
    abdeckung: usize,
    grenzminimum: VorhandeneGrenzklasse,
    grenzmaximum: VorhandeneGrenzklasse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum VorhandeneGrenzklasse {
    SichererAngriffsschnitt,
    MetrischeGrenze,
    StilleGrenze,
    SegmentOderTragendeGrenze,
}

#[derive(Debug, Clone)]
struct SemantischerSichtanker {
    takt_ordinal: usize,
    stimme: String,
    beginn: i128,
    ende: i128,
    sichtbar: bool,
    letzter_ereignis_ordinal: usize,
}

fn materialisierbare_sichtgrenzen(
    segment: &[&RegisternotationsEreignis],
) -> (BTreeSet<i128>, BTreeSet<i128>) {
    let mut geordnet = segment.to_vec();
    geordnet.sort_by_key(|ereignis| (ereignis.takt_ordinal, ereignis.id.ereignis_ordinal));

    let mut beginne = BTreeSet::new();
    let mut enden = BTreeSet::new();
    let mut aktuell = None::<SemantischerSichtanker>;

    let flush = |aktuell: &mut Option<SemantischerSichtanker>,
                 beginne: &mut BTreeSet<i128>,
                 enden: &mut BTreeSet<i128>| {
        if let Some(anker) = aktuell.take()
            && anker.sichtbar
        {
            beginne.insert(anker.beginn);
            enden.insert(anker.ende);
        }
    };

    for ereignis in geordnet {
        if !ereignis.akkordton {
            flush(&mut aktuell, &mut beginne, &mut enden);
            aktuell = Some(SemantischerSichtanker {
                takt_ordinal: ereignis.takt_ordinal,
                stimme: ereignis.stimme.clone(),
                beginn: ereignis.kanonischer_einsatz,
                ende: ereignis.kanonisches_ende,
                sichtbar: ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt,
                letzter_ereignis_ordinal: ereignis.id.ereignis_ordinal,
            });
            continue;
        }

        let fortsetzbar = aktuell.as_ref().is_some_and(|anker| {
            anker.takt_ordinal == ereignis.takt_ordinal
                && anker.stimme == ereignis.stimme
                && anker.beginn == ereignis.kanonischer_einsatz
                && anker
                    .letzter_ereignis_ordinal
                    .saturating_add(1)
                    == ereignis.id.ereignis_ordinal
        });
        if fortsetzbar {
            if let Some(anker) = aktuell.as_mut() {
                anker.sichtbar |=
                    ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt;
                anker.letzter_ereignis_ordinal = ereignis.id.ereignis_ordinal;
            }
        } else {
            flush(&mut aktuell, &mut beginne, &mut enden);
        }
    }
    flush(&mut aktuell, &mut beginne, &mut enden);
    (beginne, enden)
}

fn sichtbare_fenstergrenzen(
    segment: &[&RegisternotationsEreignis],
    schnitte: &[usize],
) -> BTreeMap<(usize, usize), (i128, i128)> {
    let mut grenzen = BTreeMap::new();
    for (linker_schnitt_index, links) in schnitte.iter().copied().enumerate() {
        let mut sichtbarer_beginn = None::<i128>;
        let mut sichtbares_ende = None::<i128>;
        let mut scan = links;
        for rechts in schnitte.iter().copied().skip(linker_schnitt_index.saturating_add(1)) {
            for ereignis in &segment[scan..rechts] {
                if ereignis.sichtbarkeit != RegisternotationsSichtbarkeit::Gedruckt {
                    continue;
                }
                sichtbarer_beginn = Some(
                    sichtbarer_beginn
                        .map_or(ereignis.kanonischer_einsatz, |aktuell| {
                            aktuell.min(ereignis.kanonischer_einsatz)
                        }),
                );
                sichtbares_ende = Some(
                    sichtbares_ende
                        .map_or(ereignis.kanonisches_ende, |aktuell| {
                            aktuell.max(ereignis.kanonisches_ende)
                        }),
                );
            }
            if let Some(paar) = sichtbarer_beginn.zip(sichtbares_ende) {
                grenzen.insert((links, rechts), paar);
            }
            scan = rechts;
        }
    }
    grenzen
}

fn lokale_sichere_schnittindizes(segment: &[&RegisternotationsEreignis]) -> Vec<usize> {
    if segment.is_empty() {
        return vec![0];
    }

    let mut schnitte = vec![0usize];
    let mut groesstes_bisheriges_ende = segment[0].kanonisches_ende;
    for index in 1..segment.len() {
        let neuer_angriff = segment[index - 1].kanonischer_einsatz
            != segment[index].kanonischer_einsatz;
        if neuer_angriff
            && groesstes_bisheriges_ende <= segment[index].kanonischer_einsatz
        {
            schnitte.push(index);
        }
        groesstes_bisheriges_ende =
            groesstes_bisheriges_ende.max(segment[index].kanonisches_ende);
    }
    if schnitte.last().copied() != Some(segment.len()) {
        schnitte.push(segment.len());
    }
    schnitte
}

fn vorhandene_grenzqualitaet(
    segment: &[&RegisternotationsEreignis],
    schnitt: usize,
    grenzen: &[RegisternotationsGrenzenEvidenz],
) -> VorhandeneGrenzklasse {
    if schnitt == 0 || schnitt == segment.len() {
        return VorhandeneGrenzklasse::SegmentOderTragendeGrenze;
    }
    let links = segment[schnitt - 1].id;
    let rechts = segment[schnitt].id;
    grenzen
        .iter()
        .filter(|grenze| grenze.links == links && grenze.rechts == rechts)
        .map(|grenze| {
            if grenze.staerke == RegisternotationsGrenzenStaerke::Tragend {
                VorhandeneGrenzklasse::SegmentOderTragendeGrenze
            } else if grenze.stille_dazwischen {
                VorhandeneGrenzklasse::StilleGrenze
            } else if grenze.taktgrenze || grenze.zaehlzeitgrenze {
                VorhandeneGrenzklasse::MetrischeGrenze
            } else {
                VorhandeneGrenzklasse::SichererAngriffsschnitt
            }
        })
        .max()
        .unwrap_or(VorhandeneGrenzklasse::SichererAngriffsschnitt)
}

fn besseres_optimales_spanfenster(
    kandidat: OptimalesSpanfenster,
    aktuell: OptimalesSpanfenster,
) -> bool {
    (
        kandidat.vorteil,
        std::cmp::Reverse(kandidat.darstellung.groesse()),
        kandidat.grenzminimum,
        kandidat.grenzmaximum,
        kandidat.abdeckung,
        std::cmp::Reverse(kandidat.links),
        std::cmp::Reverse(kandidat.rechts),
    ) > (
        aktuell.vorteil,
        std::cmp::Reverse(aktuell.darstellung.groesse()),
        aktuell.grenzminimum,
        aktuell.grenzmaximum,
        aktuell.abdeckung,
        std::cmp::Reverse(aktuell.links),
        std::cmp::Reverse(aktuell.rechts),
    )
}

fn plane_segment_optimal_zusammenhaengend(
    schluessel: &SystemFolgeSchluessel,
    segment: &[&RegisternotationsEreignis],
    grenzen: &[RegisternotationsGrenzenEvidenz],
) -> Option<RegisterNotationPlan> {
    plane_segment_mit_pflichtabdeckung(schluessel, segment, grenzen, &BTreeSet::new())
}

fn plane_segment_mit_pflichtabdeckung(
    schluessel: &SystemFolgeSchluessel,
    segment: &[&RegisternotationsEreignis],
    grenzen: &[RegisternotationsGrenzenEvidenz],
    pflicht: &BTreeSet<RegisternotationsEreignisId>,
) -> Option<RegisterNotationPlan> {
    if segment.len() < MINDEST_TONOBJEKTE {
        return None;
    }

    let pflicht_indizes = segment.iter().enumerate()
        .filter(|(_, ereignis)| pflicht.contains(&ereignis.id))
        .map(|(index, _)| index).collect::<Vec<_>>();
    if pflicht_indizes.len() != pflicht.len() {
        return None;
    }
    let schnitte = lokale_sichere_schnittindizes(segment);
    if schnitte.len() < 2 {
        return None;
    }
    let (materialisierbare_beginne, materialisierbare_enden) =
        materialisierbare_sichtgrenzen(segment);
    let sichtfenster = sichtbare_fenstergrenzen(segment, &schnitte);

    let kandidaten = [
        RegisternotationsDarstellung::Achtva,
        RegisternotationsDarstellung::Fuenfzehnma,
        RegisternotationsDarstellung::Achtvb,
        RegisternotationsDarstellung::Fuenfzehnmb,
    ];
    let mut loco_basis = vec![0_i64; segment.len() + 1];
    let mut loco_schluesselalternative = vec![0_i64; segment.len() + 1];
    let mut sichtbare = vec![0usize; segment.len() + 1];
    let mut verschoben_basis: [Vec<i64>; 4] =
        std::array::from_fn(|_| vec![0_i64; segment.len() + 1]);
    let mut verschoben_schluesselalternative: [Vec<i64>; 4] =
        std::array::from_fn(|_| vec![0_i64; segment.len() + 1]);
    let mut ungueltig: [Vec<usize>; 4] =
        std::array::from_fn(|_| vec![0usize; segment.len() + 1]);

    for (index, ereignis) in segment.iter().enumerate() {
        loco_basis[index + 1] = loco_basis[index];
        loco_schluesselalternative[index + 1] = loco_schluesselalternative[index];
        sichtbare[index + 1] = sichtbare[index];
        for kandidat_index in 0..kandidaten.len() {
            verschoben_basis[kandidat_index][index + 1] =
                verschoben_basis[kandidat_index][index];
            verschoben_schluesselalternative[kandidat_index][index + 1] =
                verschoben_schluesselalternative[kandidat_index][index];
            let ungueltig_praefix = &mut ungueltig[kandidat_index];
            ungueltig_praefix[index + 1] = ungueltig_praefix[index];
        }

        if ereignis.sichtbarkeit != RegisternotationsSichtbarkeit::Gedruckt {
            continue;
        }
        sichtbare[index + 1] = sichtbare[index + 1].saturating_add(1);
        let (basis, schluesselalternative) =
            lesbarkeitsbeitraege(ereignis.tonhoehe_midi, &schluessel.system);
        loco_basis[index + 1] = loco_basis[index + 1].saturating_add(i64::from(basis));
        loco_schluesselalternative[index + 1] = loco_schluesselalternative[index + 1]
            .saturating_add(i64::from(schluesselalternative));

        for (kandidat_index, kandidat) in kandidaten.iter().copied().enumerate() {
            let geschrieben = ereignis.tonhoehe_midi + kandidat.darstellungsoktaven() * 12;
            let oktave = geschrieben / 12 - 1;
            if !(0..=9).contains(&oktave) {
                ungueltig[kandidat_index][index + 1] =
                    ungueltig[kandidat_index][index + 1].saturating_add(1);
                continue;
            }
            let (basis, schluesselalternative) =
                lesbarkeitsbeitraege(geschrieben, &schluessel.system);
            verschoben_basis[kandidat_index][index + 1] =
                verschoben_basis[kandidat_index][index + 1].saturating_add(i64::from(basis));
            verschoben_schluesselalternative[kandidat_index][index + 1] =
                verschoben_schluesselalternative[kandidat_index][index + 1]
                    .saturating_add(i64::from(schluesselalternative));
        }
    }

    let mut bestes = None::<OptimalesSpanfenster>;
    for (rechter_schnitt_index, roh_rechts) in schnitte.iter().copied().enumerate().skip(1) {
        for roh_links in schnitte[..rechter_schnitt_index].iter().copied() {
            if pflicht_indizes.first().is_some_and(|index| roh_links > *index)
                || pflicht_indizes.last().is_some_and(|index| roh_rechts <= *index)
            {
                continue;
            }
            let strukturelle_anzahl = roh_rechts.saturating_sub(roh_links);
            if strukturelle_anzahl < MINDEST_TONOBJEKTE
                || sichtbare[roh_rechts].saturating_sub(sichtbare[roh_links]) == 0
            {
                continue;
            }
            let Some((sichtbarer_beginn, sichtbares_ende)) =
                sichtfenster.get(&(roh_links, roh_rechts)).copied()
            else {
                continue;
            };
            if !materialisierbare_beginne.contains(&sichtbarer_beginn)
                || !materialisierbare_enden.contains(&sichtbares_ende)
            {
                continue;
            }

            let aktueller_wert = fenster_lesbarkeitswert(
                &loco_basis,
                &loco_schluesselalternative,
                roh_links,
                roh_rechts,
            );
            let mut beste_darstellung = None::<(i64, RegisternotationsDarstellung)>;
            for (kandidat_index, kandidat) in kandidaten.iter().copied().enumerate() {
                if ungueltig[kandidat_index][roh_rechts]
                    .saturating_sub(ungueltig[kandidat_index][roh_links])
                    > 0
                {
                    continue;
                }
                let wert = fenster_lesbarkeitswert(
                    &verschoben_basis[kandidat_index],
                    &verschoben_schluesselalternative[kandidat_index],
                    roh_links,
                    roh_rechts,
                )
                    .saturating_add(i64::from(kandidat.bezeichnungsstrafe()));
                if beste_darstellung.is_none_or(|(bester_wert, bester_kandidat)| {
                    wert < bester_wert
                        || (wert == bester_wert
                            && kandidat.groesse() < bester_kandidat.groesse())
                }) {
                    beste_darstellung = Some((wert, kandidat));
                }
            }
            let Some((bester_wert, darstellung)) = beste_darstellung else {
                continue;
            };
            if bester_wert.saturating_add(i64::from(VERBESSERUNGSSCHWELLE))
                >= aktueller_wert
            {
                continue;
            }

            let linke_grenze = vorhandene_grenzqualitaet(segment, roh_links, grenzen);
            let rechte_grenze = vorhandene_grenzqualitaet(segment, roh_rechts, grenzen);
            let kandidat = OptimalesSpanfenster {
                links: roh_links,
                rechts: roh_rechts,
                darstellung,
                vorteil: aktueller_wert.saturating_sub(bester_wert),
                abdeckung: strukturelle_anzahl,
                grenzminimum: linke_grenze.min(rechte_grenze),
                grenzmaximum: linke_grenze.max(rechte_grenze),
            };
            if bestes.is_none_or(|aktuell| besseres_optimales_spanfenster(kandidat, aktuell)) {
                bestes = Some(kandidat);
            }
        }
    }

    let bestes = bestes?;
    let mut plan = plane_segment(
        schluessel,
        &segment[bestes.links..bestes.rechts],
        grenzen,
    )?;
    if plan.darstellung != bestes.darstellung {
        return None;
    }
    if bestes.links != 0 || bestes.rechts != segment.len() {
        plan.gruende
            .push(RegisterPlanGrund::LokaleLesbarkeitsspanne);
    }
    Some(plan)
}

fn ergaenze_klingende_zwischeninseln(
    systemfolgen: &BTreeMap<SystemFolgeSchluessel, Vec<&RegisternotationsEreignis>>,
    plaene: Vec<RegisterNotationPlan>,
) -> (Vec<RegisterNotationPlan>, usize) {
    let mut ergaenzungen = Vec::<RegisterNotationPlan>::new();

    for (schluessel, folge) in systemfolgen {
        let mut systemplaene = plaene
            .iter()
            .filter(|plan| plan.teil == schluessel.teil && plan.system == schluessel.system)
            .collect::<Vec<_>>();
        systemplaene.sort_by_key(|plan| (plan.beginn, plan.ende, plan.darstellung));

        for paar in systemplaene.windows(2) {
            let links = paar[0];
            let rechts = paar[1];
            if links.ende >= rechts.beginn
                || !gleiche_darstellungsrichtung(links.darstellung, rechts.darstellung)
            {
                continue;
            }

            let lueckenereignisse = folge
                .iter()
                .copied()
                .filter(|ereignis| {
                    ereignis.kanonischer_einsatz >= links.ende
                        && ereignis.kanonisches_ende <= rechts.beginn
                })
                .collect::<Vec<_>>();
            if !sichtbare_luecke_ist_durchgehend_klingend(
                &lueckenereignisse,
                links.ende,
                rechts.beginn,
            ) {
                continue;
            }

            let Some(darstellung) = waehle_klingende_zwischeninsel_darstellung(
                &lueckenereignisse,
                links.darstellung,
                rechts.darstellung,
                &schluessel.system,
            ) else {
                continue;
            };
            if let Some(plan) = plane_klingende_zwischeninsel(
                schluessel,
                &lueckenereignisse,
                darstellung,
            ) {
                ergaenzungen.push(plan);
            }
        }
    }

    let anzahl = ergaenzungen.len();
    let mut ausgabe = plaene;
    ausgabe.extend(ergaenzungen);
    (ausgabe, anzahl)
}

fn gleiche_darstellungsrichtung(
    links: RegisternotationsDarstellung,
    rechts: RegisternotationsDarstellung,
) -> bool {
    let links = links.darstellungsoktaven();
    let rechts = rechts.darstellungsoktaven();
    links != 0 && rechts != 0 && links.signum() == rechts.signum()
}

fn sichtbare_luecke_ist_durchgehend_klingend(
    ereignisse: &[&RegisternotationsEreignis],
    beginn: i128,
    ende: i128,
) -> bool {
    if beginn >= ende {
        return false;
    }
    let mut intervalle = ereignisse
        .iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .map(|ereignis| (ereignis.kanonischer_einsatz, ereignis.kanonisches_ende))
        .filter(|(links, rechts)| rechts > links)
        .collect::<Vec<_>>();
    intervalle.sort_unstable();

    let mut deckung = beginn;
    for (links, rechts) in intervalle {
        if links > deckung {
            return false;
        }
        deckung = deckung.max(rechts);
        if deckung >= ende {
            return true;
        }
    }
    false
}

fn waehle_klingende_zwischeninsel_darstellung(
    ereignisse: &[&RegisternotationsEreignis],
    linke_darstellung: RegisternotationsDarstellung,
    rechte_darstellung: RegisternotationsDarstellung,
    system: &str,
) -> Option<RegisternotationsDarstellung> {
    let midis = ereignisse
        .iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .map(|ereignis| ereignis.tonhoehe_midi)
        .collect::<Vec<_>>();
    if midis.is_empty() {
        return None;
    }

    let kandidat = entscheide_oktavdarstellung(&midis, MINDEST_TONOBJEKTE, system);
    (kandidat == linke_darstellung || kandidat == rechte_darstellung).then_some(kandidat)
}

fn plane_klingende_zwischeninsel(
    schluessel: &SystemFolgeSchluessel,
    ereignisse: &[&RegisternotationsEreignis],
    darstellung: RegisternotationsDarstellung,
) -> Option<RegisterNotationPlan> {
    let sichtbare = ereignisse
        .iter()
        .copied()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .collect::<Vec<_>>();
    if sichtbare.is_empty() {
        return None;
    }

    let sichtbarer_beginn = sichtbare
        .iter()
        .map(|ereignis| ereignis.kanonischer_einsatz)
        .min()?;
    let sichtbares_ende = sichtbare
        .iter()
        .map(|ereignis| ereignis.kanonisches_ende)
        .max()?;
    let betroffene = ereignisse
        .iter()
        .copied()
        .filter(|ereignis| {
            ereignis.kanonischer_einsatz >= sichtbarer_beginn
                && ereignis.kanonisches_ende <= sichtbares_ende
        })
        .collect::<Vec<_>>();
    let beginn = betroffene
        .iter()
        .map(|ereignis| ereignis.kanonischer_einsatz)
        .min()?;
    let ende = betroffene
        .iter()
        .map(|ereignis| ereignis.kanonisches_ende)
        .max()?;

    Some(RegisterNotationPlan {
        entscheidungskontext: ereignisse.iter().map(|ereignis| ereignis.id).collect(),
        betroffene_ereignisse: betroffene.iter().map(|ereignis| ereignis.id).collect(),
        teil: schluessel.teil.clone(),
        beginn_takt_ordinal: betroffene.iter().map(|ereignis| ereignis.takt_ordinal).min()?,
        ende_takt_ordinal: betroffene.iter().map(|ereignis| ereignis.takt_ordinal).max()?,
        system: schluessel.system.clone(),
        beginn,
        ende,
        darstellung,
        gruende: vec![
            RegisterPlanGrund::BestehendeBewertungsformel,
            RegisterPlanGrund::SichtbareRegisterlast,
            RegisterPlanGrund::StrukturelleKontinuitaet,
            RegisterPlanGrund::KlingendeZwischeninselKontinuitaet,
        ],
    })
}

fn entscheide_oktavdarstellung(
    kosten_midis: &[i32],
    elegibilitaets_anzahl: usize,
    system: &str,
) -> RegisternotationsDarstellung {
    if elegibilitaets_anzahl < MINDEST_TONOBJEKTE || kosten_midis.is_empty() {
        return RegisternotationsDarstellung::Keine;
    }
    let kandidaten = [
        RegisternotationsDarstellung::Achtva,
        RegisternotationsDarstellung::Fuenfzehnma,
        RegisternotationsDarstellung::Achtvb,
        RegisternotationsDarstellung::Fuenfzehnmb,
    ];
    let aktueller_wert = lesbarkeitswert(kosten_midis.iter().copied(), system);
    let mut beste = None::<(i32, RegisternotationsDarstellung)>;
    for kandidat in kandidaten {
        let verschiebung = kandidat.darstellungsoktaven() * 12;
        let mut geschriebene_midis = Vec::with_capacity(kosten_midis.len());
        let mut gueltig = true;
        for midi in kosten_midis {
            let geschrieben = *midi + verschiebung;
            let oktave = geschrieben / 12 - 1;
            if !(0..=9).contains(&oktave) {
                gueltig = false;
                break;
            }
            geschriebene_midis.push(geschrieben);
        }
        if !gueltig {
            continue;
        }
        let wert = lesbarkeitswert(geschriebene_midis.iter().copied(), system)
            + kandidat.bezeichnungsstrafe();
        if beste.is_none_or(|(bester_wert, bester_kandidat)| {
            wert < bester_wert
                || (wert == bester_wert && kandidat.groesse() < bester_kandidat.groesse())
        }) {
            beste = Some((wert, kandidat));
        }
    }
    let Some((bester_wert, kandidat)) = beste else {
        return RegisternotationsDarstellung::Keine;
    };
    if bester_wert + VERBESSERUNGSSCHWELLE < aktueller_wert {
        kandidat
    } else {
        RegisternotationsDarstellung::Keine
    }
}

fn lesbarkeitswert<I>(midis: I, system: &str) -> i32
where
    I: IntoIterator<Item = i32>,
{
    let (basis, schluesselalternative) = midis.into_iter().fold(
        (0_i32, 0_i32),
        |(basis_summe, schluesselalternative_summe), midi| {
            let (basis, schluesselalternative) = lesbarkeitsbeitraege(midi, system);
            (
                basis_summe.saturating_add(basis),
                schluesselalternative_summe.saturating_add(schluesselalternative),
            )
        },
    );
    basis.min(schluesselalternative)
}

fn lesbarkeitsbeitraege(midi: i32, system: &str) -> (i32, i32) {
    if system.trim() == "2" {
        (
            lesbarkeitsbeitrag_fuer_zone(midi, 36, 60),
            lesbarkeitsbeitrag_fuer_zone(midi, 60, 84),
        )
    } else {
        let treble = lesbarkeitsbeitrag_fuer_zone(midi, 60, 84);
        (treble, treble)
    }
}

fn lesbarkeitsbeitrag_fuer_zone(midi: i32, tief: i32, hoch: i32) -> i32 {
    if midi < tief {
        let abstand = tief - midi;
        abstand * abstand + abstand
    } else if midi > hoch {
        let abstand = midi - hoch;
        abstand * abstand + abstand
    } else {
        0
    }
}

fn fenster_lesbarkeitswert(
    basis: &[i64],
    schluesselalternative: &[i64],
    links: usize,
    rechts: usize,
) -> i64 {
    basis[rechts]
        .saturating_sub(basis[links])
        .min(
            schluesselalternative[rechts]
                .saturating_sub(schluesselalternative[links]),
        )
}

fn unteres_system_schluesselalternative_ist_lesbarer(midis: &[i32]) -> bool {
    let (basis, schluesselalternative) = midis.iter().copied().fold(
        (0_i32, 0_i32),
        |(basis_summe, schluesselalternative_summe), midi| {
            let (basis, schluesselalternative) = lesbarkeitsbeitraege(midi, "2");
            (
                basis_summe.saturating_add(basis),
                schluesselalternative_summe.saturating_add(schluesselalternative),
            )
        },
    );
    schluesselalternative < basis
}

fn klassifiziere_abweichung(
    schluessel: &SystemTaktSchluessel,
    gruppe: &[&RegisternotationsEreignis],
    alt: RegisternotationsDarstellung,
    nur_sichtbar: RegisternotationsDarstellung,
    grenzen: &[RegisternotationsGrenzenEvidenz],
    plaene: &[RegisterNotationPlan],
    ausgabe: &mut BTreeMap<RegisternotationsAbweichungsgrund, usize>,
) -> bool {
    let gruppen_ids = gruppe
        .iter()
        .map(|ereignis| ereignis.id)
        .collect::<BTreeSet<_>>();
    let relevante_plaene = plaene
        .iter()
        .filter(|plan| {
            plan.teil == schluessel.teil
                && plan.system == schluessel.system
                && plan
                    .entscheidungskontext
                    .iter()
                    .any(|id| gruppen_ids.contains(id))
        })
        .collect::<Vec<_>>();

    let alt_hat_plan = alt != RegisternotationsDarstellung::Keine;
    let neue_arten = relevante_plaene
        .iter()
        .map(|plan| plan.darstellung)
        .collect::<BTreeSet<_>>();
    let neue_haben_plan = !relevante_plaene.is_empty();
    let alte_betroffene = if alt_hat_plan {
        gruppen_ids.clone()
    } else {
        BTreeSet::new()
    };
    let neue_betroffene = relevante_plaene
        .iter()
        .flat_map(|plan| plan.betroffene_ereignisse.iter().copied())
        .filter(|id| gruppen_ids.contains(id))
        .collect::<BTreeSet<_>>();

    let tragende_grenze_in_gruppe = grenzen.iter().any(|grenze| {
        grenze.staerke == RegisternotationsGrenzenStaerke::Tragend
            && gruppen_ids.contains(&grenze.links)
            && gruppen_ids.contains(&grenze.rechts)
    });
    let gruppierungsabweichung = tragende_grenze_in_gruppe
        || relevante_plaene.len() > 1
        || relevante_plaene.iter().any(|plan| {
            plan.entscheidungskontext
                .iter()
                .any(|id| !gruppen_ids.contains(id))
        });
    let betroffene_abweichung = alte_betroffene != neue_betroffene;
    let oktavart_abweichung = alt_hat_plan != neue_haben_plan
        || (alt_hat_plan && (neue_arten.len() != 1 || !neue_arten.contains(&alt)));

    let alter_beginn = gruppe
        .iter()
        .map(|ereignis| ereignis.kanonischer_einsatz)
        .min();
    let altes_ende = gruppe
        .iter()
        .map(|ereignis| ereignis.kanonisches_ende)
        .max();
    let spannenzuschnitt_abweichung = if alt_hat_plan {
        relevante_plaene.is_empty()
            || relevante_plaene.iter().any(|plan| {
                Some(plan.beginn) != alter_beginn
                    || Some(plan.ende) != altes_ende
                    || plan.beginn_takt_ordinal != schluessel.takt_ordinal
                    || plan.ende_takt_ordinal != schluessel.takt_ordinal
            })
    } else {
        false
    };

    let abweichung = gruppierungsabweichung
        || betroffene_abweichung
        || oktavart_abweichung
        || spannenzuschnitt_abweichung;
    if !abweichung {
        return false;
    }

    let vorher = ausgabe.values().copied().sum::<usize>();
    if alt != nur_sichtbar {
        erhoehe_grund(ausgabe, RegisternotationsAbweichungsgrund::Sichtbarkeitsmenge);
    }
    if gruppierungsabweichung {
        erhoehe_grund(ausgabe, RegisternotationsAbweichungsgrund::Gruppierungsgrenze);
    }
    if betroffene_abweichung {
        erhoehe_grund(
            ausgabe,
            RegisternotationsAbweichungsgrund::BetroffeneEreignismenge,
        );
    }
    if spannenzuschnitt_abweichung {
        erhoehe_grund(ausgabe, RegisternotationsAbweichungsgrund::Spannenzuschnitt);
    }
    if oktavart_abweichung {
        erhoehe_grund(ausgabe, RegisternotationsAbweichungsgrund::Oktavart);
    }
    let sichtbare_midis = gruppe
        .iter()
        .filter(|ereignis| ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt)
        .map(|ereignis| ereignis.tonhoehe_midi)
        .collect::<Vec<_>>();
    if schluessel.system.trim() == "2"
        && unteres_system_schluesselalternative_ist_lesbarer(&sichtbare_midis)
    {
        erhoehe_grund(
            ausgabe,
            RegisternotationsAbweichungsgrund::UnteresSystemSchluesselvorrang,
        );
    }
    ausgabe.values().copied().sum::<usize>() == vorher
}

fn erhoehe_grund(
    ausgabe: &mut BTreeMap<RegisternotationsAbweichungsgrund, usize>,
    grund: RegisternotationsAbweichungsgrund,
) {
    let eintrag = ausgabe.entry(grund).or_default();
    *eintrag = eintrag.saturating_add(1);
}

#[derive(Debug, Clone, Default)]
struct TopologiePruefung {
    leere_betroffene_mengen: usize,
    nullspannen: usize,
    ereignisse_ausserhalb_spanne: usize,
    systemfremde_betroffene_ereignisse: usize,
    widerspruechliche_ueberlappungen: usize,
}

fn pruefe_topologie(
    ereignisse: &[RegisternotationsEreignis],
    plaene: &[RegisterNotationPlan],
) -> TopologiePruefung {
    let index = ereignisse
        .iter()
        .map(|ereignis| (ereignis.id, ereignis))
        .collect::<BTreeMap<_, _>>();
    let mut pruefung = TopologiePruefung::default();
    for plan in plaene {
        if plan.betroffene_ereignisse.is_empty() {
            pruefung.leere_betroffene_mengen = pruefung.leere_betroffene_mengen.saturating_add(1);
        }
        if plan.beginn >= plan.ende {
            pruefung.nullspannen = pruefung.nullspannen.saturating_add(1);
        }
        for id in &plan.betroffene_ereignisse {
            let Some(ereignis) = index.get(id) else {
                pruefung.ereignisse_ausserhalb_spanne =
                    pruefung.ereignisse_ausserhalb_spanne.saturating_add(1);
                continue;
            };
            if ereignis.kanonischer_einsatz < plan.beginn
                || ereignis.kanonisches_ende > plan.ende
            {
                pruefung.ereignisse_ausserhalb_spanne =
                    pruefung.ereignisse_ausserhalb_spanne.saturating_add(1);
            }
            if ereignis.teil != plan.teil
                || ereignis.system != plan.system
                || ereignis.takt_ordinal < plan.beginn_takt_ordinal
                || ereignis.takt_ordinal > plan.ende_takt_ordinal
            {
                pruefung.systemfremde_betroffene_ereignisse =
                    pruefung.systemfremde_betroffene_ereignisse.saturating_add(1);
            }
        }
    }

    for (index_links, links) in plaene.iter().enumerate() {
        for rechts in plaene.iter().skip(index_links.saturating_add(1)) {
            if links.teil != rechts.teil || links.system != rechts.system {
                continue;
            }
            let ueberlappt = links.beginn < rechts.ende && rechts.beginn < links.ende;
            if ueberlappt && links.darstellung != rechts.darstellung {
                pruefung.widerspruechliche_ueberlappungen =
                    pruefung.widerspruechliche_ueberlappungen.saturating_add(1);
            }
        }
    }
    pruefung
}

fn plan_fingerabdruck(plaene: &[RegisterNotationPlan]) -> String {
    let mut text = String::new();
    for plan in plaene {
        let _ = write!(
            text,
            "{}|{}|{}|{}|{}|{}|{}|K:",
            plan.teil,
            plan.beginn_takt_ordinal,
            plan.ende_takt_ordinal,
            plan.system,
            plan.beginn,
            plan.ende,
            plan.darstellung.bezeichnung()
        );
        for id in &plan.entscheidungskontext {
            let _ = write!(
                text,
                "{}:{}:{},",
                id.teil_ordinal, id.takt_ordinal, id.ereignis_ordinal
            );
        }
        text.push_str("|A:");
        for id in &plan.betroffene_ereignisse {
            let _ = write!(
                text,
                "{}:{}:{},",
                id.teil_ordinal, id.takt_ordinal, id.ereignis_ordinal
            );
        }
        text.push_str("|G:");
        for grund in &plan.gruende {
            let _ = write!(text, "{},", grund.text());
        }
        text.push('\n');
    }
    crate::sha256::hex(text.as_bytes())
}

fn checked_lcm(links: i64, rechts: i64) -> Option<i64> {
    let links = links.max(1);
    let rechts = rechts.max(1);
    let ggt = gcd(links, rechts);
    links.checked_div(ggt)?.checked_mul(rechts)
}

fn gcd(mut links: i64, mut rechts: i64) -> i64 {
    links = links.abs().max(1);
    rechts = rechts.abs().max(1);
    while rechts != 0 {
        let rest = links % rechts;
        links = rechts;
        rechts = rest;
    }
    links.max(1)
}

fn skaliere_exakt(wert: i64, skalierung: i64, teilung: i64) -> Option<i128> {
    let teilung = teilung.max(1);
    if skalierung % teilung != 0 {
        return None;
    }
    i128::from(wert).checked_mul(i128::from(skalierung / teilung))
}

pub(crate) fn materialisierungsauftrag() -> Result<RegisternotationsMaterialisierungsauftrag, String> {
    let zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .clone();
    let beweis = zustand
        .beweis
        .ok_or_else(|| "Registernotationsplan vor K00 fehlt".to_string())??;

    if beweis.leere_betroffene_mengen != 0
        || beweis.nullspannen != 0
        || beweis.ereignisse_ausserhalb_spanne != 0
        || beweis.systemfremde_betroffene_ereignisse != 0
        || beweis.widerspruechliche_ueberlappungen != 0
        || beweis.nicht_klassifizierte_abweichungen != 0
    {
        return Err(
            "Registernotationsplan ist topologisch nicht fuer Materialisierung freigegeben"
                .to_string(),
        );
    }

    let fingerprint = plan_fingerabdruck(&beweis.plaene);
    if fingerprint != beweis.plan_fingerabdruck {
        return Err("Registernotations-Planfingerabdruck hat sich nach K00 veraendert".to_string());
    }

    let ereignis_index = beweis
        .ereignisse
        .iter()
        .map(|ereignis| (ereignis.id, ereignis))
        .collect::<BTreeMap<_, _>>();

    let mut geordnet = beweis.plaene.iter().collect::<Vec<_>>();
    geordnet.sort_by_key(|plan| (&plan.teil, &plan.system, plan.beginn, plan.ende));
    for paar in geordnet.windows(2) {
        let links = paar[0];
        let rechts = paar[1];
        if links.teil == rechts.teil
            && links.system == rechts.system
            && links.ende > rechts.beginn
        {
            return Err(format!(
                "Registernotationsplaene ueberlappen im selben System: {} System {}",
                links.teil, links.system
            ));
        }
    }

    let mut materialisierungsplaene = Vec::with_capacity(beweis.plaene.len());
    for plan in &beweis.plaene {
        if plan.darstellung == RegisternotationsDarstellung::Keine {
            return Err("Registernotationsplan ohne darzustellende Oktavart".to_string());
        }
        let betroffene = plan
            .betroffene_ereignisse
            .iter()
            .map(|id| {
                ereignis_index
                    .get(id)
                    .copied()
                    .ok_or_else(|| "betroffenes Registernotationsereignis fehlt".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        if betroffene.is_empty() {
            return Err("Registernotationsplan besitzt keine betroffenen Ereignisse".to_string());
        }
        let kanonischer_beginn = betroffene
            .iter()
            .map(|ereignis| ereignis.kanonischer_einsatz)
            .min()
            .ok_or_else(|| "Registernotationsbeginn fehlt".to_string())?;
        let kanonisches_ende = betroffene
            .iter()
            .map(|ereignis| ereignis.kanonisches_ende)
            .max()
            .ok_or_else(|| "Registernotationsende fehlt".to_string())?;
        if kanonischer_beginn != plan.beginn || kanonisches_ende != plan.ende {
            return Err("Registernotationsspanne weicht von betroffener Ereignismenge ab".to_string());
        }

        let beginn_einsatz = betroffene
            .iter()
            .filter(|ereignis| ereignis.takt_ordinal == plan.beginn_takt_ordinal)
            .map(|ereignis| ereignis.einsatz)
            .min()
            .ok_or_else(|| "lokaler Registernotationsbeginn fehlt".to_string())?;
        let ende_einsatz = betroffene
            .iter()
            .filter(|ereignis| ereignis.takt_ordinal == plan.ende_takt_ordinal)
            .map(|ereignis| {
                ereignis
                    .einsatz
                    .checked_add(ereignis.dauer)
                    .ok_or_else(|| "lokales Registernotationsende ueberlaeuft".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .max()
            .ok_or_else(|| "lokales Registernotationsende fehlt".to_string())?;
        if beginn_einsatz < 0
            || ende_einsatz < 0
            || (plan.beginn_takt_ordinal == plan.ende_takt_ordinal
                && beginn_einsatz >= ende_einsatz)
        {
            return Err("ungueltige lokale Registernotationsspanne".to_string());
        }

        materialisierungsplaene.push(RegisternotationsMaterialisierungsPlan {
            teil: plan.teil.clone(),
            beginn_takt_ordinal: plan.beginn_takt_ordinal,
            ende_takt_ordinal: plan.ende_takt_ordinal,
            system: plan.system.clone(),
            beginn_einsatz,
            ende_einsatz,
            darstellung: plan.darstellung,
            sichtwiederholungen: Vec::new(),
        });
    }

    Ok(RegisternotationsMaterialisierungsauftrag {
        plaene: materialisierungsplaene,
        plan_fingerabdruck: fingerprint,
    })
}


#[derive(Debug, Clone)]
struct SichtmaterialisierungsKandidat {
    plan: RegisternotationsMaterialisierungsPlan,
    kanonischer_beginn: i128,
    kanonisches_ende: i128,
    semantische_teile: usize,
}

pub(crate) fn sichtmaterialisierungsauftrag(
) -> Result<RegisternotationsSichtmaterialisierungsauftrag, String> {
    let semantisch = materialisierungsauftrag()?;
    let zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .clone();
    let beweis = zustand
        .beweis
        .ok_or_else(|| "Registernotationsplan vor Sichtmaterialisierung fehlt".to_string())??;
    if beweis.plaene.len() != semantisch.plaene.len() {
        return Err("Semantik- und Materialisierungsplananzahl weichen ab".to_string());
    }

    let mut nach_system =
        BTreeMap::<(String, String), Vec<SichtmaterialisierungsKandidat>>::new();
    for (fachplan, materialplan) in beweis.plaene.iter().zip(&semantisch.plaene) {
        if fachplan.teil != materialplan.teil
            || fachplan.system != materialplan.system
            || fachplan.beginn_takt_ordinal != materialplan.beginn_takt_ordinal
            || fachplan.ende_takt_ordinal != materialplan.ende_takt_ordinal
            || fachplan.darstellung != materialplan.darstellung
        {
            return Err("Semantik- und Materialisierungsplanidentitaet weichen ab".to_string());
        }
        nach_system
            .entry((materialplan.teil.clone(), materialplan.system.clone()))
            .or_default()
            .push(SichtmaterialisierungsKandidat {
                plan: materialplan.clone(),
                kanonischer_beginn: fachplan.beginn,
                kanonisches_ende: fachplan.ende,
                semantische_teile: 1,
            });
    }

    let mut kontaktfusionen = 0usize;
    let mut nach_kontakt = Vec::<SichtmaterialisierungsKandidat>::new();
    for ((_teil, _system), mut kandidaten) in nach_system {
        kandidaten.sort_by_key(|kandidat| {
            (
                kandidat.kanonischer_beginn,
                kandidat.kanonisches_ende,
                kandidat.plan.darstellung,
            )
        });
        let mut aktuell = None::<SichtmaterialisierungsKandidat>;
        for kandidat in kandidaten {
            if let Some(mut links) = aktuell.take() {
                if links.kanonisches_ende == kandidat.kanonischer_beginn
                    && links.plan.darstellung == kandidat.plan.darstellung
                {
                    links.plan.ende_takt_ordinal = kandidat.plan.ende_takt_ordinal;
                    links.plan.ende_einsatz = kandidat.plan.ende_einsatz;
                    links.kanonisches_ende = kandidat.kanonisches_ende;
                    links.plan
                        .sichtwiederholungen
                        .extend(kandidat.plan.sichtwiederholungen.iter().cloned());
                    links.semantische_teile = links
                        .semantische_teile
                        .saturating_add(kandidat.semantische_teile);
                    kontaktfusionen = kontaktfusionen.saturating_add(1);
                    aktuell = Some(links);
                } else {
                    nach_kontakt.push(links);
                    aktuell = Some(kandidat);
                }
            } else {
                aktuell = Some(kandidat);
            }
        }
        if let Some(aktuell) = aktuell {
            nach_kontakt.push(aktuell);
        }
    }

    let mut nach_system =
        BTreeMap::<(String, String), Vec<SichtmaterialisierungsKandidat>>::new();
    for kandidat in nach_kontakt {
        nach_system
            .entry((kandidat.plan.teil.clone(), kandidat.plan.system.clone()))
            .or_default()
            .push(kandidat);
    }

    let mut ziel = Vec::<SichtmaterialisierungsKandidat>::new();
    let mut stille_bruecken = 0usize;
    for ((_teil, _system), mut kandidaten) in nach_system {
        kandidaten.sort_by_key(|kandidat| {
            (
                kandidat.kanonischer_beginn,
                kandidat.kanonisches_ende,
                kandidat.plan.darstellung,
            )
        });
        let mut aktuell = None::<SichtmaterialisierungsKandidat>;
        for kandidat in kandidaten {
            if let Some(mut links) = aktuell.take() {
                let gleiche_darstellung = links.plan.darstellung == kandidat.plan.darstellung;
                let positive_luecke = links.kanonisches_ende < kandidat.kanonischer_beginn;
                let ton_im_offenen_intervall = beweis.ereignisse.iter().any(|ereignis| {
                    ereignis.teil == links.plan.teil
                        && ereignis.system == links.plan.system
                        && ereignis.kanonischer_einsatz < kandidat.kanonischer_beginn
                        && ereignis.kanonisches_ende > links.kanonisches_ende
                });
                let lokale_grenzrhythmik = if gleiche_darstellung
                    && positive_luecke
                    && !ton_im_offenen_intervall
                {
                    lokale_grenzrhythmik(
                        &beweis.ereignisse,
                        &links.plan.teil,
                        &links.plan.system,
                        links.kanonisches_ende,
                        kandidat.kanonischer_beginn,
                    )
                } else {
                    None
                };
                let luecke = kandidat
                    .kanonischer_beginn
                    .saturating_sub(links.kanonisches_ende);
                let verbindet = lokale_grenzrhythmik
                    .is_some_and(|(links_dauer, rechts_dauer)| {
                        luecke > 0 && luecke <= links_dauer.min(rechts_dauer)
                    });
                if verbindet {
                    links
                        .plan
                        .sichtwiederholungen
                        .push(RegisternotationsSichtwiederholung {
                            takt_ordinal: kandidat.plan.beginn_takt_ordinal,
                            einsatz: kandidat.plan.beginn_einsatz,
                        });
                    links.plan
                        .sichtwiederholungen
                        .extend(kandidat.plan.sichtwiederholungen.iter().cloned());
                    links.plan.ende_takt_ordinal = kandidat.plan.ende_takt_ordinal;
                    links.plan.ende_einsatz = kandidat.plan.ende_einsatz;
                    links.kanonisches_ende = kandidat.kanonisches_ende;
                    links.semantische_teile = links
                        .semantische_teile
                        .saturating_add(kandidat.semantische_teile);
                    stille_bruecken = stille_bruecken.saturating_add(1);
                    aktuell = Some(links);
                } else {
                    ziel.push(links);
                    aktuell = Some(kandidat);
                }
            } else {
                aktuell = Some(kandidat);
            }
        }
        if let Some(aktuell) = aktuell {
            ziel.push(aktuell);
        }
    }

    ziel.sort_by_key(|kandidat| {
        (
            kandidat.plan.teil.clone(),
            kandidat.plan.beginn_takt_ordinal,
            kandidat.plan.beginn_einsatz,
            kandidat.plan.system.clone(),
            kandidat.plan.ende_takt_ordinal,
            kandidat.plan.ende_einsatz,
            kandidat.plan.darstellung,
        )
    });
    let plaene = ziel
        .iter()
        .map(|kandidat| kandidat.plan.clone())
        .collect::<Vec<_>>();
    let sicht_fingerabdruck = sichtplan_fingerabdruck(&plaene);

    Ok(RegisternotationsSichtmaterialisierungsauftrag {
        plaene,
        semantischer_plan_fingerabdruck: semantisch.plan_fingerabdruck,
        sicht_fingerabdruck,
        semantische_plaene: semantisch.plaene.len(),
        kontaktfusionen,
        stille_bruecken,
    })
}

fn lokale_grenzrhythmik(
    ereignisse: &[RegisternotationsEreignis],
    teil: &str,
    system: &str,
    linkes_ende: i128,
    rechter_beginn: i128,
) -> Option<(i128, i128)> {
    let gruppen = rhythmische_gruppen(ereignisse, teil, system);
    let links = gruppen
        .iter()
        .filter(|gruppe| gruppe.2 && gruppe.1 == linkes_ende)
        .map(|gruppe| gruppe.1.saturating_sub(gruppe.0))
        .max()?;
    let rechts = gruppen
        .iter()
        .filter(|gruppe| gruppe.2 && gruppe.0 == rechter_beginn)
        .map(|gruppe| gruppe.1.saturating_sub(gruppe.0))
        .max()?;
    (links > 0 && rechts > 0).then_some((links, rechts))
}

fn rhythmische_gruppen(
    ereignisse: &[RegisternotationsEreignis],
    teil: &str,
    system: &str,
) -> Vec<(i128, i128, bool)> {
    let mut gruppen = BTreeMap::<(String, i128), (i128, bool)>::new();
    for ereignis in ereignisse
        .iter()
        .filter(|ereignis| ereignis.teil == teil && ereignis.system == system)
    {
        let schluessel = (ereignis.stimme.clone(), ereignis.kanonischer_einsatz);
        let eintrag = gruppen
            .entry(schluessel)
            .or_insert((ereignis.kanonischer_einsatz, false));
        if !ereignis.akkordton {
            eintrag.0 = ereignis.kanonisches_ende;
        }
        if ereignis.sichtbarkeit == RegisternotationsSichtbarkeit::Gedruckt {
            eintrag.1 = true;
        }
    }
    gruppen
        .into_iter()
        .filter_map(|((_stimme, beginn), (ende, sichtbar))| {
            (ende > beginn).then_some((beginn, ende, sichtbar))
        })
        .collect()
}

fn sichtplan_fingerabdruck(plaene: &[RegisternotationsMaterialisierungsPlan]) -> String {
    let mut text = String::new();
    for plan in plaene {
        let _ = writeln!(
            text,
            "{}|{}:{}|{}:{}|{}|{}",
            plan.teil,
            plan.beginn_takt_ordinal,
            plan.beginn_einsatz,
            plan.ende_takt_ordinal,
            plan.ende_einsatz,
            plan.system,
            plan.darstellung.bezeichnung(),
        );
        for wiederholung in &plan.sichtwiederholungen {
            let _ = writeln!(
                text,
                "cue|{}:{}",
                wiederholung.takt_ordinal,
                wiederholung.einsatz
            );
        }
    }
    crate::sha256::hex(text.as_bytes())
}

pub(crate) fn beobachte_materialisierung(
    zeugnis: RegisternotationsMaterialisierungsZeugnis,
    carriers: Vec<OttavaCarrier>,
) -> Result<(), String> {
    let sicht = sichtmaterialisierungsauftrag()?;
    let mut zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner());
    let beweis = zustand
        .beweis
        .as_ref()
        .ok_or_else(|| "Registernotationsplan vor Materialisierungszeugnis fehlt".to_string())?
        .as_ref()
        .map_err(|fehler| fehler.clone())?;
    if zeugnis.semantische_plaene != beweis.plaene.len()
        || zeugnis.sichtlinien != sicht.plaene.len()
        || zeugnis.starts != sicht.plaene.len()
        || zeugnis.continues != sicht.stille_bruecken
        || zeugnis.stops != sicht.plaene.len()
        || zeugnis.semantischer_plan_fingerabdruck != beweis.plan_fingerabdruck
        || zeugnis.sicht_fingerabdruck != sicht.sicht_fingerabdruck
    {
        return Err(
            "Registernotationsmaterialisierung entspricht nicht exakt dem bewiesenen Sichtzielbild"
                .to_string(),
        );
    }
    zustand.ottava_kontext = Some(ottava_kontext::MaterializedContext {
        visible: sicht,
        carriers,
    });
    zustand.materialisierung = Some(zeugnis);
    Ok(())
}

pub(crate) fn fuege_ottava_kontext_hinzu(out: &mut String, master: &str, render: Option<&str>) {
    let zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .clone();
    ottava_kontext::append_context(
        out,
        zustand.beweis.as_ref().and_then(|beweis| beweis.as_ref().ok()),
        zustand.ottava_kontext.as_ref(),
        master,
        render,
    );
    reader_policy::append_report(out);
}

pub(crate) fn fuege_eingangsbeobachtung_hinzu(out: &mut String) {
    let zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .clone();
    match zustand.eingang {
        Some(eingang) => eingang.append_report(out),
        None => out.push_str("InputCapture: NOT_OBSERVED.\n"),
    }
    if let Some(Ok(beweis)) = zustand.beweis {
        let _ = writeln!(out, "InputPlanSha256: {}.", beweis.plan_fingerabdruck);
        let _ = writeln!(out, "InputBaselinePlanSha256: {}.", beweis.basis_plan_fingerabdruck);
        gemeinsame_spanne::fuege_bericht_hinzu(out, &beweis.gemeinsame_spannen);
    }
}

pub(crate) fn fuege_berichtsteil_hinzu(out: &mut String) {
    let zustand = sitzung()
        .lock()
        .unwrap_or_else(|vergiftet| vergiftet.into_inner())
        .clone();
    let Some(beweis) = zustand.beweis else {
        return;
    };

    out.push_str("REGISTERNOTATIONS-GRENZENEVIDENZ-UND-ABSCHNITTSBILDUNG\n");
    out.push_str("======================================================\n");
    out.push_str("Grenze: Der seit r262 bewiesene Logikplaner bleibt schreibfrei. r283 hat daraus den allgemeinen Sichtzustandspfad geschlossen; r287 materialisiert ausschliesslich diesen Sichtpfad ueber reale Tonanker und die zentrale StructuralWriteAuthority. Die Fachdomaene besitzt selbst keinen MusicXML-Schreibzugriff; Taktgrenzen bleiben Evidenz und keine automatische Sichtgrenze.\n");
    match beweis {
        Err(fehler) => {
            let _ = writeln!(out, "Planungsbeweis: nicht verfuegbar; Grund {fehler}.");
        }
        Ok(beweis) => {
            let stimmen = beweis
                .ereignisse
                .iter()
                .map(|ereignis| (ereignis.teil.clone(), ereignis.system.clone(), ereignis.stimme.clone()))
                .collect::<BTreeSet<_>>()
                .len();
            let taktbezeichnungen = beweis
                .ereignisse
                .iter()
                .map(|ereignis| (ereignis.teil.clone(), ereignis.taktbezeichnung.clone()))
                .collect::<BTreeSet<_>>()
                .len();
            let akkordtoene = beweis.ereignisse.iter().filter(|ereignis| ereignis.akkordton).count();
            let lokale_zeitfelder = beweis
                .ereignisse
                .iter()
                .filter(|ereignis| ereignis.einsatz >= 0 && ereignis.dauer >= 0)
                .count();
            let taktgrenzen = beweis.grenzen.iter().filter(|grenze| grenze.taktgrenze).count();
            let zaehlzeitgrenzen = beweis
                .grenzen
                .iter()
                .filter(|grenze| grenze.zaehlzeitgrenze)
                .count();
            let stillegrenzen = beweis
                .grenzen
                .iter()
                .filter(|grenze| grenze.stille_dazwischen)
                .count();
            let stillesumme = beweis
                .grenzen
                .iter()
                .map(|grenze| grenze.stille_dauer)
                .sum::<i128>();
            let groesster_tonhoehenabstand = beweis
                .grenzen
                .iter()
                .map(|grenze| grenze.tonhoehenabstand)
                .max()
                .unwrap_or(0);
            let dauerwechsel = beweis
                .grenzen
                .iter()
                .filter(|grenze| grenze.dauerveraenderung)
                .count();
            let eindeutige_grenzen = beweis
                .grenzen
                .iter()
                .filter(|grenze| grenze.links != grenze.rechts)
                .count();
            let _ = writeln!(
                out,
                "Beobachtung: SystemTaktGruppen {}; Tonereignisse {}; Gedruckt {}; VerborgenStrukturell {}; Stimmen {}; Taktbezeichnungen {}; Akkordtoene {}; LokaleZeitfelder {}; Grenzenevidenzen {}; EindeutigeGrenzen {}; Taktgrenzen {}; Zaehlzeitgrenzen {}; Stillegrenzen {}; StilleTicksSumme {}; GroessterTonhoehenabstand {}; Dauerwechsel {}; Altentscheidungen {}; SichtbarkeitsGegenrechnungAenderungen {}.",
                beweis.system_takt_gruppen,
                beweis.ereignisse.len(),
                beweis.gedruckte_ereignisse,
                beweis.verborgene_ereignisse,
                stimmen,
                taktbezeichnungen,
                akkordtoene,
                lokale_zeitfelder,
                beweis.grenzen.len(),
                eindeutige_grenzen,
                taktgrenzen,
                zaehlzeitgrenzen,
                stillegrenzen,
                stillesumme,
                groesster_tonhoehenabstand,
                dauerwechsel,
                beweis.altentscheidungen.len(),
                beweis.sichtbarkeits_gegenrechnungs_aenderungen,
            );
            let _ = writeln!(
                out,
                "Abschnittsbildung: Abschnitte {}; Taktuebergreifend {}; TragendeGrenzen {}; SchwacheGrenzen {}; TragendeTaktgrenzen {}; SchwacheTaktgrenzen {}.",
                beweis.abschnitte,
                beweis.taktuebergreifende_abschnitte,
                beweis.tragende_grenzen,
                beweis.schwache_grenzen,
                beweis.tragende_taktgrenzen,
                beweis.schwache_taktgrenzen,
            );
            let _ = writeln!(
                out,
                "NeuePlanung: Plaene {}; KlingendeZwischeninselKontinuitaetsplaene {}; KurzeRandinselKontinuitaetsplaene {}; LeereBetroffeneMengen {}; Nullspannen {}; EreignisseAusserhalbSpanne {}; SystemfremdeBetroffeneEreignisse {}; WiderspruechlicheUeberlappungen {}; NichtKlassifizierteAbweichungen {}; PlanFingerabdruck {}.",
                beweis.plaene.len(),
                beweis.klingende_zwischeninsel_kontinuitaetsplaene,
                beweis.kurze_randinsel_kontinuitaetsplaene,
                beweis.leere_betroffene_mengen,
                beweis.nullspannen,
                beweis.ereignisse_ausserhalb_spanne,
                beweis.systemfremde_betroffene_ereignisse,
                beweis.widerspruechliche_ueberlappungen,
                beweis.nicht_klassifizierte_abweichungen,
                beweis.plan_fingerabdruck,
            );
            if let Some(zeugnis) = zustand.materialisierung.as_ref() {
                let _ = writeln!(
                    out,
                    "ProduktiveSichtmaterialisierung: SemantischePlaene {}; Sichtlinien {}; Starts {}; Continues {}; Stops {}; SemantischerPlanFingerabdruck {}; SichtFingerabdruck {}; Besitzer RegisterNotationPlan->Sichtzustandspfad->StructuralWriteAuthority; SemantischeRegisterentscheidung unveraendert; SichtCuePolicy stille Bruecke -> continue + darstellungsidentischer sichtbarer Ottava-Re-Cue; finaler HorizontalLayout-Materializer entfernt den zusaetzlichen Words-Re-Cue ausschliesslich am tatsaechlichen new-system/new-page-Wiedereintritt vor dem ersten gepitchten Ereignis desselben Staffs, Scope und Continue bleiben unveraendert.",
                    zeugnis.semantische_plaene,
                    zeugnis.sichtlinien,
                    zeugnis.starts,
                    zeugnis.continues,
                    zeugnis.stops,
                    zeugnis.semantischer_plan_fingerabdruck,
                    zeugnis.sicht_fingerabdruck,
                );
            } else {
                out.push_str("ProduktiveSichtmaterialisierung: nicht beobachtet; fail-closed ausserhalb eines vollstaendigen K00-Laufs.\n");
            }
            for (grund, anzahl) in &beweis.abweichungsgruende {
                let _ = writeln!(out, "Planabweichungsgrund {}: {}.", grund.text(), anzahl);
            }
        }
    }
    out.push_str("Falsifikationsregel: Der Fachplan besitzt selbst keinerlei Schreibrecht. Die produktive Materialisierung ist nur gruen, wenn semantischer Planfingerabdruck, Sichtfingerabdruck sowie Start-/Continue-/Stopanzahl exakt dem geschlossenen Sichtzustandspfad entsprechen; der Writer darf weder Scope noch Registerart neu entscheiden.\n\n");
}

#[cfg(test)]
mod pruefungen {
    use super::*;
    use crate::model::{Event, ExpectedDuration, Measure, Part, Pitch};

    fn note(
        staff: &str,
        voice: &str,
        start: i64,
        duration: i64,
        midi: i32,
        visible: bool,
        chord: bool,
    ) -> Event {
        let octave = midi / 12 - 1;
        let pitch_class = midi.rem_euclid(12);
        let (step, alter) = match pitch_class {
            0 => ("C", 0),
            1 => ("C", 1),
            2 => ("D", 0),
            3 => ("D", 1),
            4 => ("E", 0),
            5 => ("F", 0),
            6 => ("F", 1),
            7 => ("G", 0),
            8 => ("G", 1),
            9 => ("A", 0),
            10 => ("A", 1),
            _ => ("B", 0),
        };
        Event {
            kind: EventKind::Note,
            staff: staff.to_string(),
            voice: voice.to_string(),
            start,
            duration,
            pitch: Some(Pitch::new(step.to_string(), alter, octave)),
            is_chord: chord,
            tie_stop: false,
            visible,
            note_type: None,
            dots: 0,
        }
    }

    fn rest(staff: &str, voice: &str, start: i64, duration: i64) -> Event {
        Event {
            kind: EventKind::Rest,
            staff: staff.to_string(),
            voice: voice.to_string(),
            start,
            duration,
            pitch: None,
            is_chord: false,
            tie_stop: false,
            visible: true,
            note_type: None,
            dots: 0,
        }
    }

    fn measure(number: &str, divisions: i64, beats: i64, beat_type: i64, events: Vec<Event>) -> Measure {
        let expected = ExpectedDuration::new(divisions, beats, beat_type);
        let cursor_max = expected.numerator / expected.denominator;
        Measure {
            number: number.to_string(),
            divisions,
            beats,
            beat_type,
            expected,
            cursor_min: 0,
            cursor_max,
            events,
        }
    }

    fn score(measures: Vec<Measure>) -> Score {
        Score {
            title: Some("Test".to_string()),
            parts: vec![Part {
                id: "P1".to_string(),
                name: "Klavier".to_string(),
                measures,
            }],
        }
    }

    fn four_high_notes(visible: bool) -> Vec<Event> {
        vec![
            note("1", "1", 0, 24, 88, visible, false),
            note("1", "1", 24, 24, 89, visible, false),
            note("1", "1", 48, 24, 90, visible, false),
            note("1", "1", 72, 24, 91, visible, false),
        ]
    }

    fn registerereignis(
        ereignis_ordinal: usize,
        einsatz: i128,
        dauer: i128,
        midi: i32,
    ) -> RegisternotationsEreignis {
        RegisternotationsEreignis {
            id: RegisternotationsEreignisId {
                teil_ordinal: 1,
                takt_ordinal: 1,
                ereignis_ordinal,
            },
            teil: "P1".to_string(),
            system: "1".to_string(),
            stimme: "1".to_string(),
            takt_ordinal: 1,
            taktbezeichnung: "1".to_string(),
            einsatz: i64::try_from(einsatz).unwrap(),
            dauer: i64::try_from(dauer).unwrap(),
            kanonischer_einsatz: einsatz,
            kanonisches_ende: einsatz + dauer,
            tonhoehe_midi: midi,
            sichtbarkeit: RegisternotationsSichtbarkeit::Gedruckt,
            akkordton: false,
            ueberbindungsfortsetzung: false,
        }
    }

    #[test]
    fn score_projektion_beobachtet_nur_tonereignisse() {
        let mut events = four_high_notes(true);
        events.push(rest("1", "1", 96, 24));
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.ereignisse.len(), 4);
    }

    #[test]
    fn ereignisidentitaet_ist_teil_takt_ereignisordinal() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert_eq!(beweis.ereignisse[2].id, RegisternotationsEreignisId { teil_ordinal: 1, takt_ordinal: 1, ereignis_ordinal: 3 });
    }

    #[test]
    fn akkordton_behaelt_semantische_identitaet() {
        let events = vec![
            note("1", "1", 0, 24, 96, true, false),
            note("1", "1", 0, 24, 100, true, true),
            note("1", "1", 24, 24, 98, true, false),
            note("1", "1", 48, 48, 101, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(beweis.ereignisse[1].akkordton);
        assert_eq!(beweis.ereignisse[0].kanonischer_einsatz, beweis.ereignisse[1].kanonischer_einsatz);
    }

    #[test]
    fn gedruckt_und_verborgen_strukturell_bleiben_getrennt() {
        let mut events = four_high_notes(true);
        events[3].visible = false;
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.gedruckte_ereignisse, 3);
        assert_eq!(beweis.verborgene_ereignisse, 1);
    }

    #[test]
    fn system_und_stimme_werden_nicht_verloren() {
        let events = vec![
            note("2", "3", 0, 24, 36, true, false),
            note("2", "3", 24, 24, 35, true, false),
            note("2", "3", 48, 24, 34, true, false),
            note("2", "3", 72, 24, 33, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(beweis.ereignisse.iter().all(|e| e.system == "2" && e.stimme == "3"));
    }

    #[test]
    fn teil_und_taktbezeichnung_werden_semantisch_uebernommen() {
        let beweis = analysiere_score(&score(vec![measure("A", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert!(beweis.ereignisse.iter().all(|e| e.teil == "P1" && e.taktbezeichnung == "A"));
    }

    #[test]
    fn lokaler_einsatz_und_dauer_bleiben_unveraendert() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert_eq!((beweis.ereignisse[1].einsatz, beweis.ereignisse[1].dauer), (24, 24));
    }

    #[test]
    fn tonhoehe_bleibt_klingende_midi_referenz() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert_eq!(beweis.ereignisse.iter().map(|e| e.tonhoehe_midi).collect::<Vec<_>>(), vec![88, 89, 90, 91]);
    }

    #[test]
    fn kanonische_zeit_skaliert_unterschiedliche_divisions_exakt() {
        let eins = measure("1", 24, 4, 4, four_high_notes(true));
        let zwei = measure(
            "2",
            48,
            4,
            4,
            vec![
                note("1", "1", 0, 48, 96, true, false),
                note("1", "1", 48, 48, 98, true, false),
                note("1", "1", 96, 48, 100, true, false),
                note("1", "1", 144, 48, 101, true, false),
            ],
        );
        let beweis = analysiere_score(&score(vec![eins, zwei])).unwrap();
        assert_eq!(beweis.ereignisse[1].kanonisches_ende - beweis.ereignisse[1].kanonischer_einsatz, 48);
        assert_eq!(beweis.ereignisse[5].kanonisches_ende - beweis.ereignisse[5].kanonischer_einsatz, 48);
    }

    #[test]
    fn kanonische_zeit_ist_taktuebergreifend_monoton() {
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, four_high_notes(true)),
            measure("2", 24, 4, 4, four_high_notes(true)),
        ])).unwrap();
        assert!(beweis.ereignisse[4].kanonischer_einsatz > beweis.ereignisse[3].kanonischer_einsatz);
        assert!(beweis.grenzen.iter().any(|grenze| grenze.taktgrenze));
    }

    #[test]
    fn zaehlzeitgrenze_wird_aus_kanonischer_metrik_erkannt() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert!(beweis.grenzen.iter().all(|g| g.zaehlzeitgrenze));
    }

    #[test]
    fn stille_wird_als_positive_zeitluecke_erkannt() {
        let events = vec![
            note("1", "1", 0, 24, 96, true, false),
            note("1", "1", 48, 24, 98, true, false),
            note("1", "1", 72, 24, 100, true, false),
            note("1", "1", 96, 24, 101, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 5, 4, events)])).unwrap();
        assert!(beweis.grenzen.iter().any(|g| g.stille_dazwischen && g.stille_dauer > 0));
    }

    #[test]
    fn tonhoehenabstand_wird_ohne_phrasebehauptung_beobachtet() {
        let events = vec![
            note("1", "1", 0, 24, 60, true, false),
            note("1", "1", 24, 24, 72, true, false),
            note("1", "1", 48, 24, 74, true, false),
            note("1", "1", 72, 24, 76, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.grenzen[0].tonhoehenabstand, 12);
    }

    #[test]
    fn sichtbarer_registerbruch_wird_nicht_durch_verborgene_akkordtoene_maskiert() {
        let events = vec![
            note("1", "1", 0, 24, 90, true, false),
            note("1", "1", 0, 24, 78, false, true),
            note("1", "1", 24, 24, 74, true, false),
            note("1", "1", 24, 24, 77, false, true),
            note("1", "1", 48, 24, 74, true, false),
            note("1", "1", 72, 24, 74, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.grenzen[0].tonhoehenabstand, 16);
        assert_eq!(
            beweis.grenzen[0].staerke,
            RegisternotationsGrenzenStaerke::Tragend
        );
    }

    #[test]
    fn unteres_system_behaelt_strukturelle_grenztonhoehen() {
        let events = vec![
            note("2", "2", 0, 24, 90, true, false),
            note("2", "2", 0, 24, 78, false, true),
            note("2", "2", 24, 24, 74, true, false),
            note("2", "2", 24, 24, 77, false, true),
            note("2", "2", 48, 24, 74, true, false),
            note("2", "2", 72, 24, 74, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.grenzen[0].tonhoehenabstand, 1);
        assert_eq!(
            beweis.grenzen[0].staerke,
            RegisternotationsGrenzenStaerke::Schwach
        );
    }

    #[test]
    fn dauerveraenderung_wird_als_evidenz_beobachtet() {
        let events = vec![
            note("1", "1", 0, 12, 96, true, false),
            note("1", "1", 12, 24, 98, true, false),
            note("1", "1", 36, 24, 100, true, false),
            note("1", "1", 60, 36, 101, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(beweis.grenzen.iter().any(|g| g.dauerveraenderung));
    }

    #[test]
    fn alte_achtva_entscheidung_wird_reproduziert() {
        assert_eq!(
            entscheide_oktavdarstellung(&[88, 89, 90, 91], 4, "1"),
            RegisternotationsDarstellung::Achtva
        );
    }

    #[test]
    fn alte_achtvb_entscheidung_wird_reproduziert() {
        assert_eq!(
            entscheide_oktavdarstellung(&[24, 26, 28, 29], 4, "2"),
            RegisternotationsDarstellung::Achtvb
        );
    }

    #[test]
    fn doppelte_oktavierung_bleibt_reserve_kandidat() {
        assert_eq!(
            entscheide_oktavdarstellung(&[120, 121, 122, 123], 4, "1"),
            RegisternotationsDarstellung::Fuenfzehnma
        );
    }

    #[test]
    fn vier_tonobjekte_bleiben_harte_elegibilitaetsgrenze() {
        assert_eq!(
            entscheide_oktavdarstellung(&[88, 89, 90], 3, "1"),
            RegisternotationsDarstellung::Keine
        );
    }

    #[test]
    fn unteres_system_nutzt_lesbaren_violinschluesselzustand_statt_achtva() {
        assert_eq!(
            entscheide_oktavdarstellung(&[68, 67, 67, 64], 4, "2"),
            RegisternotationsDarstellung::Keine
        );
    }

    #[test]
    fn unteres_system_darf_extrem_hohes_material_trotz_schluesselalternative_oktavieren() {
        assert_eq!(
            entscheide_oktavdarstellung(&[96, 97, 98, 99], 4, "2"),
            RegisternotationsDarstellung::Achtva
        );
    }

    #[test]
    fn unteres_system_waehlt_schluesselzustand_fuer_das_gesamte_fenster() {
        assert_eq!(lesbarkeitswert([48, 72, 48, 72], "2"), 312);
        assert_ne!(lesbarkeitswert([48, 72, 48, 72], "2"), 0);
    }

    #[test]
    fn optimale_spanne_erlaubt_keinen_kostenlosen_schluesselwechsel_pro_ton() {
        let mut ereignisse = [48, 72, 48, 72]
            .iter()
            .enumerate()
            .map(|(index, midi)| {
                registerereignis(index + 1, i128::try_from(index).unwrap() * 24, 24, *midi)
            })
            .collect::<Vec<_>>();
        for ereignis in &mut ereignisse {
            ereignis.system = "2".to_string();
            ereignis.stimme = "2".to_string();
        }
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "2".to_string(),
        };
        let plan = plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).unwrap();
        assert_eq!(plan.darstellung, RegisternotationsDarstellung::Achtva);
    }

    #[test]
    fn optimale_spanne_oktaviert_e8_t31_artiges_unteres_material_nicht() {
        let mut ereignisse = [68, 67, 67, 64]
            .iter()
            .enumerate()
            .map(|(index, midi)| {
                registerereignis(index + 1, i128::try_from(index).unwrap() * 24, 24, *midi)
            })
            .collect::<Vec<_>>();
        for ereignis in &mut ereignisse {
            ereignis.system = "2".to_string();
            ereignis.stimme = "2".to_string();
        }
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "2".to_string(),
        };
        assert!(plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).is_none());
    }

    #[test]
    fn sichtbare_last_ist_von_struktureller_mindestmenge_getrennt() {
        let entscheidung = entscheide_oktavdarstellung(&[88, 89, 90], 4, "1");
        assert_eq!(entscheidung, RegisternotationsDarstellung::Achtva);
    }

    #[test]
    fn klingende_zwischeninsel_verwendet_unveraenderte_registerentscheidung() {
        assert_eq!(lesbarkeitswert([95], "1"), 132);
        assert_eq!(
            entscheide_oktavdarstellung(&[95], MINDEST_TONOBJEKTE, "1"),
            RegisternotationsDarstellung::Achtva
        );
    }

    #[test]
    fn ein_angriff_zwischen_achtva_und_fuenfzehnma_bleibt_bei_guenstigerer_achtva() {
        let ereignis = registerereignis(1, 0, 24, 95);
        let ereignisse = vec![&ereignis];
        assert_eq!(
            waehle_klingende_zwischeninsel_darstellung(
                &ereignisse,
                RegisternotationsDarstellung::Achtva,
                RegisternotationsDarstellung::Fuenfzehnma,
                "1",
            ),
            Some(RegisternotationsDarstellung::Achtva)
        );
    }

    #[test]
    fn klingende_fuenfzehnma_zwischeninsel_darf_dieselbe_darstellung_fortsetzen() {
        let ereignisse = [
            registerereignis(1, 0, 24, 95),
            registerereignis(2, 24, 24, 99),
            registerereignis(3, 48, 24, 102),
            registerereignis(4, 72, 24, 107),
        ];
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        assert!(sichtbare_luecke_ist_durchgehend_klingend(
            &referenzen,
            0,
            96
        ));
        assert_eq!(
            waehle_klingende_zwischeninsel_darstellung(
                &referenzen,
                RegisternotationsDarstellung::Fuenfzehnma,
                RegisternotationsDarstellung::Fuenfzehnma,
                "1",
            ),
            Some(RegisternotationsDarstellung::Fuenfzehnma)
        );
    }

    #[test]
    fn lesbare_loco_zwischeninsel_bleibt_unveraendert() {
        let ereignis = registerereignis(1, 0, 24, 75);
        let ereignisse = vec![&ereignis];
        assert_eq!(
            waehle_klingende_zwischeninsel_darstellung(
                &ereignisse,
                RegisternotationsDarstellung::Achtva,
                RegisternotationsDarstellung::Achtva,
                "1",
            ),
            None
        );
    }

    #[test]
    fn sichtbare_stille_bleibt_harte_kontinuitaetsgrenze() {
        let links = registerereignis(1, 0, 24, 95);
        let rechts = registerereignis(2, 48, 24, 99);
        let ereignisse = vec![&links, &rechts];
        assert!(!sichtbare_luecke_ist_durchgehend_klingend(
            &ereignisse,
            0,
            72
        ));
    }

    #[test]
    fn optimale_zusammenhaengende_spanne_findet_hohe_insel_ohne_loco_raender() {
        let midis = [60, 61, 62, 63, 88, 89, 90, 91, 60, 61, 62, 63];
        let ereignisse = midis
            .iter()
            .enumerate()
            .map(|(index, midi)| {
                registerereignis(index + 1, i128::try_from(index).unwrap() * 24, 24, *midi)
            })
            .collect::<Vec<_>>();
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        assert!(plane_segment(&schluessel, &referenzen, &[]).is_none());
        let plan = plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).unwrap();
        assert_eq!(plan.beginn, 96);
        assert_eq!(plan.ende, 192);
        assert_eq!(plan.darstellung, RegisternotationsDarstellung::Achtva);
        assert!(plan
            .gruende
            .contains(&RegisterPlanGrund::LokaleLesbarkeitsspanne));
    }

    #[test]
    fn optimale_zusammenhaengende_spanne_behaelt_durchgehend_hohen_abschnitt_ganz() {
        let ereignisse = (0..8)
            .map(|index| {
                registerereignis(
                    index + 1,
                    i128::try_from(index).unwrap() * 24,
                    24,
                    88 + i32::try_from(index).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        let plan = plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).unwrap();
        assert_eq!(plan.beginn, 0);
        assert_eq!(plan.ende, 192);
        assert!(!plan
            .gruende
            .contains(&RegisterPlanGrund::LokaleLesbarkeitsspanne));
    }

    #[test]
    fn pro_evidenzsegment_entsteht_hoechstens_eine_lokale_oktavspanne() {
        let midis = [88, 89, 90, 91, 60, 61, 62, 63, 92, 93, 94, 95];
        let ereignisse = midis
            .iter()
            .enumerate()
            .map(|(index, midi)| {
                registerereignis(index + 1, i128::try_from(index).unwrap() * 24, 24, *midi)
            })
            .collect::<Vec<_>>();
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        let plan = plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).unwrap();
        assert_eq!(plan.beginn, 192);
        assert_eq!(plan.ende, 288);
        assert_eq!(plan.darstellung, RegisternotationsDarstellung::Achtva);
    }

    #[test]
    fn zwei_extreme_einzeltoene_erzeugen_keine_randinsel_sonderregel() {
        let ereignisse = [
            registerereignis(1, 0, 24, 108),
            registerereignis(2, 24, 24, 110),
        ];
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        assert!(plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).is_none());
    }

    #[test]
    fn sicherer_schnitt_darf_keinen_hineinragenden_ton_zerschneiden() {
        let ereignisse = [
            registerereignis(1, 0, 120, 60),
            registerereignis(2, 24, 24, 88),
            registerereignis(3, 48, 24, 89),
            registerereignis(4, 72, 24, 90),
            registerereignis(5, 96, 24, 91),
        ];
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schnitte = lokale_sichere_schnittindizes(&referenzen);
        assert_eq!(schnitte, vec![0, 5]);
    }

    #[test]
    fn sichtanker_stop_folgt_dem_chordhost_und_nicht_einem_laengeren_chordmember() {
        let mut ereignisse = [
            registerereignis(1, 0, 24, 88),
            registerereignis(2, 24, 24, 89),
            registerereignis(3, 48, 24, 90),
            registerereignis(4, 72, 24, 91),
            registerereignis(5, 72, 48, 95),
        ];
        ereignisse[4].akkordton = true;
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let (beginne, enden) = materialisierbare_sichtgrenzen(&referenzen);
        assert!(beginne.contains(&72));
        assert!(enden.contains(&96));
        assert!(!enden.contains(&120));
    }

    #[test]
    fn optimale_spanne_waehlt_keinen_nicht_materialisierbaren_chordmember_stop() {
        let mut ereignisse = [
            registerereignis(1, 0, 24, 88),
            registerereignis(2, 24, 24, 89),
            registerereignis(3, 48, 24, 90),
            registerereignis(4, 72, 24, 91),
            registerereignis(5, 72, 48, 95),
        ];
        ereignisse[4].akkordton = true;
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        assert!(
            plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).is_none()
        );
    }

    #[test]
    fn optimale_spanne_behaelt_materialisierbaren_chordhost_stop() {
        let mut ereignisse = [
            registerereignis(1, 0, 24, 88),
            registerereignis(2, 24, 24, 89),
            registerereignis(3, 48, 24, 90),
            registerereignis(4, 72, 24, 91),
            registerereignis(5, 72, 24, 95),
        ];
        ereignisse[4].akkordton = true;
        let referenzen = ereignisse.iter().collect::<Vec<_>>();
        let schluessel = SystemFolgeSchluessel {
            teil: "P1".to_string(),
            system: "1".to_string(),
        };
        let plan =
            plane_segment_optimal_zusammenhaengend(&schluessel, &referenzen, &[]).unwrap();
        assert_eq!(plan.beginn, 0);
        assert_eq!(plan.ende, 96);
        assert_eq!(plan.darstellung, RegisternotationsDarstellung::Achtva);
    }

    #[test]
    fn entscheidungskontext_und_betroffene_menge_duerfen_verschieden_sein() {
        let mut events = four_high_notes(true);
        events[0].visible = false;
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        let plan = &beweis.plaene[0];
        assert!(plan.entscheidungskontext.len() > plan.betroffene_ereignisse.len());
    }

    #[test]
    fn plan_spannt_exakt_die_betroffene_ereignismenge() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        let plan = &beweis.plaene[0];
        assert_eq!(plan.beginn, beweis.ereignisse[0].kanonischer_einsatz);
        assert_eq!(plan.ende, beweis.ereignisse[3].kanonisches_ende);
    }

    #[test]
    fn ganze_zaehlzeit_stille_teilt_strukturell_getrennte_abschnitte() {
        let events = vec![
            note("1", "1", 0, 12, 96, true, false),
            note("1", "1", 12, 12, 98, true, false),
            note("1", "1", 48, 12, 100, true, false),
            note("1", "1", 60, 12, 101, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(beweis
            .grenzen
            .iter()
            .any(|grenze| grenze.staerke == RegisternotationsGrenzenStaerke::Tragend));
        assert!(beweis.abweichungsgruende.contains_key(&RegisternotationsAbweichungsgrund::Gruppierungsgrenze));
    }

    #[test]
    fn taktgrenze_allein_bleibt_schwach() {
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, four_high_notes(true)),
            measure("2", 24, 4, 4, four_high_notes(true)),
        ]))
        .unwrap();
        let grenze = beweis.grenzen.iter().find(|grenze| grenze.taktgrenze).unwrap();
        assert_eq!(grenze.staerke, RegisternotationsGrenzenStaerke::Schwach);
    }

    #[test]
    fn schwache_taktgrenze_erlaubt_taktuebergreifenden_abschnitt() {
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, four_high_notes(true)),
            measure("2", 24, 4, 4, four_high_notes(true)),
        ]))
        .unwrap();
        assert_eq!(beweis.abschnitte, 1);
        assert_eq!(beweis.taktuebergreifende_abschnitte, 1);
        assert!(beweis.plaene.iter().any(|plan| {
            plan.beginn_takt_ordinal == 1 && plan.ende_takt_ordinal == 2
        }));
    }

    #[test]
    fn oktavsprung_ist_tragende_grenze() {
        let events = vec![
            note("1", "1", 0, 24, 72, true, false),
            note("1", "1", 24, 24, 84, true, false),
            note("1", "1", 48, 24, 85, true, false),
            note("1", "1", 72, 24, 86, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.grenzen[0].tonhoehenabstand, 12);
        assert_eq!(
            beweis.grenzen[0].staerke,
            RegisternotationsGrenzenStaerke::Tragend
        );
    }

    #[test]
    fn kurze_stille_allein_bleibt_schwach() {
        let events = vec![
            note("1", "1", 0, 12, 88, true, false),
            note("1", "1", 18, 12, 89, true, false),
            note("1", "1", 30, 12, 90, true, false),
            note("1", "1", 42, 54, 91, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(beweis.grenzen[0].stille_dazwischen);
        assert_eq!(beweis.grenzen[0].stille_dauer, 6);
        assert_eq!(
            beweis.grenzen[0].staerke,
            RegisternotationsGrenzenStaerke::Schwach
        );
    }

    #[test]
    fn taktgrenze_mit_dauerwechsel_wird_tragend() {
        let links = vec![
            note("1", "1", 0, 24, 88, true, false),
            note("1", "1", 24, 24, 89, true, false),
            note("1", "1", 48, 24, 90, true, false),
            note("1", "1", 72, 24, 91, true, false),
        ];
        let rechts = vec![
            note("1", "1", 0, 12, 91, true, false),
            note("1", "1", 12, 12, 90, true, false),
            note("1", "1", 24, 24, 89, true, false),
            note("1", "1", 48, 48, 88, true, false),
        ];
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, links),
            measure("2", 24, 4, 4, rechts),
        ]))
        .unwrap();
        let grenze = beweis.grenzen.iter().find(|grenze| grenze.taktgrenze).unwrap();
        assert!(grenze.dauerveraenderung);
        assert_eq!(grenze.staerke, RegisternotationsGrenzenStaerke::Tragend);
    }

    #[test]
    fn ueberlappende_angriffe_werden_nicht_als_tragende_grenze_getrennt() {
        let events = vec![
            note("1", "1", 0, 48, 72, true, false),
            note("1", "2", 24, 24, 84, true, false),
            note("1", "1", 48, 24, 85, true, false),
            note("1", "1", 72, 24, 86, true, false),
        ];
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert_eq!(beweis.grenzen[0].tonhoehenabstand, 12);
        assert_eq!(
            beweis.grenzen[0].staerke,
            RegisternotationsGrenzenStaerke::Schwach
        );
    }

    #[test]
    fn taktuebergreifender_plan_bleibt_topologisch_geschlossen() {
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, four_high_notes(true)),
            measure("2", 24, 4, 4, four_high_notes(true)),
        ]))
        .unwrap();
        assert_eq!(beweis.systemfremde_betroffene_ereignisse, 0);
        assert_eq!(beweis.widerspruechliche_ueberlappungen, 0);
        assert_eq!(beweis.nullspannen, 0);
    }

    #[test]
    fn taktuebergreifender_plan_markiert_ueberbrueckte_taktgrenze() {
        let beweis = analysiere_score(&score(vec![
            measure("1", 24, 4, 4, four_high_notes(true)),
            measure("2", 24, 4, 4, four_high_notes(true)),
        ]))
        .unwrap();
        assert!(beweis.plaene[0]
            .gruende
            .contains(&RegisterPlanGrund::SchwacheTaktgrenzeUeberbrueckt));
    }

    #[test]
    fn neue_planung_hat_geschlossene_topologie() {
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, four_high_notes(true))])).unwrap();
        assert_eq!(
            (
                beweis.leere_betroffene_mengen,
                beweis.nullspannen,
                beweis.ereignisse_ausserhalb_spanne,
                beweis.systemfremde_betroffene_ereignisse,
                beweis.widerspruechliche_ueberlappungen,
            ),
            (0, 0, 0, 0, 0)
        );
    }

    #[test]
    fn planung_und_fingerabdruck_sind_deterministisch() {
        let s = score(vec![measure("1", 24, 4, 4, four_high_notes(true))]);
        let eins = analysiere_score(&s).unwrap();
        let zwei = analysiere_score(&s).unwrap();
        assert_eq!(eins.plaene, zwei.plaene);
        assert_eq!(eins.plan_fingerabdruck, zwei.plan_fingerabdruck);
    }

    #[test]
    fn abweichung_bekommt_maschinenlesbaren_grund() {
        let mut events = four_high_notes(true);
        events[3].visible = false;
        events[3].pitch = Some(Pitch::new("C".to_string(), 0, 9));
        let beweis = analysiere_score(&score(vec![measure("1", 24, 4, 4, events)])).unwrap();
        assert!(!beweis.abweichungsgruende.is_empty());
    }

    #[test]
    fn beobachtung_hat_nur_unveraenderlichen_text_eingang() {
        let beobachter: fn(&str) = beobachte_vor_k00;
        let _ = beobachter;
    }

    #[test]
    fn keine_etueden_takt_oder_positionssonderregel_im_fachmodell() {
        let quelle = include_str!("mod.rs");
        let produktivteil = quelle.split("#[cfg(test)]").next().unwrap_or(quelle);
        assert!(!produktivteil.contains("Etuede Nr."));
        assert!(!produktivteil.contains("Etüde Nr."));
        assert!(!produktivteil.contains("Takt 22"));
        assert!(!produktivteil.contains("Takt 29"));
        assert!(!produktivteil.contains("Takt 30"));
    }
    #[test]
    fn eingehende_ueberbindung_verweigert_geplanten_oktavbeginn() {
        for (label, divisions) in [("Alpha", 24), ("Renamed", 96)] {
            let mut events = four_high_notes(true);
            for event in &mut events { event.start *= divisions / 24; event.duration *= divisions / 24; }
            let baseline = analysiere_score(&score(vec![measure(label, divisions, 4, 4, events.clone())])).unwrap();
            assert!(!baseline.plaene.is_empty());
            events[0].tie_stop = true;
            let guarded = analysiere_score(&score(vec![measure(label, divisions, 4, 4, events)])).unwrap();
            assert!(guarded.ereignisse[0].ueberbindungsfortsetzung);
            assert!(guarded.plaene.is_empty());
            assert_eq!(guarded.nicht_klassifizierte_abweichungen, 0);
        }
    }

    #[test]
    fn naechster_takt_schuetzt_verdeckte_ueberbindung_am_oktavende() {
        let first = measure("First", 24, 4, 4, four_high_notes(true));
        let next = measure("Next", 24, 4, 4, vec![note("1", "9001", 0, 96, 36, true, false), note("1", "9001", 0, 24, 48, false, true)]);
        let baseline = analysiere_score(&score(vec![first.clone(), next.clone()])).unwrap();
        let boundary = baseline.ereignisse.iter().find(|e| e.takt_ordinal == 2).unwrap().kanonischer_einsatz;
        assert!(baseline.plaene.iter().any(|plan| plan.ende == boundary));
        let mut continuation = next; continuation.events[1].tie_stop = true;
        let guarded = analysiere_score(&score(vec![first, continuation])).unwrap();
        assert!(guarded.plaene.iter().all(|plan| plan.beginn != boundary && plan.ende != boundary));
    }

    #[test]
    fn ueberbindung_in_fremdem_system_oder_teil_verweigert_keinen_plan() {
        let source = score(vec![measure("Base", 24, 4, 4, four_high_notes(true))]);
        let baseline = analysiere_score(&source).unwrap();
        let mut foreign = source.clone();
        let mut other = note("2", "9001", 0, 96, 48, false, false); other.tie_stop = true;
        foreign.parts[0].measures[0].events.push(other);
        let mut second = source.parts[0].clone(); second.id = "Independent".into();
        second.measures[0].events[0].tie_stop = true;
        foreign.parts.push(second.clone()); foreign.parts.push({ second.id = "DuplicateIndependent".into(); second });
        let guarded = analysiere_score(&foreign).unwrap();
        let local = guarded.plaene.into_iter().filter(|p| p.teil == "P1" && p.system == "1").collect::<Vec<_>>();
        assert_eq!(local, baseline.plaene);
    }

    #[test]
    fn konstante_oktavspanne_darf_interne_ueberbindung_enthalten() {
        let mut events = four_high_notes(true);
        let baseline = analysiere_score(&score(vec![measure("Base", 24, 4, 4, events.clone())])).unwrap();
        events[1].pitch = events[0].pitch.clone(); events[1].tie_stop = true;
        let guarded = analysiere_score(&score(vec![measure("Base", 24, 4, 4, events)])).unwrap();
        assert!(!guarded.plaene.is_empty());
        assert_eq!(guarded.plaene.iter().map(|p| (p.beginn, p.ende, p.darstellung)).collect::<Vec<_>>(),
            baseline.plaene.iter().map(|p| (p.beginn, p.ende, p.darstellung)).collect::<Vec<_>>());
    }

    #[test]
    fn geschuetzte_grenzen_bleiben_bei_zeitversetzung_und_scope_umbenennung_geschuetzt() {
        let basis = analysiere_score(&score(vec![measure("Synthetic", 24, 4, 4, four_high_notes(true))])).unwrap();
        let original = basis.plaene.first().unwrap();
        for (verschiebung, faktor, teil, system) in [
            (0, 1, "Local", "Upper"),
            (480, 4, "Renamed", "Lower"),
        ] {
            let mut plan = original.clone();
            plan.teil = teil.into(); plan.system = system.into();
            plan.beginn = original.beginn * faktor + verschiebung;
            plan.ende = original.ende * faktor + verschiebung;
            for grenze in [plan.beginn, plan.ende] {
                let mut ereignis = registerereignis(1, grenze, faktor, 96);
                ereignis.teil = teil.into(); ereignis.system = system.into();
                ereignis.ueberbindungsfortsetzung = true;
                assert!(!oktavspanne_hat_ueberbindungsfreie_grenzen(&plan, &[ereignis.clone()]));
                let mut unabhaengig = ereignis.clone(); unabhaengig.teil = "Independent".into();
                assert!(!oktavspanne_hat_ueberbindungsfreie_grenzen(&plan, &[ereignis, unabhaengig.clone()]));
                assert!(oktavspanne_hat_ueberbindungsfreie_grenzen(&plan, &[unabhaengig]));
            }
        }
    }

    #[test]
    fn eingangsbeobachtung_bindet_plan_und_verwirft_alten_sitzungszustand() {
        let _sperre = sperre_fuer_tests();
        let note = "<note><pitch><step>C</step><octave>7</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>";
        let xml = format!("<score-partwise><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"SyntheticAlpha\"><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}</measure></part></score-partwise>", note.repeat(4));
        let direkt = analysiere_score(&crate::musicxml::parse_musicxml(&xml).unwrap()).unwrap();
        assert!(!direkt.plaene.is_empty());
        loesche();
        beobachte_vor_k00(&xml);
        let vorher = format!("{:?}", materialisierungsauftrag().unwrap());
        let mut out = String::new();
        fuege_eingangsbeobachtung_hinzu(&mut out);
        assert!(out.contains(&format!("InputSha256: {}.", crate::sha256::hex(xml.as_bytes()))));
        assert!(out.contains(&format!("InputPlanSha256: {}.", direkt.plan_fingerabdruck)));
        assert_eq!(vorher, format!("{:?}", materialisierungsauftrag().unwrap()));
        beobachte_vor_k00("<broken");
        out.clear();
        fuege_eingangsbeobachtung_hinzu(&mut out);
        assert!(out.contains(&format!("InputSha256: {}.", crate::sha256::hex(b"<broken"))));
        assert!(!out.contains("InputPlanSha256:"));
        loesche();
        out.clear();
        fuege_eingangsbeobachtung_hinzu(&mut out);
        assert_eq!(out, "InputCapture: NOT_OBSERVED.\n");
    }

}
