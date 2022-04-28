use std::path::Path;

use las::{Read, Reader};

type Geokey = u16;

pub struct LasReader {
    pub reader: Reader,
}

impl LasReader {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let reader = Reader::from_path(path).unwrap();
        Self { reader }
    }

    pub fn print_coords(&mut self) {
        for p in self.reader.points() {
            let p = p.unwrap();
            println!("Coord: {}, {}, {}, {:?}", p.x, p.y, p.z, p.color);
        }
    }

    pub fn print_raw_vlrs(&self) {
        let header = self.reader.header();
        for v in header.all_vlrs() {
            let d = v.data.chunks(2);
            for n in d {
                let number = u16::from_le_bytes(n.try_into().unwrap());
                print!("{} ", number);
            }
        }
    }

    pub fn get_geokey(&self) -> Option<Geokey> {
        let mut geo_data = None;
        for r in self.reader.header().all_vlrs() {
            if r.record_id == 34735 {
                geo_data = Some(&r.data);
                break;
            }
        }
        if let Some(data) = geo_data {
            let mut converted = Vec::new();
            for n in data.chunks(2) {
                let array = n.try_into().unwrap();
                let number = u16::from_le_bytes(array);
                converted.push(number)
            }
            return Some(converted.iter().skip_while(|&&n| n != 3072).nth(3).unwrap().clone())
        }
        None
    }
}
