# docker

## Terminology
- __container:__
- __image:__

## Run a container
```sh
# daemon mode
$ docker run -d -p 80:80 --name webserver nginx

# interactive
$ docker run -t --rm <image_id> sh
```

## Attach to running container
```sh
$ docker exec -it <image_id> sh
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

## Example node app
```docker
FROM mhart/alpine-node:6

# Create app directory
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Install app dependencies
COPY package.json /usr/src/app/
RUN npm install

# Bundle app source
COPY . /usr/src/app

EXPOSE 8080
CMD [ "npm", "start" ]
```

## Docker ADD vs COPY
- `ADD` performs tar auto extraction, and can target URLs
- `COPY` is less fancy therefore more safe
- http://stackoverflow.com/questions/24958140/docker-copy-vs-add

## Inspect docker properties
Stuff like labels don't show up in `$ docker images`:
```sh
$ docker inspect <image_name>
```

## Remove all stopped containers
```sh
$ docker rm "$(docker ps -a -q)"
```

## Remove all untagged images
```sh
$ docker rmi "$(docker images | grep "^<none>" | awk '{ print $3 }')"
```

## See Also
- [the challenges of container configuration](https://speakerdeck.com/garethr/the-challenges-of-container-configuration)
