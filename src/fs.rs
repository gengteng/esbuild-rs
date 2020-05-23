use std::collections::HashMap;
use std::path::{Path as StdPath, PathBuf};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum EntryKind {
    Dir = 1,
    File = 2,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: EntryKind,
    pub sym_link: String,
}

pub trait FileSystem {
    // The returned map is immutable and is cached across invocations. Do not
    // mutate it.
    fn read_directory<P: AsRef<StdPath>>(&self, path: P) -> HashMap<String, Entry>;
    fn read_file<P: AsRef<StdPath>>(&self, path: P) -> Option<String>;

    // This is part of the interface because the mock interface used for tests
    // should not depend on file system behavior (i.e. different slashes for
    // Windows) while the real interface should.
    fn abs<P: AsRef<StdPath>>(&self, path: P) -> Option<PathBuf>;
    fn dir<P: AsRef<StdPath>>(&self, path: P) -> PathBuf;
    fn base<P: AsRef<StdPath>>(&self, path: P) -> PathBuf;
    fn join<P: AsRef<StdPath>>(&self, path: Vec<P>) -> PathBuf;
    fn relative_to_cwd<P: AsRef<StdPath>>(&self, path: P) -> Option<PathBuf>;
}

#[derive(Debug, Clone)]
pub struct MockFileSystem {
    pub dirs: HashMap<PathBuf, HashMap<String, Entry>>,
    pub files: HashMap<PathBuf, String>,
}

impl MockFileSystem {
    // pub fn new(mut input: HashMap<PathBuf, String>) -> Self {
    //     let mut dirs = HashMap::new();
    //     let mut files = HashMap::new();
    //
    //     for (k, v) in input.drain() {
    //         files.insert(k.clone(), v.clone());
    //         let original = k;
    //     }
    //
    //     Self { dirs, files }
    // }
}

// impl FileSystem for MockFileSystem {
//     fn read_directory<P: AsRef<StdPath>>(&self, path: P) -> HashMap<String, Entry, RandomState> {
//         unimplemented!()
//     }
//
//     fn read_file<P: AsRef<StdPath>>(&self, path: P) -> Option<String> {
//         unimplemented!()
//     }
//
//     fn abs<P: AsRef<StdPath>>(&self, path: P) -> Option<PathBuf> {
//         unimplemented!()
//     }
//
//     fn dir<P: AsRef<StdPath>>(&self, path: P) -> PathBuf {
//         unimplemented!()
//     }
//
//     fn base<P: AsRef<StdPath>>(&self, path: P) -> PathBuf {
//         unimplemented!()
//     }
//
//     fn join<P: AsRef<StdPath>>(&self, path: Vec<P>) -> PathBuf {
//         unimplemented!()
//     }
//
//     fn relative_to_cwd<P: AsRef<StdPath>>(&self, path: P) -> Option<PathBuf> {
//         unimplemented!()
//     }
// }
