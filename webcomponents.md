# web components
A quick overview of how web components work. Web components are the umbrella
term for a bunch of technologies, but based on conversations with people who've
used them extensively (>1 year) the only interesting parts are `custom
elements` and `shadow dom`.

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
