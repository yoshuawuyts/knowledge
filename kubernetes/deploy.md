# deploy

## Update running deployment container
From CI / CD you might want to update a deployment; you can update any existing
deployment by changing the running container. The deployment will then rebuild
the whole thing. It could be that repliction controllers aren't cleaned up
because of k8@1.3 the garbage collector isn't enabled by default yet.
```sh
$ kubectl set image deployment/<deployment_name> \
  <container_name>="<container_name>:<version>"

# example
$ kubectl set image deployment/nginx nginx="nginx:1.9.1"
```

To prevent caching of a prior image, the `imagePullPolicy` must be set to
`Always`:
```yml
apiVersion: extensions/v1beta1
kind: Deployment
metadata:
  name: nginx
spec:
  replicas: 1
  template:
    metadata:
      labels:
        run: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.9.1
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
```

If a container with the same image is deployed, a new deployment will not be
triggered. To force a new rollout annotations should be updated so that a new
rollout is forced.
```sh
$ kubectl patch deployment web -p \
  "{\"spec\":{\"template\":{\"metadata\":{\"labels\":{\"date\":\"`date +'%s'`\"}}}}}"
```
- https://github.com/kubernetes/kubernetes/issues/27081

## Get latest deployment status
```sh
$ kubectl rollout status deployment/<deployment_name>
```

## View deployment history
```sh
$ kubectl rollout history deployment/<deployment_name>
```

## See Also
- http://kubernetes.io/docs/user-guide/deployments/#creating-a-deployment
