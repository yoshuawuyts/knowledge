# test

## Testing outbound services
Sometimes a part of your application must be tested that communicates with an
intricate external service (e.g. a third party api such as S3). There are three
approaches that can be taken:

- hit a service endpoint (usually somewhere on the internet)
- stub out the request at the code level, replacing the code with other code.
- catch the request at the network level and respond with a predetermined
  response.

Hitting actual endpoints does not work offline and leads to monolithic
architectures. It has by far the slowest feedback loop, and does not accurately
reflect whether or not expectations are being met (e.g. too many requests,
timeouts, etc.)

Stubbing code out causes tests to be bound to an _implementation_, which is
brittle and won't survive refactors. This de-incentivizes writing tests because
they give no guarantees.

Stubbing requests out causes tests to be to _behavior_, which does survive
refactors. By stubbing requests you set expectations of the outgoing request,
and reply with a certain response. It tests a range of expectations, and
verifies your application complies with those expectations.

You can capture network requests by using `nock`:
```js
const nock = require('nock')
nock.recorder.rec()
```
Which will then output all data to stdout.
