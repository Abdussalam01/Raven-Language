use std::path;
use include_dir::File;
use data::{Readable, SourceSet};
use crate::FileWrapper;

#[cfg(test)]
mod test {
    use std::path;
    use include_dir::{Dir, DirEntry, include_dir};
    use data::{Arguments, RunnerSettings};
    use crate::build;
    use crate::test::InnerFileSourceSet;

    static TESTS: Dir = include_dir!("lib/test/test");

    #[test]
    pub fn test_magpie() {
        test_recursive(&TESTS);
    }

    fn test_recursive(dir: &'static Dir) {
        for entry in dir.entries() {
            match entry {
                DirEntry::File(file) => {
                    let arguments = Arguments::build_args(false, RunnerSettings {
                        sources: vec!(),
                        debug: false,
                        compiler: "llvm".to_string(),
                    });

                    let path = file.path().to_str().unwrap().replace(path::MAIN_SEPARATOR, "::");
                    println!("Running {}", path);
                    let path = format!("{}::test", &path[0..path.len() - 3]);
                    match build::<bool>(path.clone(), arguments, vec!(Box::new(InnerFileSourceSet {
                        set: file
                    }))) {
                        Ok(inner) => match inner {
                            Some(found) => if !found {
                                assert!(false, "Failed test {}!", path)
                            },
                            None => assert!(false, "Failed to find method test in test {}", path)
                        },
                        Err(()) => assert!(false, "Failed to compile test {}!", path)
                    }
                }
                DirEntry::Dir(dir) => {
                    test_recursive(dir);
                }
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct InnerFileSourceSet {
    set: &'static File<'static>,
}

impl SourceSet for InnerFileSourceSet {
    fn get_files(&self) -> Vec<Box<dyn Readable>> {
        return vec!(Box::new(FileWrapper { file: self.set }));
    }

    fn relative(&self, other: &Box<dyn Readable>) -> String {
        let name = other.path()
            .replace(path::MAIN_SEPARATOR, "::");
        return name[0..name.len() - 3].to_string();
    }

    fn cloned(&self) -> Box<dyn SourceSet> {
        return Box::new(self.clone());
    }
}