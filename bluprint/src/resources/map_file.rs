#[derive(Default)]
pub struct MapFileRes(Option<MapFile>);

pub struct MapFile {
    path: std::path::PathBuf,
}
