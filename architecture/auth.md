# auth
Authentication, credentials and such.

- __Authentication__ - is the process of verification that an individual,
  entity or website is who it claims to be. Authentication in the context of
  web applications is commonly performed by submitting a user name or ID and
  one or more items of private information that only a given user should know.
  [(source)](https://www.owasp.org/index.php/Authentication_Cheat_Sheet)

- __Session Management__ - is a process by which a server maintains the state
  of an entity interacting with it. This is required for a server to remember
  how to react to subsequent requests throughout a transaction. Sessions are
  maintained on the server by a session identifier which can be passed back and
  forward between the client and server when transmitting and receiving
  requests.  Sessions should be unique per user and computationally very
  difficult to predict.
  [(source)](https://www.owasp.org/index.php/Authentication_Cheat_Sheet)

## Cookies
### cookie attributes
- __Secure:__ only send cookie on HTTPS connection. Prevents man in the middle
  attacks.
- __HttpOnly:__ don't allow access to cookies through JS / DOM.
- __Domain:__ send cookie to specified domain + subdomain instead of origin
  server.
- __Path:__ only send cookie to specified directory or subdirectory (path /
  resource) within the web app.
- __Max-Age:__ preferable over `Expires`. Makes cookies persist, and not
  disappear when browser window is closed.
- __Expires:__ expire a cookie on a date.

## OAuth
Oauth does 2 things:
- allow a resource to talk to other resources (e.g. inter-app communication)
- allow a 3rd party client to talk to a resource (e.g. custom clients)

Architecturally there are 2 parts: the authorization server which issues
tokens, and the API which requires tokens.
```
     +--------+                               +---------------+
     |        |--(A)- Authorization Request ->| Authorization |
     |        |                               |     Server    |
     |        |<-(B)-- Authorization Grant ---|               |
     |        |                               +---------------+
     |        |
     |        |                               +---------------+
     |        |--(C)-- Authorization Grant -->| Authorization |
     | Client |                               |     Server    |
     |        |<-(D)----- Access Token -------|               |
     |        |                               +---------------+
     |        |
     |        |                               +---------------+
     |        |--(E)----- Access Token ------>|    Resource   |
     |        |                               |     Server    |
     |        |<-(F)--- Protected Resource ---|               |
     +--------+                               +---------------+
```
### authorization grant
4 types of authorizations can be granted by the auth server:
- __auth code:__ client -> resource -> auth server (creates auth code) ->
- __implicit:__
- __resource owner password credentials:__
- __client credentials:__

### performing a request
Either with a querystring or with a header. Header is preferable, qs is useful
for quick browser testing.
```sh
$ curl https://api.github.com/user?access_token=OAUTH-TOKEN
```
```sh
$ curl -H "Authorization: token OAUTH-TOKEN" https://api.github.com/user
```

### redirect uri
`redirect_uri` is optional, as applications should have a default
`redirect_uri`. When specified, port, protocol and route should match up
perfectly, only sub paths are allowed.

### oauth scopes
Scopes limit what actions can be performed by a token. It's good practice to
log them out in the response headers.
```
curl -H "Authorization: token OAUTH-TOKEN" https://api.github.com/users/technoweenie -I
HTTP/1.1 200 OK
X-OAuth-Scopes: repo, user
X-Accepted-OAuth-Scopes: user
```
- `X-OAuth-Scopes` - list scopes your token has authorized.
- `X-Accepted-OAuth-Scopes` - list scopes action checks for.

A good pattern of defining scopes is by  `<domain>` / `<domain>:<sublevel>`.

### errors
[ tbi ]

### links
- [developers.github/oauth](https://developer.github.com/v3/oauth/)
- [oauth.net](http://oauth.net/)
- [four attacks on oauth](http://software-security.sans.org/blog/2011/03/07/oauth-authorization-attacks-secure-implementation)
- [oauth2 and cookie convergence](https://www.subbu.org/blog/2010/09/oauth-2-0-and-cookie-convergence)
- [oauth access tokens vs session key](http://security.stackexchange.com/questions/20222/oauth-access-token-vs-session-key)

## Single Sign On (SSO)
```txt
req client -> authorization grant -> req server
req server -- authorization grant -> resource owner
req server <- authorization grant -- resource owner
req server -- authorization grant -> authorization server
req server <- access token --------- authorization server
req client <- access token --------- req server
req client -- access token --------> resource server
req client <- protected resource --- resource server
```
### tokens
- __access token:__
- __refresh token:__

### internal services
When using internal OAuth for internal services, it's easier to use a
whitelist than ask the user for approval. Though generally this shouldn't be
needed.

### single sign out
The other SSO is single sign out. When a user signs out of a service on a
domain, they should be signed out of all services on the domain. This can be
somewhat hard to achieve when applications.
- [ietf/oauth2.0](http://tools.ietf.org/html/rfc6749)
- [blog.heroku/oauth-sso](https://blog.heroku.com/archives/2013/11/14/oauth-sso)
- [hueniverse/twitter-oauth-sso](http://hueniverse.com/2009/04/16/introducing-sign-in-with-twitter-oauth-style-connect/)
- [stormpath/what-is-oauth](https://stormpath.com/blog/what-the-heck-is-oauth/)
- [stormpath/oauth-is-not-sso](https://stormpath.com/blog/oauth-is-not-sso/)

## JSON Web Token (JWT)
Self-container, safe.

Every JWT contains 3 parts:
- first 36 bits are info
- bits in the middle are the JSON payload
- last 43 bits are the signature hash

The payload should have the following values:
- __iss:__ who issued the token (issuer)
- __exp:__ token expiry
- __scope:__ which resources can be accessed
- __sub:__ subject to who the token was issued

## Cross-Site Request Forgery (CSRF)
Because cookies are included on every call to a domain, regardless from which
domain that call occurs, it can be abused to do bad things.

### synchronizer token design pattern
To prevent bad things, the classic approach is to use CSRF tokens. A CSRF token
can be the same as the session, and should generally be stored in the session
storage.
- [owasp/csrf](https://www.owasp.org/index.php/Cross-Site_Request_Forgery_%28CSRF%29)
- [whitehatsec/session-token](https://blog.whitehatsec.com/tag/session-token/)
- [blog.jdriven/stateless-csrf](http://blog.jdriven.com/2014/10/stateless-spring-security-part-1-stateless-csrf-protection/)
- [mdn/sessionStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)

## See Also
- [authentication cheat sheet](https://www.owasp.org/index.php/Authentication_Cheat_Sheet)
