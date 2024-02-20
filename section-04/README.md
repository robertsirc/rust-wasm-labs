# Section Four Updating the Settings for the Policy

## Introduction

This section we will modify our policy settings and add some unit test.

## Prerequisites

All previous sections completed.

## Reconfigure

### Settings

In the `src/settings.rs` file we are going to modify it to look like this:

```Rust
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub cpu_limits: String,
}
```

This is where we are setting our cpu_limits value in the settings.

### Settings Validation

There is a `fn` already in the `src/settings.rs` file called `validate` we are going to modify this to validate our settings:

```Rust
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

```Rust
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

If you run `cargo test` the terminal will show you all the test passing:

```bash
$cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/pod_sizer-9c6b787ddc83ed29)

running 5 tests
test settings::tests::accept_settings_with_cpu_limits_set ... ok
test settings::tests::reject_settings_with_no_cpu_limits_set ... ok
test tests::accept_request_with_non_pod_resource ... ok
test tests::accept_pod_with_valid_name ... ok
test tests::reject_pod_with_invalid_name ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Conclusion

In our policy we have modified the settings to handle `cpu_limits` and updated the test to handle it.
