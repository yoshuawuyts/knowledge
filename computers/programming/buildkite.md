# buildkite

## wait for all other steps to complete first
```yaml
- command: 'command.sh'
- wait
- command: 'echo The command passed'
```

## create a blocking step
```yaml
- command: 'command.sh'
- block <label_name>
- command: 'deploy.sh'
```

This block step will only be available on `"master"` branches:
```yaml
- command: 'test.sh'
- block: ':rocket: Release'
  branches: "master"
- command: 'deploy.sh'
  branches: "master"
```
- https://buildkite.com/docs/agent/build-pipelines
