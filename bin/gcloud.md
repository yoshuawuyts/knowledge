# gcloud
Documentation for the google cloud command.

## Upload a docker container
````sh
$ gcloud docker push gcr.io/<project_id>/hello-node:v1
```

## List configuration
```sh
$ gcloud config list
```

## Set availability zone
Just like AWS, gcloud has different availability zones:
```sh
$ gcloud config set compute/zone us-central1-b
```
To view which availability zones are available:
```sh
$ gcloud compute zones list
```

## Create a new container cluster
Create a new cluster of machines. Use the `-m` flag to specify a machine type,
e.g. `-m f1-micro`. Clusters are created in the current compute zone.
```sh
$ gcloud container clusters create <cluster_name>
$ gcloud container clusters create <cluster_name> -m <machine_type>
```

## List running container clusters
```sh
$ gcloud container clusters list
```

## Tear down a container cluster
```sh
$ gcloud container clusters delete <cluster_name>
```

## See Also
- [bin/helm](./helm)
