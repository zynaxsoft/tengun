use tengun_las::reader::LasReader;

fn main() {
    let mut reader = LasReader::from_path("/home/tanapol/wsl-share/LaserTanapol.las");
    println!("The CRS is {}", reader.get_geokey().unwrap());
    let coords = reader.get_coords();
    println!("{}", coords.unwrap().len());
}
