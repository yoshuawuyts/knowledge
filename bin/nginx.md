# nginx
Nginx is a pretty good proxy, it's generally recommended to run it everywhere.

## Install
With `nix` do:
```sh
$ nix-env -i nginx-1.9.11
```

## commands
```sh
$ nginx -s stop        # quit nginx server (SIGTERM)
$ nginx -s quit        # quit nginx server (SIGQUIT)
$ nginx -s reopen      # reopen nginx server (SIGUSR1)
$ nginx -s reload      # reload service (SIGHUP)
$ nginx -t file.conf   # assert file
```

## point nginx to different conf file
```sh
$ nginx -p "$(pwd)/" -c './conf/nginx.conf'
```

## Configuring a static server
Nginx uses namespaces and directives for its configuration. Directives for a
static server:
- __listen:__ the port the server will listen at
- __server\_name:__ match the URL and apply the rules on it
- __root:__ directory (static) files are stored in
- __location:__ takes string / regex and a block.
__try\_files:__ attempt to read files in a certain order. Also looks for
  `.html` extentions and matches those

```nginx
server {
  listen 80;
  server_name example.com;
  root /var/www/example;
  location / {
    try_files $uri $uri/ /index.html;
  }
}
```
- [nginx directives](http://nginx.org/en/docs/dirindex.html)

## match exact string with location
```nginx
location = / { ... }
```

## variables
- __$uri:__ uri that was received

## Example config
```nginx
events {
  worker_connections  1024;
}

http {
  server {
    listen 127.0.0.1:3000;
    server_name localhost;

    access_log /tmp/localhost.log;
    charset utf-8;

     location / {
       proxy_pass http://127.0.0.1:1337/;
       proxy_set_header X-Real-IP $remote_addr;
       proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
       proxy_set_header Host $http_host;
       proxy_set_header X-Nginx-Proxy true;

       proxy_redirect off;
    }
  }
}
```


## start nginx in non daemon mode
```nginx
# nginx.conf
daemon off;
```
- [nginx/core_module](http://nginx.org/en/docs/ngx_core_module.html)

## OpenResty
OpenResty allows extending nginx with lua to turn it into a full-fledged
webserver.

### directives
- access_by_lua
- content_by_lua
- header_filter_by_lua

## See Also
- [scripting nginx with lua](http://www.londonlua.org/scripting_nginx_with_lua/slides.html)
- [openresty/lua-nginx-module](https://github.com/openresty/lua-nginx-module/)
- [nginx introduction](http://carrot.is/coding/nginx_introduction)
