use crate::{Error, Result};

pub fn name<P: AsRef<std::path::Path>>(path: P) -> Result<std::ffi::OsString> {
    path.as_ref()
        .file_name()
        .map(std::ffi::OsString::from)
        .ok_or_else(|| Error::PathWithoutBasename(std::path::PathBuf::from(path.as_ref())))
}

trait Stringifiable {
    fn stringify(self) -> String;
}

impl Stringifiable for std::ffi::OsString {
    fn stringify(self) -> String {
        self.into_string()
            .unwrap_or_else(|orig| String::from(orig.to_string_lossy()))
    }
}

impl<'a> Stringifiable for &'a std::ffi::OsStr {
    fn stringify(self) -> String {
        String::from(self.to_string_lossy())
    }
}

pub fn stringify(string: impl Stringifiable) -> String {
    string.stringify()
}

pub enum Path {
    Dir(Dir),
    File(File),
}

pub struct Dir {
    pub name: String,
    pub status: Status,
    pub children: Vec<Path>,
}

pub struct File {
    pub name: std::path::PathBuf,
    pub status: Status,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Equal,
    Partial,
    MissingLeft,
    MissingRight,
}

// trait NamedStatus {
//     fn name(&self) -> &str;
//     fn status(&self) -> Status;
// }
//
// impl NamedStatus for Path {
//     fn name(&self) -> &str {
//         match self {
//             Path::Dir(dir) => dir.name(),
//             Path::File(file) => file.name(),
//         }
//     }
//
//     fn status(&self) -> Status {
//         match self {
//             Path::Dir(dir) => dir.status(),
//             Path::File(file) => file.status(),
//         }
//     }
// }
//
// impl NamedStatus for Dir {
//     fn name(&self) -> &str {
//         &self.name
//     }
//
//     fn status(&self) -> Status {
//         self.status
//     }
// }
//
// impl NamedStatus for File {
//     fn name(&self) -> &str {
//         &self.name
//     }
//
//     fn status(&self) -> Status {
//         self.status
//     }
// }

pub struct Paths {
    pub parent: std::path::PathBuf,
    pub paths: Vec<std::ffi::OsString>,
}
