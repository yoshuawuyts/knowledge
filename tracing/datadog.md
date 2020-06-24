# datadog

## API gotchas

There are two different API endpoints for datadog:

- https://api.datadoghq.com
- https://api.datadoghq.eu

When constructing there needs to be a switch between the two.

## Authenticate

This step checks if the API key is valid. This is not required, but useful to
establish the API actually works.

```sh
curl -X GET https://api.datadoghq.com/api/v1/validate \
    -H "Content-Type: application/json" \
    -H "DD-API-KEY: ${DD_CLIENT_API_KEY}" 
```

This will return 200 on success, and 403 on failure. If a failure occurs the
shape of the object is:

```json
{
  "errors": [
    "Bad Request"
  ]
}
```

- https://docs.datadoghq.com/api/v1/authentication/

## Send logs
Send logs to datadog. Restrictions

- Up to 5mb total for the payload
- Up to 500 messages
- Up to 256kb per message

Can either send a single string as the message, or an array of strings. If it
was successful it returns `200` with an empty JSON object.

```sh
curl -X POST https://http-intake.logs.datadoghq.eu/v1/input \
    -H "Content-Type: application/json" \
    -H "DD-API-KEY: ${DD_CLIENT_API_KEY}" \
    -d @- << EOF
{}
EOF 
```
```json
{
  "ddsource": "nginx",
  "ddtags": "env:staging,service:payment",
  "hostname": "i-012345678",
  "message": "2019-11-19T14:37:58,995 INFO [process.name][20081] Hello World"
}
```
- https://docs.datadoghq.com/api/v1/logs/#send-logs
