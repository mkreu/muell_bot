use std::collections::{VecDeque, HashMap};
use chrono::prelude::*;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

#[derive(Debug)]
pub struct DateMgr {
    dates : HashMap<TrashType, VecDeque<NaiveDate>>
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TrashType {
    pub name : String
}

impl DateMgr {
    pub fn new() -> DateMgr {
        DateMgr { dates: HashMap::new() }
    }

    pub fn dates(&self) -> &HashMap<TrashType, VecDeque<NaiveDate>> {
        &self.dates
    }

    pub fn upcoming_dates(&self) -> Vec<(&TrashType, &NaiveDate)> {
        self.dates().iter()
            .filter(|entry| entry.1.get(0).is_some())
            .map(|(tonne, date)| (tonne, date.get(0).unwrap()))
            .collect()
    }

    pub fn next_date(&self) -> Option<(&NaiveDate, Vec<&TrashType>)> {
        let upcoming = self.upcoming_dates();
        let mut trashes = Vec::new();
        let date = match upcoming.get(0) {
            Some(tup) => {
                tup.1
            }
            None => {
                return None;
            }
        };

        for tup in upcoming {
            if tup.1 == date {
                trashes.push(tup.0);
            }
            else if tup.1 < date {
                trashes = Vec::new();
                trashes.push(tup.0);
            }
        }
        Some((date, trashes))
    }

    pub fn remove_old(&mut self) {
        for (k, mut vec) in &self.dates.clone() {
            let new_vec = vec.iter().filter(|date| date > &&Local::now().naive_local().date())
                .map(|date| date.to_owned())
                .collect();
            self.dates.insert(k.to_owned(), new_vec);
        }
    }

    pub fn append_file(&mut self, filename: &str) -> Result<(), Box<Error>> {
        let f = File::open(filename)?;
        let mut rdr = BufReader::new(f);

        //Parse Header
        let mut header = String::new();
        rdr.read_line(&mut header)?;
        let mut idx = Vec::new(); //Saves names with index in file
        for s in header.split(';') {
            let trash = TrashType{name : String::from(s.trim())};
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
                i = i+1;
            }
        }

        println!("{:?}", self);
        Ok(())
    }
}
