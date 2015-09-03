# web components
A quick overview of how web components work. Web components are the umbrella
term for a bunch of technologies, but based on conversations with people who've
used them extensively (`>1 year`) the only interesting parts are `custom
elements` and `shadow dom`.

## Status
### [ 03/09/15 ]
I first wrote this section as I was searching for reusability patterns.
custom elements seemed like a good solution to reusability. After all: smart
people were using it and it was aimed at modularity.

After having tried and used custom elements, I'm now __against the usage of web
components within applications__. Custom elements should only be used to
implement elements that could have been browser built-ins. For every other use
case the OO approach doesn't work well.

## Create
Custom elements are registered with a name as the first argument, and a set of
options as the second:
```js
const CustomButton = document.registerElement('custom-button', {
  prototype: Object.create(window.HTMLButtonElement.prototype),
  extends: 'button'
})
```
The `custom-button` can now be called in one of two ways:
```js
// a function was returned
const customButton = new CustomButton()

// an existing element was extended
document.createElement('custom-button')
document.createElement('button', 'custom-button')
```
and because we're extending we can do in html:
```html
<button is="custom-button">foobar</button>
```

## Manage
Custom elements fire lifecycle events:
```js
.createdCallback()                       // after element was created
.attachedCallback()                      // after element was attached to DOM
.detachedCallback()                      // after element was detached from dom
.attributeChangedCallback(attr, old, nw) // on element attribute change
```
An example of a reusable self contained updating time element:
```js
let CustomTimeProto = Object.create(window.HTMLTimeElement.prototype)
let timeEls = []
let raf = null

// register element
window.customTimeElement = document.registerElement('custom-time', {
  prototype: CustomTimeProto,
  extends: 'time'
})

// register the element, so multiple instances use the
// same update (poor man's RAF) loop
CustomTimeProto.createdCallback = function () {
  timeEls.push(this)
}

// initialize the update loop
CustomTimeProto.attachedCallback = function () {
  if (!raf) raf = setInterval(updateTimeEls, 60 * 1000)
}

// unregister the element, and destroy the update
// loop if it's void
CustomTimeProto.detachedCallback = function () {
  const ix = timeEls.indexOf(this)
  if (ix !== -1) timeEls.splice(ix, 1)
  if (!timeEls.length) raf = null
}

// change the text in the element to show
// how much it diffs compared to the
// `datetime` property on the element
CustomTimeProto.recalc = function () {
  const start = this.getAttribute('datetime')
  return new Date() - start
}

// iterate over all registered els
// and update the `textContent`
function updateTimeEls () {
  timeEls.forEach((time) => {
    time.textContent = time.recalc()
  })
}
```

## Data sharing in an application
In a real application there's a hierarchy in the way your application is
structured. Components are consumed by views, and views are directed by a
router. Also: data must be retrieved from sources, and to prevent duplicate
calls from happening it's preferable that the data is shared. The
`custom-element` module enables attaching multiple listeners to a single event,
which allows a separation between module-level listeners and application-level
listeners.

An important note is that with webcomponents all injected data should be shared
through the attributes, analogous to react's `props`.

An example of a `flux` application component (not on the module level):

```js
const customElement = require('custom-element')
const emitter = require('@ns/my-event-emitter')

// we're extending the custom element that we defined in the previous
// example to listen to changes in our event emitter. Whenever the
// timezone changes we update the `datetime` attribute with the
// corresponding offset.
var TimezoneTimeElement = customElement(window.customTimeElement)
TimezoneTimeElement.on('attached', function () {
  emitter.on('timezone:change', (timezoneOffset) => {
    const datetime = this.getAttribute('datetime')
    this.setAttribute('datetime', datetime + timezoneOffset)
  })
})

document.registerElement('timezone-time-element', TimezoneTimeElement)
```

## High performance components
Sometimes regular dom updates are too slow, and you need a more performant way
of rendering elements. There are two choices for this: `virtual-dom` and
`webGL`. WebGL is it's own programming language with various nooks and crannies
(you render pixels, not elements) so usually `virtual-dom` will be your go-to
tool in these kinds of situations. By using `virtual-dom` within web
components you combine high performance data rendering with self-encapsulation.

#### unresolved questions
- how do you pass data into a complex webcomponent? (don't want enormous lines
  of json injected into an html property).
- is it possible to expose both a `js` + `html` api for `virtual-dom` nodes?
  That way the amount of render loops can be minimized.

[examples tbd]

## Testing
In order to guarantee correctness of self-contained elements, they must be
tested. Examples below are extracted from
[`github/time-element`](https://github.com/github/time-elements/tree/master/test):

#### Test element creation
Create the element and test its attributes
```js
// from document.createElement()
const time = document.createElement('time', 'local-time')

// from constructor
const time = new window.LocalTimeElement()
```

#### Test attributes on creation
```js
const time = new window.LocalTimeElement()
assert.equal(time.nodeName, 'TIME')
assert.equal(time.getAttribute('is'), 'local-time')
// also test for blank states, e.g. the behavior if an attribute is not set
```

#### Test results based on attributes
After having asserted that the element is indeed created as the right type, you
can proceed to check if the content of the element is what was expected based
on the input values. Module level components should be self-contained, which
means there should be no unwanted side effects (like triggering a router).
Examples:
##### simple
```js
const time = document.createElement('time', 'relative-time')
time.textContent = 'Jun 30'
time.setAttribute('datetime', 'bogus')
assert.equal(time.textContent, 'Jun 30')
```

##### complicated
```js
const now = new Date().toISOString();
const root = document.createElement('div')
root.innerHTML = '<time is="relative-time" datetime="' + now + '"></time>'
if ('CustomElements' in window) {
  window.CustomElements.upgradeSubtree(root)
}
assert.equal(root.children[0].textContent, 'just now')
```

## Reusable webcomponents
Probably the hardest part of building webcomponents is finding the right
abstractions for reusability. Since we're mostly extending prototypes it can be
hard to find the correct abstractions for good base classes. In userland this
is about the only interface you should need to expose:
```js
document.registerElement('search-sidebar', require('@ns/search-sidebar'))
const sidebar = document.createElement('aside', 'search-sidebar')
sidebar.on('query', () => /*do something*/)
```

## See Also
- [timoxley/polyfill-webcomponents](https://github.com/timoxley/polyfill-webcomponents)
- [github/time-elements](https://github.com/github/time-elements/blob/master/time-elements.js)
- [shadow dom tutorial](http://www.html5rocks.com/en/tutorials/webcomponents/shadowdom/)
- [caniuse/web-components](http://caniuse.com/#search=web%20components)
- [mdn/register-element](https://developer.mozilla.org/en-US/docs/Web/API/Document/registerElement)
- [introduction to custom elements](http://webcomponents.org/articles/introduction-to-custom-elements/)
- [how GH is using webcomponents in production](http://webcomponents.org/articles/interview-with-joshua-peek/)
- [requireio/custom-element](https://github.com/requireio/custom-element)
- [webcomponents/webcomponentsjs](https://github.com/webcomponents/webcomponentsjs)
- [async-form-element](https://github.com/josh/async-form-element)
