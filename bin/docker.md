# docker

## Terminology
- __container:__
- __image:__

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

## Labels
In order to determine where containers come from, strong labels are necessary.
After all: without technical controls you only have social guarantees. Label
standards should be decided upon and enforced.
```dockerfile
LABEL com.example.git.repository="https://github.com/my-org/my-repo"
      com.example.git.sha="7ed5fd94fa9e3b244c8fce56c7b721037e127829"
      com.example.build.time="2016-04-24T15:43:05+00:00"
      com.example.docs="https://github.com/my-org/my-repo/docs"
      com.example.dockerfile="/Dockerfile"
      com.example.api.packages="apk info -vv"
```
In addition to strong labels, it's also recommended to embed Dockerfiles in
images.

## Image management
```sh
$ docker rmi <image_name>
$ docker ps -a  # see all created containers
$ docker rm $(docker ps -aq)  # remove all existing containers
```

## See Also
- [the challenges of container configuration](https://speakerdeck.com/garethr/the-challenges-of-container-configuration)
