use minidom::Element;
use roster::load_ros::Roster;
use std::env;
use std::process;

mod roster;

const LINE: &'static str = "=======";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file_name: String = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    file_name.push_str(".ros");

    let ros_str = roster::load_ros::read_ros(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem reading .ros file: {err}");
        process::exit(1);
    });

    let root: Element = roster::load_ros::get_xml(&ros_str);

    let roster: Roster = roster::load_ros::create_roster(&root);

    display_roster(&roster);

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
    println!("{}{}{}", LINE, roster.get_name(), LINE);
    println!("Power Level: {:.1}", roster.get_power_level());
    println!("Points: {:.1}", roster.get_points());
    for force in roster.get_forces() {
        println!("{}{}{}", LINE, force.get_name(), LINE);
        println!("Faction: {}", force.get_faction());
    }
}
