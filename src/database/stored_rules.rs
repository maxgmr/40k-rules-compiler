use serde::Deserialize;
use serde_json;
use std::fs;
use std::io::Result;
use std::slice::Iter;

const JSON_PATH: &'static str = "./files/rules.json";

pub struct Rule {
    name: String,
    description: String,
    phase: Phase,
    turn: Turn,
    conditions: Vec<String>,
}

impl Rule {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn get_phase(&self) -> &Phase {
        &self.phase
    }
    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }
    pub fn get_conditions(&self) -> &Vec<String> {
        &self.conditions
    }
}

#[derive(Debug)]
pub enum Phase {
    BeforeGame,
    CommandStart,
    Command,
    CommandEnd,
    MovementStart,
    Movement,
    Reinforcements,
    MovementEnd,
    PsychicStart,
    PsychicTest,
    PsychicDeny,
    PsychicDamage,
    PsychicEnd,
    ShootingStart,
    ShootingTargets,
    ShootingHits,
    ShootingWounds,
    ShootingSaves,
    ShootingDamage,
    ShootingEnd,
    ChargeStart,
    Charge,
    HeroicIntervention,
    ChargeEnd,
    FightStart,
    FightPileIn,
    FightTargets,
    FightHits,
    FightWounds,
    FightSaves,
    FightDamage,
    FightConsolidate,
    FightEnd,
    MoraleStart,
    Morale,
    CombatAttrition,
    UnitCoherency,
    MoraleEnd,
    TurnEnd,
}

impl Phase {
    pub fn name(value: i32) -> Option<Phase> {
        match value {
            0 => Some(Phase::BeforeGame),
            1 => Some(Phase::CommandStart),
            2 => Some(Phase::Command),
            3 => Some(Phase::CommandEnd),
            4 => Some(Phase::MovementStart),
            5 => Some(Phase::Movement),
            6 => Some(Phase::Reinforcements),
            7 => Some(Phase::MovementEnd),
            8 => Some(Phase::PsychicStart),
            9 => Some(Phase::PsychicTest),
            10 => Some(Phase::PsychicDeny),
            11 => Some(Phase::PsychicDamage),
            12 => Some(Phase::PsychicEnd),
            13 => Some(Phase::ShootingStart),
            14 => Some(Phase::ShootingTargets),
            15 => Some(Phase::ShootingHits),
            16 => Some(Phase::ShootingWounds),
            17 => Some(Phase::ShootingSaves),
            18 => Some(Phase::ShootingDamage),
            19 => Some(Phase::ShootingEnd),
            20 => Some(Phase::ChargeStart),
            21 => Some(Phase::Charge),
            22 => Some(Phase::HeroicIntervention),
            23 => Some(Phase::ChargeEnd),
            24 => Some(Phase::FightStart),
            25 => Some(Phase::FightPileIn),
            26 => Some(Phase::FightTargets),
            27 => Some(Phase::FightHits),
            28 => Some(Phase::FightWounds),
            29 => Some(Phase::FightSaves),
            30 => Some(Phase::FightDamage),
            31 => Some(Phase::FightConsolidate),
            32 => Some(Phase::FightEnd),
            33 => Some(Phase::MoraleStart),
            34 => Some(Phase::Morale),
            35 => Some(Phase::CombatAttrition),
            36 => Some(Phase::UnitCoherency),
            37 => Some(Phase::MoraleEnd),
            38 => Some(Phase::TurnEnd),
            _ => None,
        }
    }
    pub fn value(&self) -> i32 {
        match *self {
            Phase::BeforeGame => 0,
            Phase::CommandStart => 1,
            Phase::Command => 2,
            Phase::CommandEnd => 3,
            Phase::MovementStart => 4,
            Phase::Movement => 5,
            Phase::Reinforcements => 6,
            Phase::MovementEnd => 7,
            Phase::PsychicStart => 8,
            Phase::PsychicTest => 9,
            Phase::PsychicDeny => 10,
            Phase::PsychicDamage => 11,
            Phase::PsychicEnd => 12,
            Phase::ShootingStart => 13,
            Phase::ShootingTargets => 14,
            Phase::ShootingHits => 15,
            Phase::ShootingWounds => 16,
            Phase::ShootingSaves => 17,
            Phase::ShootingDamage => 18,
            Phase::ShootingEnd => 19,
            Phase::ChargeStart => 20,
            Phase::Charge => 21,
            Phase::HeroicIntervention => 22,
            Phase::ChargeEnd => 23,
            Phase::FightStart => 24,
            Phase::FightPileIn => 25,
            Phase::FightTargets => 26,
            Phase::FightHits => 27,
            Phase::FightWounds => 28,
            Phase::FightSaves => 29,
            Phase::FightDamage => 30,
            Phase::FightConsolidate => 31,
            Phase::FightEnd => 32,
            Phase::MoraleStart => 33,
            Phase::Morale => 34,
            Phase::CombatAttrition => 35,
            Phase::UnitCoherency => 36,
            Phase::MoraleEnd => 37,
            Phase::TurnEnd => 38,
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    Friendly,
    Opponent,
    Both,
}

impl Turn {
    pub fn name(value: i32) -> Option<Turn> {
        match value {
            0 => Some(Turn::Friendly),
            1 => Some(Turn::Opponent),
            2 => Some(Turn::Both),
            _ => None,
        }
    }
    pub fn value(&self) -> i32 {
        match *self {
            Turn::Friendly => 0,
            Turn::Opponent => 1,
            Turn::Both => 2,
        }
    }
}

#[derive(Deserialize, Debug)]
struct ValueRule {
    conditions: Vec<String>,
    description: String,
    name: String,
    phase: i32,
    turn: i32,
}

pub fn read_rules() -> Result<Vec<Rule>> {
    let mut rules: Vec<Rule> = vec![];
    let data = fs::read_to_string(JSON_PATH)?;
    let res: Vec<ValueRule> = serde_json::from_str(&data)?;

    for obj in res {
        rules.push(Rule {
            name: obj.name.clone(),
            description: obj.description.clone(),
            phase: match Phase::name(obj.phase) {
                Some(p) => p,
                None => Phase::BeforeGame,
            },
            turn: match Turn::name(obj.turn) {
                Some(t) => t,
                None => Turn::Both,
            },
            conditions: obj.conditions.clone(),
        });
    }

    Ok(rules)
}
