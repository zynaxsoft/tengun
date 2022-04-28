use std::path::Path;

use las::{Read, Reader, Point};

type Geokey = u16;

pub struct LasReader {
    reader: Reader,
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

    pub fn get_coords(&mut self) -> Result<Vec<Point>, las::Error> {
        self.reader.points().collect()
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
        let record = self
            .reader
            .header()
            .all_vlrs()
            .find(|r| r.record_id == 34735)?;
        let geokey: Vec<u8> = record
            .data
            .iter()
            .cloned()
            .skip_while(|&n| n != 12)  // 0x00C0 for 3072 in LE
            .skip(5)
            .take(2)
            .collect();
        if record.is_empty() {
            return None
        }
        Some(u16::from_le_bytes(geokey[..2].try_into().unwrap()))
    }
}
