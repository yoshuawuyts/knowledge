# docker

## Run a container
```sh
$ docker run -d -p 80:80 --name webserver nginx
```

## Attach a volume
```sh
$ docker run \
  -v /home/<user>/.ssh:/root/.ssh \
  -e BUILDKITE_AGENT_TOKEN="<token>" \
  buildkite/agent
```
- https://docs.docker.com/v1.8/userguide/dockervolumes/
- https://github.com/buildkite/docker-ssh-env-config/blob/master/ssh-env-config.sh

## Docker in Docker
To run Docker, it must have access to `/var/run/docker.sock`. This provides API
access to stuff. To run docker inside of docker, just mount the socket as a
volume inside the container, and it can instrument containers on the root
machine as expected (and have full access to the API).
```sh
$ docker run -v /var/run/docker.sock:/var/run/docker.sock buildkite/agent
```
