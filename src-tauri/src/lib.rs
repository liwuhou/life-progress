use std::path::Path;

use anyhow::Result;
use life_progress_core::{load_profile, Profile};

pub fn load_configured_profile() -> Result<Option<Profile>> {
    load_profile()
}

pub fn is_init_done() -> Result<bool> {
    Ok(load_configured_profile()?.is_some())
}

pub fn is_profile_configured_at(path: &Path) -> Result<bool> {
    Ok(Profile::load_from_path(path)?.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temporary_profile_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "life-progress-app-{name}-{}-{}.toml",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn reports_absent_profile() -> Result<()> {
        assert!(!is_profile_configured_at(&temporary_profile_path(
            "absent"
        ))?);
        Ok(())
    }

    #[test]
    fn reports_invalid_profile() -> Result<()> {
        let path = temporary_profile_path("invalid");
        fs::write(&path, "invalid = [")?;
        assert!(is_profile_configured_at(&path).is_err());
        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn reports_configured_profile() -> Result<()> {
        let path = temporary_profile_path("configured");
        fs::write(
            &path,
            "schema_version = 1\nbirthday = '1994-12-10'\nnation = 'Common'\n",
        )?;
        assert!(is_profile_configured_at(&path)?);
        fs::remove_file(path)?;
        Ok(())
    }
}
