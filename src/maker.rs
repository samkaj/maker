/// A Maker creates a makefile from a list of files.
pub trait Maker {
    /// Create a new Maker.
    fn new(files: Vec<String>, output_dir: String) -> Self;
    /// Build the makefile.
    fn build(&mut self);
    /// Dump the makefile as a string.
    fn dump(&self) -> String;
}
