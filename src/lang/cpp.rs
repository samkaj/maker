mod cpp {
    use crate::maker::Maker;

    /// A Maker for C/C++ projects
    pub struct Cpp {
        files: Vec<String>,
        output_dir: String,
        header_files: Vec<String>,
        source_files: Vec<String>,
        obj_files: Vec<String>,
        makefile: String,
    }

    impl Maker for Cpp {
        fn new(files: Vec<String>, output_dir: String) -> Self {
            Cpp {
                files,
                output_dir,
                header_files: vec![],
                source_files: vec![],
                obj_files: vec![],
                makefile: String::new(),
            }
        }

        fn build(&mut self) {
            self.categorize_files();
            self.create_object_files();
            self.create_variables();
        }

        fn dump(&self) -> String {
            self.makefile.clone()
        }
    }

    impl Cpp {
        /// Split the files in to source files and headers
        fn categorize_files(&mut self) {
            let source_endings = vec![String::from(".cpp"), String::from(".cc"), String::from(".c")];
            let header_endings = vec![String::from(".hpp"), String::from(".h")];

            for file in &self.files {
                match file {
                    header
                        if header_endings
                            .iter()
                            .any(|h| file.to_lowercase().ends_with(h)) =>
                    {
                        self.header_files.push(header.to_string());
                    }
                    source
                        if source_endings
                            .iter()
                            .any(|s| file.to_lowercase().ends_with(s)) =>
                    {
                        self.source_files.push(source.to_string());
                    }
                    _ => {}
                }
            }
        }

        /// For each source file, create a corresponding object file, i.e., `foo.cpp -> foo.o`
        fn create_object_files(&mut self) {
            for source_file in &self.source_files {
                let file_name = source_file.rsplit_once(".").unwrap().0.to_string() + ".o";
                let out_file_name =
                    self.output_dir.clone() + "/" + file_name.split_once("/").unwrap().1;
                self.obj_files.push(out_file_name);
            }
        }

        /// Construct the Makefile
        fn create_variables(&mut self) {
            let objs = String::from("OBJS    = ") + &self.obj_files.join(" ");
            let headers = String::from("HEADERS = ") + &self.header_files.join(" ");
            let sources = String::from("SOURCES = ") + &self.source_files.join(" ");
            let out = String::from("OUT     = a.o");
            let flags = String::from("FLAGS   = -g -c -Wall ")
                + "-I"
                + self.source_files.get(0).unwrap().split_once("/").unwrap().0; // FIXME: what is this line? We should pass the source path and it should be ok
            let cc = String::from("CC      = ") + "cc"; // FIXME: pass the compiler
            self.makefile = format!(
                r#"# Object files:
{}
# Header files:
{}
# Source files:
{}
# Executable name, run the program with ./a.o
{}
# Compiler flags:
{}
# Compiler:
{}"#,
                objs.trim(),
                headers.trim(),
                sources.trim(),
                out.trim(),
                flags.trim(),
                cc.trim()
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_categorize_files() {
            let mut cpp = Cpp {
                files: vec![
                    String::from("src/main.cpp"),
                    String::from("src/foo.hpp"),
                    String::from("src/foo.cpp"),
                ],
                output_dir: String::from("target"),
                header_files: vec![],
                source_files: vec![],
                obj_files: vec![],
                makefile: String::new(),
            };

            cpp.categorize_files();

            assert_eq!(cpp.header_files, vec![String::from("src/foo.hpp")]);
            assert_eq!(
                cpp.source_files,
                vec![String::from("src/main.cpp"), String::from("src/foo.cpp")]
            );
        }

        #[test]
        fn test_create_object_files() {
            let mut cpp = Cpp {
                files: vec![
                    String::from("src/main.cpp"),
                    String::from("src/foo.hpp"),
                    String::from("src/foo.cpp"),
                ],
                output_dir: String::from("target"),
                header_files: vec![],
                source_files: vec![],
                obj_files: vec![],
                makefile: String::new(),
            };

            cpp.categorize_files();
            cpp.create_object_files();

            assert_eq!(
                cpp.obj_files,
                vec![String::from("target/main.o"), String::from("target/foo.o")]
            );
        }

        #[test]
        fn test_create_variables() {
            let mut cpp = Cpp {
                files: vec![
                    String::from("src/main.cpp"),
                    String::from("src/foo.hpp"),
                    String::from("src/foo.cpp"),
                ],
                output_dir: String::from("target"),
                header_files: vec![String::from("src/foo.hpp")],
                source_files: vec![String::from("src/main.cpp"), String::from("src/foo.cpp")],
                obj_files: vec![String::from("target/main.o"), String::from("target/foo.o")],
                makefile: String::new(),
            };

            cpp.create_variables();

            assert_eq!(
                cpp.makefile,
                r#"# Object files:
OBJS    = target/main.o target/foo.o
# Header files:
HEADERS = src/foo.hpp
# Source files:
SOURCES = src/main.cpp src/foo.cpp
# Executable name, run the program with ./a.o
OUT     = a.o
# Compiler flags:
FLAGS   = -g -c -Wall -Isrc
# Compiler:
CC      = cc"#
            );
        }
    }
}