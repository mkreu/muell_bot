use chrono::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct DateMgr {
    dates: HashMap<TrashType, VecDeque<NaiveDate>>,
}

const SYMB_RESTMUELL: &str = "🗑";
const SYMB_PAPIER: &str = "📃";
const SYMB_GELBETONNE: &str = "♻️";
const SYMB_BIOMUELL: &str = "💩";
const SYMB_DEFAULT: &str = "";

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TrashType {
    pub name: String,
    pub symbol: &'static str,
}

impl TrashType {
    fn new(name: &str) -> TrashType {
        TrashType {
            name: String::from(name),
            symbol: match name {
                "Restmüll" => SYMB_RESTMUELL,
                "Papier" => SYMB_PAPIER,
                "Bioabfall" => SYMB_BIOMUELL,
                "Gelbe Tonne" => SYMB_GELBETONNE,
                _ => SYMB_DEFAULT,
            },
        }
    }
}

impl fmt::Display for TrashType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.symbol, self.name)
    }
}

impl DateMgr {
    pub fn new() -> DateMgr {
        DateMgr {
            dates: HashMap::new(),
        }
    }

    pub fn dates(&self) -> &HashMap<TrashType, VecDeque<NaiveDate>> {
        &self.dates
    }

    pub fn upcoming_dates(&self) -> Vec<(&TrashType, &NaiveDate)> {
        self.dates()
            .iter()
            .filter(|entry| entry.1.get(0).is_some())
            .map(|(tonne, date)| (tonne, date.get(0).unwrap()))
            .collect()
    }

    pub fn next_date(&self) -> Option<(&NaiveDate, Vec<&TrashType>)> {
        let mut upcoming = self.upcoming_dates();
        upcoming.sort_by_key(|tup| tup.1);
        let mut trashes = Vec::new();
        let date = match upcoming.get(0) {
            Some(tup) => tup.1,
            None => {
                return None;
            }
        };

        for tup in upcoming {
            if tup.1 == date {
                trashes.push(tup.0);
            } else if tup.1 < date {
                trashes = Vec::new();
                trashes.push(tup.0);
            }
        }
        Some((date, trashes))
    }

    pub fn remove_old(&mut self) {
        for (k, vec) in &self.dates.clone() {
            let new_vec = vec
                .iter()
                .filter(|date| date.and_hms(11, 0, 0) > Local::now().naive_local())
                .map(|date| date.to_owned())
                .collect();
            self.dates.insert(k.to_owned(), new_vec);
        }
    }

    pub fn append_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let f = File::open(filename)?;
        let mut rdr = BufReader::new(f);

        //Parse Header
        let mut header = String::new();
        rdr.read_line(&mut header)?;
        let mut idx = Vec::new(); //Saves names with index in file
        for s in header.split(';') {
            let trash = TrashType::new(s.trim());
            self.dates.entry(trash.clone()).or_insert(VecDeque::new());
            idx.push(trash);
        }

        //Parse dates
        for l in rdr.lines().filter_map(|result| result.ok()) {
            let split = l.split(";");
            if let (4, _) = split.size_hint() {
                continue;
            }
            let mut i = 0;
            for s in split.map(|s| s.trim()) {
                if !s.is_empty() {
                    let date = NaiveDate::parse_from_str(s, "%d.%m.%Y")?;
                    //Remove past dates, maybe move this to other fn
                    if date > Local::now().naive_local().date() {
                        self.dates.get_mut(&idx[i]).unwrap().push_back(date);
                    }
                }
                i = i + 1;
            }
        }

        println!("{:?}", self);
        Ok(())
    }
}
