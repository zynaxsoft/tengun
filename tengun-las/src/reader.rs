use std::path::Path;

use las::{Read, Reader};

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
}
