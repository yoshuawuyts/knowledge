# nginx
Nginx is a pretty good proxy, it's generally recommended to run it everywhere.

## Install
With `nix` do:
```sh
$ nix-env -i nginx-1.9.11
```

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

## signals
```sh
$ nginx -s {stop,quit,reopen,reload}
```

## point nginx to different conf file
```sh
$ nginx -p "$(pwd)/" -c './conf/nginx.conf'
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
