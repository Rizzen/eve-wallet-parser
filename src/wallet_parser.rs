use std::collections::HashMap;
use chrono::NaiveDateTime;

pub struct WalletEntry {
    date: NaiveDateTime,
    count: i64,
    name: String,
    unit_price: f64,
    transaction_price: f64,
    client: String,
    location: String
}

pub struct Wallet {
    entries: Vec<WalletEntry>
}

pub trait Normalizable {
    fn normalize_number(&self) -> String;
}

pub trait Isk {
    fn remove_currency(&self) -> String;
}

impl Normalizable for &str {
    fn normalize_number(&self) -> String {
        return self.chars()
            .filter(|&c| c != '.' && c != ' ')
            .map(|c| if c == ',' { '.' } else { c })
            .collect::<String>();
    }
}

impl Isk for String {
    fn remove_currency(&self) -> String {
        return self.split("ISK").next().unwrap().to_string();
    }
}

pub fn parse_wallet(input: String) {
    let lines = input.split("\r\n");
    let mut map: HashMap<String, Vec<WalletEntry>> = HashMap::new();

    for line in lines {
        let parts = line.split("\t").collect::<Vec<_>>();
        let date_time = NaiveDateTime::parse_from_str(parts[0], "%Y.%m.%d %H:%M").unwrap();
        let num_slice = parts[1].normalize_number();

        let count = num_slice.parse::<i64>().unwrap();
        let name = parts[2].to_string();
        let unit_price = parse_isk(parts[3]);
        let trans_price = parse_isk(parts[4]);
        let client = parts[5].to_string();
        let location = parts[6].to_string();

        let key = name.clone();
        let existing = map.get_mut(&key);
        match existing {
            None => {
                let entry = WalletEntry {
                    date: date_time,
                    count,
                    name,
                    unit_price,
                    transaction_price: trans_price,
                    client,
                    location,
                };
                map.insert(key, vec![entry]);
            }

            Some(mut ex) => {
                let entry = WalletEntry {
                    date: date_time,
                    count,
                    name,
                    unit_price,
                    transaction_price: trans_price,
                    client,
                    location,
                };
                &ex.push(entry);
            }
        };
    }

    println!("{}", map.iter().count())
}

fn parse_isk(input: &str) -> f64 {
    return input.normalize_number().remove_currency().parse::<f64>().unwrap()
}