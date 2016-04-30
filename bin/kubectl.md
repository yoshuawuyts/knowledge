# kubectl

## Install kubectl
```sh
$ gcloud components install kubectl
```

## run a pod
```sh
# runs the "hello-node" example from google
$ kubectl run <name> --image=gcr.io/google-samples/node-hello:1.0 --port=8080
```

## Expose a deployment to the outside world
```sh
$ kubectl expose deployment <name> --type="LoadBalancer"
```

## List deployments
```sh
$ kubectl get deployments
```
