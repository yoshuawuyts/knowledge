# dom
While most developers have some understanding of how HTML and the DOM work,
there are a good amount of tags available that are most likely being misused. I
don't care about non-evergreen browsers, so that's the only guarantee about
compatibility I'm able to make.

## Tags
```txt
<nav>       the main navigation controls
<header>    the introductory block at the start of the page
<main>      the main content, should cover most of the page
<section>   the different parts of main
<footer>    page end, credits and stuff
<aside>     the sidebar, if any exists
```

## Events
A big gotcha of `dom` events it that they propagate to their parent nodes if left
unhandled. By setting `this.stopPropegation()` the events are no longer
propagated to their parents.

Another gotcha with the `dom`, is that when you're building dynamic systems
with JavaScript, existing dom nodes will have default behavior that is
executed. For example: when submitting a form, the default behavior is to
refresh the page after submission. To prevent this from happening you have to
use the `this.preventDefault()` method within the form element.

## Reflows & repaints
> A repaint occurs when changes are made to an elements skin that changes
> visibility, but do not affect its layout. Examples of this include outline,
> visibility, or background color. According to Opera, repaint is expensive
> because the browser must verify the visibility of all other nodes in the DOM
> tree.

> A reflow is even more critical to performance because it involves changes
> that affect the layout of a portion of the page (or the whole page).

- [source](http://stackoverflow.com/questions/2549296/whats-the-difference-between-reflow-and-repaint)
- [csstriggers.com](http:‰//csstriggers.com/)

## Buttons
Creating a button can be done in multiple ways (all depending on style).
Function comes down to semantics. Best practices for this are:
- __a__: use anchor tags for external links
- __button__: use buttons for all non-form related actions
- __input__: use input tags for all form related actions

[source](http://davidwalsh.name/html5-buttons)

## Push notifications on the web
Create a manifest in `index.html`:
```html
<link rel="manifest" href="/manifest.json">
```
- [installable Web Apps with the WebApp Manifest in Chrome for Android](http://updates.html5rocks.com/2014/11/Support-for-installable-web-apps-with-webapp-manifest-in-chrome-38-for-Android)
- [push notifications on the Open Web](http://updates.html5rocks.com/2015/03/push-notificatons-on-the-open-web)
- [introduction to service workers](http://www.html5rocks.com/en/tutorials/service-worker/introduction/)
- [notifications api](https://notifications.spec.whatwg.org/)
- [push api](http://w3c.github.io/push-api/)

## Open links in new tab
```js
window.open(url, '_blank')
```
- [stackoverflow](http://stackoverflow.com/a/11384018/1541707)

## window.fetch
Uses promises, first `.then` call defines in what form the data will be
displayed. The value can be one of `arrayBuffer()`, `blob()`, `formData()`,
`json()` and `text()`.
```js
window.fetch('http://localhost:1337')
  .then(function (res) { return res.text()})
  .then(function (res) {
    console.log(res)
  })
```

Setting cookie headers is not possible with `fetch`. Instead cookies can be set
through the `credentials` api. The value can be one of `omit`, `same-origin`,
`include` where `omit` is the default.
```js
fetch('/something', { credentials: 'same-origin' })
```

- [mdn/fetch#body](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch#Body)
- [mdn/Request.credentials](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)

## Append class in JS
```js
const node = document.querySelector('.foo')
node.classList.remove('ugly')
node.classList.add('pretty')
node.classList.toggle('hide')
```

## Create popup banner
__iOS__
```html
<meta
  name="apple-itunes-app"
  content="app-id=myAppStoreID, affiliate-data=myAffiliateData, app-argument=myURL">
```

__android__
```json
"prefer_related_applications": true,
"related_applications": [
  {
  "platform": "play",
  "id": "com.google.samples.apps.iosched"
  }
]
```
- [developers.google.com/app-install-banners](https://developers.google.com/web/updates/2015/03/increasing-engagement-with-app-install-banners-in-chrome-for-android#native)
- [developer.apple.com/smart-banners](https://developer.apple.com/library/mac/documentation/AppleApplications/Reference/SafariWebContent/PromotingAppswithAppBanners/PromotingAppswithAppBanners.html)

## media queries in JS
Using [window.matchMedia](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)
```js
if (window.matchMedia("(min-width: 400px)").matches) {
  /* the viewport is at least 400 pixels wide */
} else {
  /* the viewport is less than 400 pixels wide */
}
```

## set values in node
- `node.texContent` for text
- `node.innerHTML` for HTML

## File upload
```js
hx`
  <form>
    <input type="file" onchange=${uploadFile}>
  </form>
`

function uploadFile (e) {
  const el = e.srcElement
  const form = new window.FormData()
  const files = el.files
  for (var i = 0; i < files.length; i++) {
    const file = el.files[i]
    console.log(file)
    form.append(file.name, file)
  }

  const opts = {
    uri: 'http://localhost:1337/torrent',
    body: form
  }
  xhr.post(opts, function (err, resp, body) {
    el.value = ''
    if (err) return console.error(err)
  })
}
```
- [working with files in JS](https://www.nczonline.net/blog/2012/05/08/working-with-files-in-javascript-part-1/)
- [drag-drop](https://github.com/feross/drag-drop)
- [filepicker-element](https://github.com/shama/filepicker-element)
- [MDN/formdata](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)
