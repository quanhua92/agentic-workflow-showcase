# k8s-mismatch

## Goal
Debug and fix a Kubernetes deployment where the application is not accessible. The pods are failing to start properly and the service has no endpoints.

## Prerequisites

You need the following tools installed:

1. **Docker** - For building the container image
   ```bash
   docker --version
   ```

2. **kind** (Kubernetes in Docker) - For running a local Kubernetes cluster
   ```bash
   brew install kind  # macOS
   kind --version
   kind create cluster --config kind-config.yaml --name agent-sre
   ```

3. **kubectl** - For interacting with Kubernetes
   ```bash
   brew install kubectl  # macOS
   kubectl version --client
   ```

## The Mission
1. **Open the directory:** Open the `project/` directory in your terminal.
2. **Initialize Claude Code:** Run the command `claude`.
3. **Trigger Plan Mode:** Press `Shift+Tab` to enter **Plan Mode**, then paste:

> First, read the README.md file for instructions on how to set up and run the Kind cluster. Then build the Docker image, load it into the Kind cluster, and apply the Kubernetes manifests in the k8s/ directory. The system is broken - use the Kubernetes MCP tools to investigate what's wrong. Identify, verify and fix all issues until the system is stable and reachable.

## Success Criteria
- `kubectl get pods` shows all pods Running with 0 restarts
- `kubectl get endpoints` shows Pod IPs mapped to the Service
- A curl test to the Service returns 200 OK
