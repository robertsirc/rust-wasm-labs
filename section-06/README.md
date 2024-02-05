# Section Six Building and Testing

## Introduction

This section we are going to build and test our policy locally.

## Prerequisites

All previous sections completed.

## Building the Policy

If you don't have the correct target installed (`wasm32-unknown-unknown`) you are going to need to install it

```bash
rustup target add wasm32-wasi
```

To build the `.wasm` file there is a simple command to run:

```bash
make policy.wasm
```

The output should be in a directory like this:

```bash
policy-root-directory/policy.wasm
```

## Annotating the Policy

Annotating a policy allow you to set metadata that travels with the policy. When using the template this file is in the root directory called `metadata.yaml`.

With another make target  we can annotate the file:

```bash
make annotated-policy.wasm
```

## Testing the Policy

Testing policies are done through `kwctl run`.

This command will test to make sure that the policy is accepting a pod created at the valid CPU limit:

```bash
kwctl run --request-path test_data/pod_creation_cpu_1.json --settings-json '{ "cpu_limits": "1.0"}' policy-root-directory/annotated-policy.wasm
```

This command will test to make sure that the policy is accepting a pod created at the invalid CPU limit:

```bash
kwctl run --request-path test_data/pod_creation_cpu_2.json --settings-json '{ "cpu_limits": "1.0"}' policy-root-directory/annotated-policy.wasm
```

## Conclusion

In this section we annotated our policy using `kwctl`. We also tested our policy using `kwctl`.
