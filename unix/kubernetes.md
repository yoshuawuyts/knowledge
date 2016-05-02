# kubernetes

## Terminology
- __images:__ each container in a pod has an image. Currently only `docker`
  images are supported.
- __pod:__ smallest deployable unit in a kube cluster. Consists of one or more
  images.
- __replication controller (rc):__ multi-pod hypervisor, makes sure just the
  right amount of pods are running at any given time.
- __deployments:__ sets of `pods` and `replication controllers`. Can be defined
  in a declarative format which describes the desired state
- __services:__ persistant wrapper around a `deployment`; provides policies,
  virtual network addresses and labels
- __node:__ single virtual or physical machine in a kube cluster
- __cluster:__ group of nodes firewalled from the internet
- __ingress resources:__ incoming traffic router

## See Also
- [bin/kubectl](../bin/kubectl.md)
- [bin/gcloud](../bin/gcloud.md)
- [bin/docker](../bin/docker.md)
