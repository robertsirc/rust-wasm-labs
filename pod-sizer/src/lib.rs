use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

use lazy_static::lazy_static;

extern crate wapc_guest as guest;
use guest::prelude::*;

use k8s_openapi::api::core::v1 as apicore;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity as apimachinery_quantity;

extern crate kubewarden_policy_sdk as kubewarden;
use kubewarden::{logging, protocol_version_guest, request::ValidationRequest, validate_settings};

mod settings;
use settings::Settings;

use slog::{info, o, warn, Logger};

lazy_static! {
    static ref LOG_DRAIN: Logger = Logger::root(
        logging::KubewardenDrain::new(),
        o!("policy" => "sample-policy")
    );
}

#[no_mangle]
pub extern "C" fn wapc_init() {
    register_function("validate", validate);
    register_function("validate_settings", validate_settings::<Settings>);
    register_function("protocol_version", protocol_version_guest);
}

#[derive(Debug, PartialEq)]
enum PolicyResponse {
    Accept,
    Reject(String),
}

fn validate(payload: &[u8]) -> CallResult {
    let validation_request: ValidationRequest<Settings> = ValidationRequest::new(payload)?;

    let pod = match serde_json::from_value::<apicore::Pod>(validation_request.request.object) {
        Ok(pod) => pod,
        Err(_) => return kubewarden::accept_request(),
    };

    let settings = validation_request.settings;

    info!(LOG_DRAIN, "starting validation");
    
    match validate_pod(pod, settings)? {
        PolicyResponse::Accept => kubewarden::accept_request(),
        PolicyResponse::Reject(message) => kubewarden::reject_request(Some(message), None),
    }
    
}

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

fn container_at_or_under_limit(container: apicore::Container, settings_cpu_limit: String) -> bool {
    let limits = container.resources.unwrap_or_default().limits.unwrap_or_default();
    let container_cpu_limit = limits.get("cpu").unwrap().0.clone();

    if container_cpu_limit == settings_cpu_limit {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    
}
