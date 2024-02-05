# Section Two Cluster Installation

## Introduction

This section will cover the installation of Kubewarden.

## Prerequisites

For this section we are going to need the following

### Cluster

To install Kubewarden you will need a Kubernetes cluster. This could be [Rancher Desktop](https://rancherdesktop.io/) or [k3s](https://k3s.io/). For this cluster 2GB of memory and 2 cores should be enough.

The cluster will also need a [`cert-manager`](https://cert-manager.io/docs/installation/) installed.

### CLI

To deploy Kubewarden we are going to need [Helm](https://helm.sh/) install on your development machine.

We are also going to need to install [kwctl](https://github.com/kubewarden/kwctl) on your local work station.

## Installing Kubewarden

Add the Kubewarden repo via `Helm`:

```bash
helm repo add kubewarden https://charts.kubewarden.io
```

Ensure that the `cert-manager` is deployed:

```bash
kubectl wait --for=condition=Available deployment --timeout=2m -n cert-manager --all
```

Install `kubewarden-crds`:

```bash
helm install --wait -n kubewarden --create-namespace kubewarden-crds kubewarden/kubewarden-crds
```

Install `kubewarden-controller`:

```bash
helm install --wait -n kubewarden kubewarden-controller kubewarden/kubewarden-controller
```

Install `kubewarden-defaults` Helm chart to get a default policy server:

```bash
helm install --wait -n kubewarden kubewarden-defaults kubewarden/kubewarden-defaults
```

## Conclusion

Installing `Helm` and `kwctl` on your development machine and Kubewarden on a cluster.
