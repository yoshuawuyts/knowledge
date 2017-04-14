# loading
Getting performance out of networking in the browser is tricky, but luckily
there's plenty of primitives that allow us to have fine grained control over
things.

## Preload
Load a resource without blocking the page's ability from going interactive.

Here's how to immediately apply preloaded stylesheets:
```js
<link rel="preload" href="style.css" onload="this.rel=stylesheet">
```

## Prefetch
Load a resource eventually; no rush but it'll probably be used soonish

## HTTP/2 Push
Blunt, crude tool - not a fan.

## See Also
- https://medium.com/reloading/preload-prefetch-and-priorities-in-chrome-776165961bbf
