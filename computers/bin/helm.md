# helm
Helm is the Kubernetes config manager

## Concepts
Helm packages applications for consumption within kubernetes. Each package is
called a `chart` (keep the ship analogies going).

## Installing helm
```sh
$ brew install kubernetes-helm  # OS X
$ curl -s https://get.helm.sh | bash  # linux
```

## Updating charts
The charts index should be updated periodically:
```sh
$ helm update
```

## Search
Search for available charts:
```sh
$ helm search <keyword>
$ helm search redis  # example
```

## Investigate
```sh
$ helm info <chart_name>
```

## Install
```sh
$ helm install <chart_name>
$ helm install redis-cluster
```

## Manage repos
```sh
$ helm repo add <repo_name> <repo_git_url>
$ helm repo add mycharts https://github.com/dev/mycharts  # example
$ helm repo list
```

## See Also
- http://helm.sh/
- https://github.com/kubernetes/helm
- https://github.com/helm/helm
- https://deis.com/blog/2015/introducing-helm-for-kubernetes/
- https://deis.com/blog/2015/why-kubernetes-needs-helm/
- https://github.com/buildkite/helm-charts
