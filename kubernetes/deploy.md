# deploy

## Update running deployment container
From CI / CD you might want to update a deployment; you can update any existing
deployment by changing the running container. The deployment will then rebuild
the whole thing. It could be that repliction controllers aren't cleaned up
because of k8@1.3 the garbage collector isn't enabled by default yet.
```sh
$ kubectl set image deployment/form nginx="nginx:1.9.1"
```
