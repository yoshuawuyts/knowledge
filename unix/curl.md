# curl

```
-X <HTTP_VERB> ................ use a certain http verb
--data-binary <data>  ......... keep data as-is, don't attach newlines
--cookie <token> .............. pass in a cookie
-H <header> ................... add a header
-d <data> ..................... pass in data
-L ............................ follow redirects
```

## Cookies
Can be retrieved from the chrome network tab.

```sh
$ curl localhost:8080 -b 'CSRF_token=asdf'
```

## Headers
```
curl -H "Content-Type: application/json" http://localhost:3000
```

## Data
```sh
curl -H "Content-Type: application/json" -d \
'{"username":"xyz","password":"xyz"}' http://localhost:3000/api/login
```
or with `--data-binary`
```sh
curl --data-binary @myFile.json http://localhost:3000/api
```

## Log status code only
```sh
$ curl localhost:8080 --silent --write-out "\n%{http_code}\n" | sed -n '$p'

# and to get the response body too
$ curl localhost:8080 --silent --write-out "\n%{http_code}\n" | sed -n '$d'
```

## See Also
- [curl(1)](http://man.cx/curl)
