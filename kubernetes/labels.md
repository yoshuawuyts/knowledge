# labels
Kubernetes is big on naming things. The same group of containers can have
multiple cross-sections which means a cluster can be optimally utilized.

## names
```txt
# deployment stages
stage: test      # temporary stuff that's separate from the other stuff
stage: staging   # this is what's staged to be live later. Like a beta
stage: live      # production is a weird word, use live instead

# tier
tier: public    # all stuff that's public-facing
tier: private   # all our internal crunching stuff
tier: tooling   # tooling to support the things

# uniquely named things
app: <name>    # define what the thing is running e.g. "postgres", "nginx"
role: <name>   # define the role it plays in the system e.g. "auth", "gateway",
                 "database", "cache"
```
- deployment stages might also benefit from namespaces, as the services can
  then be accessed through selectors like `postgres.test` and
  `postgres.staging`.
