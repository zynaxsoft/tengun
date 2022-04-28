use tengun_las::reader::LasReader;

fn main() {
    let reader = LasReader::from_path("/home/tanapol/wsl-share/LaserTanapol.las");
    println!("The CRS is {}", reader.get_geokey().unwrap());
}
