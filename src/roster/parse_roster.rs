use minidom::Element;

use crate::roster::data::{Force, Roster, RosterRule, Unit};

const NS: &'static str = "http://www.battlescribe.net/schema/rosterSchema";

pub fn create_roster(root: &Element) -> Roster {
    let name: String = get_attr(root, "name");
    let (power_level, points) = get_costs(root);
    let forces: Vec<Force> = get_forces(root);
    Roster::new(name, power_level, points, forces)
}

fn get_costs(r: &Element) -> (f64, f64) {
    let mut pl: f64 = 0.0;
    let mut pts: f64 = 0.0;
    for c0 in r.children() {
        if c0.is("costs", NS) {
            for c1 in c0.children() {
                if c1.is("cost", NS) {
                    match c1.attr("value") {
                        Some(v) => match c1.attr("name") {
                            Some(" PL") => {
                                pl = v.parse::<f64>().unwrap_or_else(|err| {
                                    eprintln!("Problem parsing Power Level from .ros file: {err}");
                                    0.0
                                })
                            }
                            Some("pts") => {
                                pts = v.parse::<f64>().unwrap_or_else(|err| {
                                    eprintln!("Problem parsing Points from .ros file: {err}");
                                    0.0
                                })
                            }
                            Some(_) => (),
                            None => (),
                        },
                        None => println!("No cost data found on roster"),
                    }
                }
            }
        }
    }
    (pl, pts)
}

fn get_forces(r: &Element) -> Vec<Force> {
    let mut forces: Vec<Force> = vec![];
    for c0 in r.children() {
        if c0.is("forces", NS) {
            for c1 in c0.children() {
                if c1.is("force", NS) {
                    let faction = get_attr(c1, "catalogueName");
                    let name = get_attr(c1, "name");
                    let units: Vec<Unit> = get_units(c1);

                    forces.push(Force::new(faction, name, units));
                }
            }
        }
    }
    forces
}

fn get_units(f: &Element) -> Vec<Unit> {
    let mut units: Vec<Unit> = vec![];

    for c0 in f.children() {
        if c0.is("selections", NS) {
            for c1 in c0.children() {
                if c1.is("selection", NS) {
                    if match c1.attr("type") {
                        Some("model") => true,
                        Some("unit") => true,
                        Some(_) => false,
                        None => false,
                    } {
                        let id: String = get_attr(c1, "id");
                        let name: String = get_attr(c1, "name");
                        let rules: Vec<RosterRule> = get_rules(c1);
                        units.push(Unit::new(id, name, rules));
                    }
                }
            }
        }
    }

    units
}

fn get_rules(u: &Element) -> Vec<RosterRule> {
    let mut rules: Vec<RosterRule> = vec![];

    for c0 in u.children() {
        if c0.is("rules", NS) {
            for c1 in c0.children() {
                if c1.is("rule", NS) {
                    let id: String = get_attr(c1, "id");
                    let name: String = get_attr(c1, "name");
                    let description: String = get_rule_description(c1);
                    rules.push(RosterRule::new(id, name, description));
                }
            }
        }

        if c0.is("profiles", NS) {
            for c1 in c0.children() {
                if c1.is("profile", NS) {
                    if match c1.attr("typeName") {
                        Some("Abilities") => true,
                        Some(_) => false,
                        None => false,
                    } {
                        let id: String = get_attr(c1, "id");
                        let name: String = get_attr(c1, "name");
                        let description: String = get_ability_description(c1);
                        rules.push(RosterRule::new(id, name, description));
                    }
                }
            }
        }
    }

    rules
}

fn get_rule_description(r: &Element) -> String {
    for c0 in r.children() {
        if c0.is("description", NS) {
            return c0.text();
        }
    }
    String::from("")
}

fn get_ability_description(a: &Element) -> String {
    for c0 in a.children() {
        if c0.is("characteristics", NS) {
            for c1 in c0.children() {
                if c1.is("characteristic", NS) {
                    if match c1.attr("name") {
                        Some("Description") => true,
                        Some(_) => false,
                        None => false,
                    } {
                        return c1.text();
                    }
                }
            }
        }
    }
    String::from("")
}

fn get_attr(e: &Element, s: &str) -> String {
    match e.attr(s) {
        Some(x) => String::from(x),
        None => String::from(""),
    }
}
