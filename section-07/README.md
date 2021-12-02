# Section Seven Deploying and Testing

## Introduction

This section we are going to deploy our policy to Kubernetes and test it.

## Prerequisites

All previous sections completed.

## Deploying the Policy

In the project root directory. There is a file called `pod-sizer.yml` This file looks something like this:

```yml
apiVersion: policies.kubewarden.io/v1alpha2
kind: ClusterAdmissionPolicy
metadata:
  name: pod-sizer
spec:
  module: registry://ghcr.io/robertsirc/rust-wasm-labs/pod_sizer:v0.0.1
  rules:
  - apiGroups: [""]
    apiVersions: ["v1"]
    resources: ["pods"]
    operations:
    - CREATE
  
  mutating: false
  settings:
    cpu_limits: "1.0"
```

Note since this is testing locally the `module` is coming from a local file system.

In the terminal deploy the `ClusterAdmissionPolicy` with `kubectl`:

```bash
kubectl apply -f pod-sizer.yml
```

And you should get an output like this:

```bash
clusteradmissionpolicy.policies.kubewarden.io/pod-sizer created
```

To make sure the `ClusterAdmissionPolicy` deployed correctly we can check it with a WebHook:

```bash
kubectl get validatingwebhookconfigurations.admissionregistration.k8s.io -l kubewarden
```

## Testing Policy

To test this we can attempt to deploy a pod that exceeds our limits:

```bash
kubectl 
```

## Conclusion

We have created a new policy written in Rust and built as a WASM. Deployed it to Kubernetes and tested it.
