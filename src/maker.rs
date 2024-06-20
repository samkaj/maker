/// A Maker creates a makefile from a list of files.
pub trait Maker {
    /// Build the makefile.
    fn build(&mut self);
    /// Dump the makefile as a string.
    fn dump(&self) -> String;
}
