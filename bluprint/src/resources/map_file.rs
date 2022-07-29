pub struct MapFile {
    path: std::path::PathBuf,
}

impl MapFile {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    pub fn file_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }
}
