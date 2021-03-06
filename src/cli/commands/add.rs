use crate::core::Dependency;
use crate::core::LoadableConfig;
use crate::core::VendorSpec;
use crate::VENDOR_YML;
use log::error;
use log::info;

pub fn run(
    url: &str,
    refname: &str,
    extensions: Option<Vec<String>>,
    targets: Option<Vec<String>>,
    ignores: Option<Vec<String>>,
) {
    let mut spec = match VendorSpec::load_from(VENDOR_YML) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let mut dep = Dependency::new(url, refname);
    if let Some(extensions) = extensions {
        dep.extensions = extensions;
    }
    if let Some(targets) = targets {
        dep.targets = targets;
    }
    if let Some(ignores) = ignores {
        dep.ignores = ignores;
    }
    spec.add(dep);

    match spec.save_to(VENDOR_YML) {
        Ok(_) => {
            info!("added dependency {}@{}", url, refname);
        }
        Err(err) => {
            error!("add failed: {}", err)
        }
    }
}
