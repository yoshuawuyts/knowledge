# pods

## Health Checking
```yaml
apiVersion: extensions/v1beta1
kind: Deployment
metadata:
  name: buildkite-agent-builder-private-git
spec:
  replicas: 2
  template:
    metadata:
      labels:
        app: buildkite-agent-builder-private-git
        heritage: helm
    spec:
      containers:
          livenessProbe:
            initialDelaySeconds: 15
            timeoutSeconds: 1
            exec:
              command:
                - sudo
                - docker
                - ps
```
- http://kubernetes.io/docs/user-guide/liveness/
