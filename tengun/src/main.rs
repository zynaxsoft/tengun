use tengun_las::reader::LasReader;

fn main() {
    let reader = LasReader::from_path("/home/tanapol/wsl-share/LaserTanapol.las");
    reader.print_raw_vlrs();
}
