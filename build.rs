fn main() {
    #[cfg(feature = "static")]
    qt::prepare();
}

#[cfg(feature = "static")]
mod qt {
    macro_rules! unwrap {
        ($value: expr, $message: literal) => {
            unwrap!($value, $message,)
        };
        ($value: expr, $message: literal, $($arg: tt)*) => {
            match $value {
                Ok(value) => value,
                Err(e) => panic!(concat!($message, ": {}"), e, $($arg)*),
            }
        };
    }

    pub fn prepare() {
        println!("cargo:rerun-if-var-changed=QT_DIR");

        let qt_dir = std::path::PathBuf::from(qt_dir());

        let mut searches = std::collections::HashSet::new();
        let mut libs = Vec::new();
        find_libs(qt_dir, &mut searches, &mut libs);

        for search in searches {
            let Some(search) = search.to_str() else {
                panic!("Could not convert `{}` to string", search.display());
            };
            println!("cargo:rustc-link-search={search}");
        }

        println!("cargo:rustc-link-lib=GL");
        for lib in libs {
            let Some(lib) = lib
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .and_then(|l| l.strip_prefix("lib"))
            else {
                panic!("Could not get file name for `{}`", lib.display());
            };
            println!("cargo:rustc-link-lib={lib}");
        }
    }

    fn qt_dir() -> String {
        if let Ok(qt_dir) = std::env::var("QT_DIR") {
            return qt_dir;
        }

        println!("cargo:warning=QT_DIR not set. Falling back to querying qmake");
        println!("cargo:rerun-if-var-changed=QMAKE");

        let qmake = std::env::var("QMAKE").unwrap_or_else(|_| String::from("qmake"));
        let stdout = unwrap!(
            std::process::Command::new(qmake)
                .args(["-query", "QT_INSTALL_PREFIX"])
                .output(),
            "Failed to get output from qmake"
        )
        .stdout;

        String::from(
            unwrap!(
                std::str::from_utf8(&stdout),
                "Failed to parse qmake output as UTF-8"
            )
            .trim(),
        )
    }

    fn find_libs(
        path: std::path::PathBuf,
        searches: &mut std::collections::HashSet<std::path::PathBuf>,
        libs: &mut Vec<std::path::PathBuf>,
    ) {
        let dir = unwrap!(
            path.read_dir(),
            "Could not scan `{}` for linking targets",
            path.display()
        );

        for entry in dir {
            let entry = unwrap!(
                entry,
                "Could not entry from `{}` for linking targets",
                path.display()
            );

            let entry = entry.path();
            if entry.is_file() {
                if let Some("a") = entry.extension().and_then(std::ffi::OsStr::to_str) {
                    if !searches.contains(&path) {
                        searches.insert(path.clone());
                    }
                    libs.push(entry);
                }
            } else if entry.is_dir() {
                find_libs(entry, searches, libs);
            }
        }
    }
}
