# k8s-mismatch - Development Guide

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
   ```

3. **kubectl** - For interacting with Kubernetes
   ```bash
   brew install kubectl  # macOS
   kubectl version --client
   ```

## Setup Commands

### 1. Create the Kind cluster
```bash
kind create cluster --config ../kind-config.yaml --name agent-sre
```

### 2. Build the Docker image
```bash
docker build -t web-server:latest app/
```

### 3. Load the image into Kind
```bash
kind load docker-image web-server:latest --name agent-sre
```

### 4. Apply the Kubernetes manifests
```bash
kubectl apply -f k8s/
```

## Common Debugging Commands

### Check pod status
```bash
kubectl get pods
```

### Check pod logs
```bash
kubectl logs <pod-name>
```

### Describe a pod (see events, probe status)
```bash
kubectl describe pod <pod-name>
```

### Check service endpoints
```bash
kubectl get endpoints
```

### Check deployment status
```bash
kubectl get deployment
```

### Test the service (port-forward)
```bash
kubectl port-forward service/web-server-service 8080:80
curl http://localhost:8080
```

## Clean Up

Delete the Kind cluster when done:
```bash
kind delete cluster --name agent-sre
```
