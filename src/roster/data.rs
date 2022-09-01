pub struct Roster {
    name: String,
    power_level: f64,
    points: f64,
    forces: Vec<Force>,
}

impl Roster {
    pub fn new(name: String, power_level: f64, points: f64, forces: Vec<Force>) -> Roster {
        Roster {
            name,
            power_level,
            points,
            forces,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_power_level(&self) -> &f64 {
        &self.power_level
    }
    pub fn get_points(&self) -> &f64 {
        &self.points
    }
    pub fn get_forces(&self) -> &Vec<Force> {
        &self.forces
    }
}

pub struct Force {
    faction: String,
    name: String,
    units: Vec<Unit>,
}

impl Force {
    pub fn new(faction: String, name: String, units: Vec<Unit>) -> Force {
        Force {
            faction,
            name,
            units,
        }
    }

    pub fn get_faction(&self) -> &String {
        &self.faction
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_units(&self) -> &Vec<Unit> {
        &self.units
    }
}

pub struct Unit {
    id: String,
    name: String,
    rules: Vec<RosterRule>,
}

impl Unit {
    pub fn new(id: String, name: String, rules: Vec<RosterRule>) -> Unit {
        Unit { id, name, rules }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_rules(&self) -> &Vec<RosterRule> {
        &self.rules
    }
}

pub struct RosterRule {
    id: String,
    name: String,
    description: String,
}

impl RosterRule {
    pub fn new(id: String, name: String, description: String) -> RosterRule {
        RosterRule {
            id,
            name,
            description,
        }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
}
