use anyhow::format_err;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn reset_vendor<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|err| format_err!("cannot reset vendor folder: {}", err))?
    }
    Ok(())
}

/// Ensures vendor directory is present in root path
pub fn ensure_vendor<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir(&path).map_err(|err| {
            format_err!(
                "cannot create vendor folder '{name}': {err}",
                name = path.display(),
                err = err
            )
        })?
    }
    if !path.is_dir() {
        return Err(format_err!(
            "vendor path '{}' already exists, and it's not a directory",
            path.display()
        ));
    }
    Ok(path.to_owned())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::utils::tests;

    #[test]
    fn test_ensure_vendor_empty_root() {
        let root = &tests::tempdir();
        let vendor = root.path().join("vendor");

        match ensure_vendor(vendor) {
            Ok(actual) => {
                assert!(actual.exists());
                assert!(actual.is_dir());
            }
            Err(err) => {
                panic!("expected vendor to succeed, but failed with: {}", err);
            }
        }
    }

    #[test]
    fn test_ensure_vendor_err_vendor_is_file() {
        let root = &tests::tempdir();
        let vendor = root.path().join("vendor");
        tests::write_to(&vendor, "");

        match ensure_vendor(&vendor) {
            Ok(actual) => {
                panic!("expected to fail, but succeeded with: {}", actual.display());
            }
            Err(err) => {
                assert_eq!(
                    format!(
                        "vendor path '{}' already exists, and it's not a directory",
                        vendor.display()
                    ),
                    err.to_string()
                )
            }
        }
    }
}
