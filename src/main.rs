use std::env;
use std::process;

mod roster;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file_name: String = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    file_name.push_str(".ros");

    let roster = roster::load_ros::read_ros(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem reading .ros file: {err}");
        process::exit(1);
    });

    println!("{roster}");

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
