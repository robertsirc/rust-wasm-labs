# Section Six Building and Deploying

## Introduction

This section we are going to build and deploy our policy.

## Prerequisites

All previous sections completed.

## Building the Policy

If you don't have the correct target installed (`wasm32-unknown-unknown`) you are going to need to install it

``` Shell
rustup target add wasm32-unknown-unknown
```

To build the `.wasm` file there is a simple command to run:

``` Shell
make build
```
