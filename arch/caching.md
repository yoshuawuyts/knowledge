# caching
Caching is what makes servers not die. Be nice; keep your server alive.

## collapsed forwarding
- bundle multiple requests into a single request
- prevents accidental proxy ddos (as possible by cloudfront)

## etag
- hash of data
- compare all the time
- `302`!

## heavy objects
- large cachable object is better than multiple uncachable ones
- querystrings are the son of the devil
- put a cache in front of the large response to cache it regardless
- `302`'s are your friend

## API-gateway in front of the API
- authenticate
- cache routes
- custom rules
