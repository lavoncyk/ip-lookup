use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;
use std::usize;

#[derive(Debug)]
struct GeoLocation {
    network_range_start: Ipv4Addr,
    network_range_end: Ipv4Addr,
    country_code: String,
    city: String,
}

fn int_to_ipv4_addr(value: u32) -> Ipv4Addr {
    Ipv4Addr::new(
        (value >> 24) as u8,
        (value >> 16 & 0xFF) as u8,
        (value >> 8 & 0xFF) as u8,
        (value & 0xFF) as u8,
    )
}

fn load_database() -> io::Result<Vec<GeoLocation>> {
    let mut locations = Vec::new();

    let file = File::open("data/database.csv")?;
    let reader = io::BufReader::new(file);
    let chars_to_trim = [' ', '"'];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 8 {
            // Some expected fields are missing, ignore the line.
            continue;
        }

        let network_range_start = match parts[0]
            .trim_matches(&chars_to_trim)
            .parse::<u32>() {
                Ok(val) => int_to_ipv4_addr(val),
                Err(_) => {
                    continue;
                }
        };
        let network_range_end = match parts[1]
            .trim_matches(&chars_to_trim)
            .parse::<u32>() {
                Ok(val) => int_to_ipv4_addr(val),
                Err(_) => {
                    continue;
                }
        };

        let country_code = parts[2].trim_matches(&chars_to_trim).to_string();
        let city = parts[5].trim_matches(&chars_to_trim).to_string();

        let location = GeoLocation {
            network_range_start,
            network_range_end,
            country_code,
            city,
        };

        locations.push(location);
    }

    Ok(locations)
}

fn lookup_ip(ip: Ipv4Addr, database: &Vec<GeoLocation>) -> Option<String> {
    let mut start: usize = 0;
    let mut end = database.len();
    while start <= end {
        let mid = (start + end) >> 1;

        if ip >= database[mid].network_range_start
           && ip <= database[mid].network_range_end {
            return Some(
                format!("{},{}", database[mid].country_code, database[mid].city),
            );
        }

        else if ip < database[mid].network_range_start {
            end = mid - 1;
        }
        else {
            start = mid + 1;
        }
    }
    None
}

fn main() {
    let mut database: Option<Vec<GeoLocation>> = None;
    println!("READY");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();
        let command: Vec<&str>= trimmed_input.split_whitespace().collect();
        match command.as_slice() {
            ["LOAD"] => match load_database() {
                Ok(result) => {
                    database = Some(result);
                    println!("OK");
                }
                Err(_) => {
                    println!("ERR");
                }
            },
            ["LOOKUP", ip] => {
                match &database {
                    Some(db) => {
                        match ip.parse::<Ipv4Addr>() {
                            Ok(ip) => match lookup_ip(ip, db) {
                                Some(location) => println!("{}", location),
                                None => println!("ERR"),
                            },
                            Err(_) => println!("ERR"),
                        }
                    },
                    None => println!("ERR"),
                };
            }
            ["EXIT"] => {
                println!("OK");
                break;
            }
            _ => println!("ERR"),
        };
    }
}
