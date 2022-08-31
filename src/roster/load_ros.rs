use minidom::Element;
use std::fs;

const NS: &'static str = "http://www.battlescribe.net/schema/rosterSchema";

pub struct Roster {
    name: String,
    power_level: f64,
    points: f64,
    forces: Vec<Force>,
}

impl Roster {
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
}

impl Force {
    pub fn get_faction(&self) -> &String {
        &self.faction
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub fn read_ros(file_name: &String) -> std::io::Result<String> {
    let ros_string = fs::read_to_string(file_name)?;
    return Ok(ros_string);
}

pub fn get_xml(s: &String) -> Element {
    s.parse().unwrap()
}

pub fn create_roster(root: &Element) -> Roster {
    let name: String = get_name(&root);
    let (power_level, points) = get_costs(&root);
    let forces: Vec<Force> = get_forces(&root);
    Roster {
        name,
        power_level,
        points,
        forces,
    }
}

fn get_name(r: &Element) -> String {
    match r.attr("name") {
        Some(n) => String::from(n),
        None => String::from(""),
    }
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
                    let faction = match c1.attr("catalogueName") {
                        Some(x) => String::from(x),
                        None => String::from(""),
                    };
                    let name = match c1.attr("name") {
                        Some(x) => String::from(x),
                        None => String::from(""),
                    };
                    forces.push(Force { faction, name });
                }
            }
        }
    }
    forces
}
