use crate::{Error, Result};

pub fn name<P: AsRef<std::path::Path>>(path: P) -> Result<String> {
    path.as_ref()
        .file_name()
        .map(std::ffi::OsStr::to_string_lossy)
        .map(String::from)
        .ok_or_else(|| Error::PathWithoutBasename(std::path::PathBuf::from(path.as_ref())))
}

enum Path {
    Dir(Dir),
    File(File),
}

pub struct Dir {
    pub name: String,
    pub status: Status,
    pub children: Vec<Path>,
}

pub struct File {
    pub name: String,
    pub status: Status,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Equal,
    Partial,
    MissingLeft,
    MissingRight,
}

trait NamedStatus {
    fn name(&self) -> &str;
    fn status(&self) -> Status;
}

impl NamedStatus for Path {
    fn name(&self) -> &str {
        match self {
            Path::Dir(dir) => dir.name(),
            Path::File(file) => file.name(),
        }
    }

    fn status(&self) -> Status {
        match self {
            Path::Dir(dir) => dir.status(),
            Path::File(file) => file.status(),
        }
    }
}

impl NamedStatus for Dir {
    fn name(&self) -> &str {
        &self.name
    }

    fn status(&self) -> Status {
        self.status
    }
}

impl NamedStatus for File {
    fn name(&self) -> &str {
        &self.name
    }

    fn status(&self) -> Status {
        self.status
    }
}
// use crate::error::*;
//
// pub type Dir = Path<true>;
// pub type File = Path<false>;
//
// pub type Dir = Path<true>;
// pub type File = Path<false>;
//
// pub struct Path<const DIR: bool> {
//     path: std::path::PathBuf,
//     status: Status,
// }
//
// impl<const DIR: bool> Path<DIR> {
//     pub fn new(path: std::path::PathBuf, status: Status) -> Result<Self> {
//         if !path.exists() {
//             return Err(Error::PathDoesNotExist(path));
//         }
//
//         if path.is_dir() != DIR {
//             return if DIR {
//                 Err(Error::PathNotDirectory(path))
//             } else {
//                 Err(Error::PathNotFile(path))
//             };
//         }
//
//         Ok(Self { path, status })
//     }
//
//     pub fn path(self) -> std::path::PathBuf {
//         self.path
//     }
//
//     pub fn status(&self) -> Status {
//         self.status
//     }
// }
//
// impl<const DIR: bool> std::ops::Deref for Path<DIR> {
//     type Target = std::path::PathBuf;
//
//     fn deref(&self) -> &Self::Target {
//         &self.path
//     }
// }
//
// impl<const DIR: bool> AsRef<std::path::Path> for Path<DIR> {
//     fn as_ref(&self) -> &std::path::Path {
//         self.path.as_ref()
//     }
// }
//
// #[derive(Copy, Clone, Eq, PartialEq)]
// pub enum Status {
//     Equal,
//     Partial,
//     Missing,
// }
//
// // pub struct Path<const DIR: bool> {
// //     name: std::ffi::OsString,
// //     path: std::path::PathBuf,
// //     status: Status,
// // }
// //
// // impl<const DIR: bool> Path<DIR> {
// //     pub fn new(path: std::path::PathBuf) -> Result<Self> {
// //         if !path.exists() {
// //             return Err(Error::PathDoesNotExist(path));
// //         }
// //
// //         if path.is_dir() != DIR {
// //             return if DIR {
// //                 Err(Error::PathNotDirectory(path))
// //             } else {
// //                 Err(Error::PathNotFile(path))
// //             };
// //         }
// //
// //         match path.file_name() {
// //             None => Err(Error::PathWithoutBasename(path)),
// //             Some(base_name) => {
// //                 let name = base_name.to_owned();
// //                 let status = Status::Unvisited;
// //                 Ok(Self { name, path, status })
// //             }
// //         }
// //     }
// //
// //     pub fn path(self) -> std::path::PathBuf {
// //         self.path
// //     }
// //
// //     pub fn with(self, status: Status) -> Self {
// //         Self { status, ..self }
// //     }
// // }
// //
// // impl<const DIR: bool> std::ops::Deref for Path<DIR> {
// //     type Target = std::path::PathBuf;
// //
// //     fn deref(&self) -> &Self::Target {
// //         &self.path
// //     }
// // }
// //
// // impl<const DIR: bool> AsRef<std::path::Path> for Path<DIR> {
// //     fn as_ref(&self) -> &std::path::Path {
// //         self.path.as_ref()
// //     }
// // }
// //
// // impl<const DIR: bool> std::cmp::Eq for Path<DIR> {}
// //
// // impl<const DIR: bool> std::cmp::PartialEq for Path<DIR> {
// //     fn eq(&self, other: &Self) -> bool {
// //         self.name.eq(&other.name)
// //     }
// // }
// //
// // impl<const DIR: bool> std::cmp::Ord for Path<DIR> {
// //     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
// //         self.name.cmp(&other.name)
// //     }
// // }
// //
// // impl<const DIR: bool> std::cmp::PartialOrd for Path<DIR> {
// //     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
// //         Some(self.cmp(other))
// //     }
// // }
// //
// // impl<const DIR: bool> std::fmt::Display for Path<DIR> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         std::fmt::Display::fmt(&self.name.to_string_lossy(), f)
// //     }
// // }
// //
// // #[derive(Copy, Clone, Eq, PartialEq)]
// // pub enum Status {
// //     Unvisited,
// //     Equal,
// //     Partial,
// //     Missing,
// // }
// //
// // // pub trait Status: private::Sealed {}
// // //
// // // mod private {
// // //     pub trait Sealed {}
// // // }
// // //
// // // struct Unvisited;
// // //
// // // impl Status for Unvisited {}
// // // impl private::Sealed for Unvisited {}
// // //
// // // #[derive(Copy, Clone, Eq, PartialEq)]
// // // enum CmpStatus {
// // //     Equal,
// // //     Partial,
// // //     Missing,
// // // }
// // // impl Status for CmpStatus {}
// // // impl private::Sealed for CmpStatus {}
