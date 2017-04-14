# kubernetes

## Terminology
- __images:__ each container in a pod has an image. Currently only `docker`
  images are supported.
- __pod:__ smallest deployable unit in a kube cluster. Consists of one or more
  images.
- __replica sets (rs):__ multi-pod hypervisor, makes sure just the
  right amount of pods are running at any given time.
- __deployments:__ sets of `pods` and `replication controllers`. Can be defined
  in a declarative format which describes the desired state
- __services:__ persistant wrapper around a `deployment`; provides policies,
  virtual network addresses and labels
- __node:__ single virtual or physical machine in a kube cluster
- __cluster:__ group of nodes firewalled from the internet
- __ingress resources:__ incoming traffic router

_note:_ `replication controllers (rc)` used to be on this list, but is
currently in the process of being superseded by `rs`. They should be similar in
most regards.

## SSL
- [kelseyhightower/docker-kubernetes-tls-guide](https://github.com/kelseyhightower/docker-kubernetes-tls-guide)

## Helm
`helm(1)` is to kube, what `homebrew` is to OS X.

## Labels
Labels are added onto system objects to provide a multi-tiered structure. The
following labels are enforced by `helm(1)`, and are probably a good idea to
rely on for non-helm kube structures too:
- __group:__ same role. E.g. frontend, api, data
- __provider:__ type of service provided. E.g. etcd, postgres, s3
- __mode:__ operation mode of the service. E.g. standalone, clustered,
  discovery

## Secrets
- [user-guide/secrets](http://kubernetes.io/docs/user-guide/secrets)
- https://github.com/kelseyhightower/kube-cert-manager

## See Also
- [bin/docker](../bin/docker.md)
- [bin/gcloud](../bin/gcloud.md)
- [bin/helm](../bin/helm.md)
- [bin/kubectl](../bin/kubectl.md)
- https://kubernetesbootcamp.github.io/kubernetes-bootcamp/index.html
