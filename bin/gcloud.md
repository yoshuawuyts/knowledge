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

## Create a new container cluster
```sh
$ gcloud container clusters create <cluster_name>
```

## Tear down a container cluster
```sh
$ gcloud container clusters delete <cluster_name>
```

## See Also
- [bin/helm](./helm)
