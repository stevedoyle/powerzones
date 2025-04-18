use powerzones::calc_power_zones;
use std::env;
use std::process;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <FTP>", args[0]);
        process::exit(1);
    }

    let ftp: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid FTP value");
            process::exit(1);
        }
    };

    let zones = calc_power_zones(ftp);

    println!("{:<20} {:<10}", "Zone", "Range (W)");
    println!("{:-<30}", "");
    for zone in zones.iter() {
        println!(
            "{:<20} {:<10}",
            format!("{}", zone.name),
            format!("{}-{}", zone.lower, zone.upper)
        );
    }
}
