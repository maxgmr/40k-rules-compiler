use database::stored_rules::{Phase, Rule, Turn};
use minidom::Element;
use organise::compile_rules::OrganisedRules;
use roster::data::Roster;
use std::env;
use std::process;

mod database;
mod organise;
mod roster;

const BIG_LINE: &'static str = "=======";
const SMALL_LINE: &'static str = "-----";
const TEST_MODE: bool = true;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file_name: String = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    file_name.push_str(".ros");

    let rules = database::stored_rules::read_rules().unwrap_or_else(|err| {
        eprintln!("Problem reading rules.json: {err}");
        process::exit(1);
    });

    if TEST_MODE {
        println!("{} rules found in database.", rules.len());
    }

    let ros_str: String = roster::load_ros::read_ros(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem reading .ros file: {err}");
        process::exit(1);
    });

    let root: Element = roster::load_ros::get_xml(&ros_str);

    let roster: Roster = roster::parse_roster::create_roster(&root);

    if TEST_MODE {
        display_roster(&roster);
        display_database(&rules);
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<String, std::io::Error> {
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not enough arguments",
        ));
    }
    let file_name: String = args[1].clone();
    Ok(file_name)
}

fn display_roster(roster: &Roster) {
    println!("{}{}{}", BIG_LINE, roster.get_name(), BIG_LINE);
    println!("Power Level: {:.1}", roster.get_power_level());
    println!("Points: {:.1}", roster.get_points());
    for force in roster.get_forces() {
        println!("{}{}{}", BIG_LINE, force.get_name(), BIG_LINE);
        println!("Faction: {}", force.get_faction());
        for unit in force.get_units() {
            println!("{}{}{}", BIG_LINE, unit.get_name(), BIG_LINE);
            println!("{}Rules{}", SMALL_LINE, SMALL_LINE);
            for rule in unit.get_rules() {
                println!("{}", SMALL_LINE);
                println!("{}", rule.get_name());
                println!("{}", rule.get_description());
            }
            // println!("{}{}{}", SMALL_LINE, SMALL_LINE, SMALL_LINE);
        }
        // println!("{}{}{}", BIG_LINE, BIG_LINE, BIG_LINE);
    }
}

fn display_database(database: &Vec<Rule>) {
    let organised_rules_vec: Vec<OrganisedRules> =
        organise::compile_rules::organise_by_phase(database);
    println!("{}RULES IN DATABASE{}", BIG_LINE, BIG_LINE);
    for organised_rules in organised_rules_vec {
        if organised_rules.get_rules().len() > 0 {
            println!(
                "{}{:#?} {:#?}{}",
                BIG_LINE,
                organised_rules.get_turn(),
                organised_rules.get_phase(),
                BIG_LINE
            );
        }
    }
}
