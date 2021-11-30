use crate::LOG_DRAIN;

use serde::{Deserialize, Serialize};
use slog::info;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub cpu_limits: String,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        info!(LOG_DRAIN, "starting settings validation");
        if self.cpu_limits.is_empty() {
            Err(String::from("No CPU limits is set."))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn accept_settings_with_cpu_limits_set() -> Result<(), ()> {
        let cpu_limits = String::from("0.1");
        let settings = Settings { cpu_limits };

        assert!(settings.validate().is_ok());
        Ok(())
    }

    #[test]
    fn reject_settings_with_no_cpu_limits_set() -> Result<(), ()> {
        let cpu_limits = String::new();
        let settings = Settings { cpu_limits };

        assert!(settings.validate().is_err());
        Ok(())
    }
}
