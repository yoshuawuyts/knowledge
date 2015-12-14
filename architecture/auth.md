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
Cookies disappear by default when browser window is closed.
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
```
client -- authorization grant -> resource owner
client <- authorization grant -- resource owner
client -- authorization grant -> authorization server
client <- access token --------- authorization server
client -- access token --------> resource server
client <- protected resource --- resource server
```
- [four attacks on oauth](http://software-security.sans.org/blog/2011/03/07/oauth-authorization-attacks-secure-implementation)
- [oauth2 and cookie convergence](https://www.subbu.org/blog/2010/09/oauth-2-0-and-cookie-convergence)
- [oauth access tokens vs session key](http://security.stackexchange.com/questions/20222/oauth-access-token-vs-session-key)

## JSON Web Token (JWT)
[ tbi ]

## Cross-Site Request Forgery (CSRF)
- [owasp/csrf](https://www.owasp.org/index.php/Cross-Site_Request_Forgery_%28CSRF%29)

## See Also
- [authentication cheat sheet](https://www.owasp.org/index.php/Authentication_Cheat_Sheet)
