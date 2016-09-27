# daemonset
```json
{
  "apiVersion": "extensions/v1beta1",
  "kind": "DaemonSet",
  "metadata": {
    "name": "shell-daemon"
  },
  "spec": {
    "selector": {
      "matchLabels": {
        "tier": "tooling",
        "app": "shell",
        "role": "util"
      }
    },
    "template": {
      "metadata": {
        "labels": {
          "tier": "tooling",
          "app": "shell",
          "role": "util"
        }
      },
      "spec": {
        "restartPolicy": "Always",
        "containers": [
          {
            "name": "service-shell",
            "image": "yoshuawuyts/service-shell"
          }
        ]
      }
    }
  }
}
```

## See Also
- http://kubernetes.io/docs/admin/daemons/#what-is-a-daemon-set
- https://github.com/helm/charts/search?utf8=%E2%9C%93&q=daemonset&type=Code
