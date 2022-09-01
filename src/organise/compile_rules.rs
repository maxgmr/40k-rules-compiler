use crate::database::stored_rules::{Phase, Rule, Turn};

pub struct OrganisedRules {
    phase: &'static Phase,
    turn: &'static Turn,
    rules: Vec<Rule>,
}

impl OrganisedRules {
    pub fn get_phase(&self) -> &Phase {
        &self.phase
    }
    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }
    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}

pub fn organise_by_phase(rules: &Vec<Rule>) -> Vec<OrganisedRules> {
    let mut organised_rules_vec: Vec<OrganisedRules> = vec![];
    // TODO
    organised_rules_vec
}
