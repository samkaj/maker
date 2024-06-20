/// A Maker creates a makefile from a list of files.
pub trait Maker {
    /// Build the makefile.
    fn build(&mut self);
    /// Dump the makefile as a string.
    fn dump(&self) -> String;
    /// Dump the makefile to a file. If the file already exists, a backup will be created before writing.
    fn dump_to_file(&self, path: &str) -> std::io::Result<()> {
        if std::path::Path::new(path).exists() {
            let backup_path = format!("{}.bak", path);
            std::fs::copy(path, backup_path)?;
        }
        std::fs::write(path, self.dump())
    }
}
