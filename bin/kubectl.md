# kubectl
Kubernetes is for clusters, cluster love kube.

## Install kubectl
```sh
$ gcloud components install kubectl
```
On the google container engine, the google cloud SDK is installed in one of
these locations:
```txt
/usr/local/share/google/google-cloud-sdk
/usr/lib/google-cloud-sdk
```

If `gcloud` doesn't allow upgrades, the config file must be configured to allow
for it:
```sh
$ sed -i -e 's/true/false/' \
  /usr/local/share/google/google-cloud-sdk/lib/googlecloudsdk/core/config.json
```

If `kubectl` is installed but the bin cannot be found, you'll probably need to
source it from the path:
```sh
PATH="/usr/lib/google-cloud-sdk/bin:$PATH"
```
- https://code.google.com/p/google-cloud-sdk/issues/detail?id=336
- http://stackoverflow.com/questions/35900502/google-cloud-platform-delete-or-stop-instances-without-prompt
- http://stackoverflow.com/questions/30029717/kubectl-is-not-installed-on-gce-by-default

## Run a pod
Requires a working cluster, preferably created on the goog cloud.
```sh
# runs the "hello-node" example from google
$ kubectl run <deployment_name> \
  --image=gcr.io/google-samples/node-hello:1.0 \
  --port=8080
```

## Expose a deployment to the outside world
```sh
$ kubectl expose deployment <deployment_name> --type="LoadBalancer"
```

## List deployments
```sh
$ kubectl get deployments
```

## List service info
Useful to retrieve open ports and public IP for a service. If you're not sure
which services are available, run `kubectl get deployments` first.
```sh
$ kubectl get services <service_name>
```

## List pods
```sh
$ kubectl get pods
```

## Delete pod
Usually you'd want to delete a _deployment_ though, but if you want to delete a
pod do:
```sh
$ kubectl get pods
# kubectl delete pod <pod_name>
```

## Echo cluster info
Echo the DNS, dashboard, Heapster and other stuff. Useful if you wanna login to
the dashboard:
```sh
$ kubectl cluster-info
```

## View logs
Get the pod name using `kubectl get pods` and then:
```sh
$ kubectl logs <pod_name>
```

## View config
```sh
$ kubectl config view
```

## View configuration
If you're trying to login to the admin view / dashboard:
```sh
# get dashboard url
$ kubectl cluster-info | grep kubernetes-dashboard | awk '{ print $5 }'

# get configuration; grab the values under "users" to log into the admin UI
$ kubectl config view
```

## Scaling deployments
```sh
$ kubectl scale deployment <deployment_name> --replicas=4
$ kubectl get deployment
$ kubectl get pods
```

## Deleting deployments
```sh
$ kubectl get deployments
$ kubectl delete deployments <deployment_name>
```

## Create a secret from a file
```sh
$ kubectl create secret generic <secret_name> \
  --from-file=ssh-privatekey=/path/to/.ssh/id_rsa \
  --from-file=ssh-publickey=/path/to/.ssh/id_rsa.pub
```
Or from a `.yaml` file:
```sh
$ kubectl create secret -f ./my-secret.yml
```

## Create a deployment from a file
```sh
$ kubectl apply -f ./my-deployment.yaml
```

## Roll back a deployment
```sh
$ apply -f docs/user-guide/bad-nginx-deployment.yaml
$ kubectl get rs
$ kubectl get pods
$ kubectl describe deployment
$ kubectl rollout history deployment/nginx-deployment
$ kubectl rollout history deployment/nginx-deployment --revision=2
$ kubectl rollout undo deployment/nginx-deployment
$ kubectl rollout undo deployment/nginx-deployment --to-revision=2
```

## Debugging pods and containers
Sometimes things break and you need to debug stuff. For example SSH in:
```sh
$ kubectl get po
$ kubectl describe pods ${POD_NAME}
```

- http://kubernetes.io/docs/user-guide/debugging-pods-and-replication-controllers/

## See Also
- http://kubernetes.io/docs/getting-started-guides/
- http://kubernetes.io/docs/hellonode
- http://kubernetes.io/docs/user-guide/ui/
- http://kubernetes.io/docs/admin/high-availability/
