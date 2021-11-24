use log::{info};

mod vector2f;
mod vector3f;

const SPACE: &str = " ";

pub fn read_from(path: &str) {
    info!("Reading wavefront file from '{}'.", path);

    let line1 = "v 1.00 2.00 3.00";
    let v1 = vector3f::create_from(line1);
    info!("v1 = {:?}", v1);

    let line2 = "vt 2.00 3.00";
    let v2 = vector2f::create_from(line2);
    info!("v2 = {:?}", v2);
}
