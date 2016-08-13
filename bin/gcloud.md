# gcloud
Documentation for the google cloud command.

## Bind new project to account
```sh
$ gloud config set project <project_name>
```

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

## Get cluster credentials
After creating a cluster using whichever method you prefer (`gcloud`, `ui`,
`terraform`), the credentials must be loaded into `kubectl`:
```sh
$ gcloud container clusters get-credentials <cluster_name>
```

## Get in-depth cluster information
Sometimes you want to see a little more. The `describe` command does just that:
```sh
$ gcloud container clusters describe <cluster_name>
```

## SSH into instance
```sh
$ gcloud compute instances list
$ gcloud compute ssh <instance_name> --zone=<instance_zone>
```
And to attach to a running container:
```sh
$ sudo docker ps -a
$ sudo docker exec -it <container_id> bash
```

## See Also
- [bin/helm](./helm)
