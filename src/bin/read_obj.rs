extern crate wavefront_reader;

fn main() {
    env_logger::init();

    let path = "my/new/path/file.obj";
    wavefront_reader::read_from(path);
}
