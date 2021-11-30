# Section Five Updating the Policy

## Introduction

This section we are going to update the policy to handle the changes we are making for sizing a pod.

## Prerequisites

All previous sections completed.

## Reconfigure

### Fixing the Build

From the previous section the `lib.rs` file had errors when you run the `cargo test`. To correct we are going to need to modify the `lib.rs` file and remove all the test.

### Fixing the Validation

The test pass but they are not testing what we need them to cover. We are going to add a library dependency to our `Cargo.toml` file:

``` Toml
[dependencies]
anyhow = "1.0.45"
```

Next we are going to add the `anyhow` library to our `lib.rs` file:

``` Rust
use anyhow::{anyhow, Result};

use lazy_static::lazy_static;
```

We will remove the previous `match` statement from the `validate` function:

``` Rust
    // TODO: you can unmarshal any Kubernetes API type you are interested in
    match serde_json::from_value::<apicore::Pod>(validation_request.request.object) {
        Ok(pod) => {
            
            // TODO: your logic goes here
            if pod.metadata.name == Some("invalid-pod-name".to_string()) {
                let pod_name = pod.metadata.name.unwrap();
                info!(
                    LOG_DRAIN,
                    "rejecting pod";
                    "pod_name" => &pod_name
                );
                kubewarden::reject_request(
                    Some(format!("pod name {} is not accepted", &pod_name)),
                    None,
                )
            } else {
                info!(LOG_DRAIN, "accepting resource");
                kubewarden::accept_request()
            }
        }
        Err(_) => {
            // TODO: handle as you wish
            // We were forwarded a request we cannot unmarshal or
            // understand, just accept it
            warn!(LOG_DRAIN, "cannot unmarshal resource: this policy does not know how to evaluate this resource; accept it");
            kubewarden::accept_request()
        }
    }
```

And replace it with this:

``` Rust
let pod = match serde_json::from_value::<apicore::Pod>(validation_request.request.object) {
        Ok(pod) => pod,
        Err(_) => return kubewarden::accept_request(),
};
```

Next add an enum for `PolicyResponse` that should look like this:

``` Rust
#[derive(Debug, PartialEq)]
enum PolicyResponse {
    Accept,
    Reject(String),
}
```

This will handle the responses we are looking for.

Next we are going to create a function that will validate the pod called `validate_pod`:

``` Rust
fn validate_pod(pod: apicore::Pod, settings: settings::Settings) -> Result<PolicyResponse> {
    let pod_spec = pod.spec.ok_or_else(|| anyhow!("invalid pod spec"))?;

    let all_containers = pod_spec.containers.into_iter().all(|container| {
        container_at_or_under_limit(container, settings.cpu_limits.clone())
    });

    if all_containers {
        Ok(PolicyResponse::Accept)
    } else {
        Ok(PolicyResponse::Reject("Rejected".to_string()))
    }
}
```

And call this new function from our `validate` function:

``` Rust
match validate_pod(pod, settings)? {
    PolicyResponse::Accept => kubewarden::accept_request(),
    PolicyResponse::Reject(message) => kubewarden::reject_request(Some(message), None),
}
```

Next create a function called `container_at_or_under_limit` this will handle our logic we are looking to implement:

``` Rust
fn container_at_or_under_limit(container: apicore::Container, settings_cpu_limit: String) -> bool {

    true
}
```

This will work for now we will come back and finish our validation after we set up some test.

### Adding a Test

First the following references reference like this:

``` Rust
use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity as apimachinery_quantity;
```

We are going to add a test to ensure our pods have containers at the cpu limit. This test will look something like this:

``` Rust
#[test]
fn pods_at_limit_set() -> Result<()> {
    let cpu_limits = String::from("1.5");
    
    let mut _limits: BTreeMap<String, apimachinery_quantity> = BTreeMap::new();
    _limits.insert(String::from("cpu"), apimachinery_quantity { 0: String::from("1.5") });
    
    assert_eq!(
        validate_pod(
            apicore::Pod {
                spec: Some({
                    apicore::PodSpec {
                        containers: vec![
                            apicore::Container {
                                resources: Some({
                                    apicore::ResourceRequirements {
                                        limits: Some(_limits),
                                        ..apicore::ResourceRequirements::default()
                                    }
                                }),
                                ..apicore::Container::default()
                            }
                        ],
                        ..apicore::PodSpec::default()
                    }
                }),
                ..apicore::Pod::default()
            },
            Settings { cpu_limits }
        )?,
        PolicyResponse::Accept
    );
    Ok(())
}
```
