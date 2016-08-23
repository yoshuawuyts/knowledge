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

## Get latest deployment status
```sh
$ kubectl rollout status deployment/<deployment_name>
```

## See Also
- http://kubernetes.io/docs/user-guide/deployments/#creating-a-deployment
