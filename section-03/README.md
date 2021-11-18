# Section Three Creating Rust Policy

## Introduction

This section will cover the creation of a policy for Kubewarden written in Rust.

## Prerequisites

All previous sections completed.

## Creating Project from Template

In a terminal run the following command:

```bash
cargo install cargo-generate
```

```bash
cargo generate --git https://github.com/kubewarden/policy-rust-template \
               --branch main \
               --name pod-sizer
```

## Conclusion

A quick installation of `cargo-generate` and creating a new policy called `pod-sizer`.
