# Section Four Updating the Settings for the Policy

## Introduction

This section we will modify our policy settings and add some unit test.

## Prerequisites

All previous sections completed.

## Reconfigure

### Settings

In the `src/settings.rs` file we are going to modify it to look like this:

``` Rust
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub cpu_limits: String,
}
```

This is where we are setting our cpu_limits value in the settings.

### Settings Validation

There is a `fn` already in the `src/settings.rs` file called `validate` we are going to modify this to validate our settings:

``` Rust
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
```

This is ONLY checking to see if a CPU limit has been set. We can modify this later.

### Settings Tests

The settings file has a small subset of tests, we are going to modify these test to test our settings. The settings tests should look like this:

``` Rust
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
```

If you run `cargo test` the terminal will have a few errors in other files:

``` Shell
error[E0063]: missing field `cpu_limits` in initializer of `settings::Settings`
  --> src/lib.rs:78:23
   |
78 |             settings: Settings {},
   |                       ^^^^^^^^ missing `cpu_limits`

error[E0063]: missing field `cpu_limits` in initializer of `settings::Settings`
  --> src/lib.rs:98:23
   |
98 |             settings: Settings {},
   |                       ^^^^^^^^ missing `cpu_limits`

error[E0063]: missing field `cpu_limits` in initializer of `settings::Settings`
   --> src/lib.rs:118:23
    |
118 |             settings: Settings {},
    |                       ^^^^^^^^ missing `cpu_limits`

For more information about this error, try `rustc --explain E0063`.
```

We will address this in another section.

## Conclusion

In our policy we have modified the settings to handle `cpu_limits` and updated the test to handle it.
