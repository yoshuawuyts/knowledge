# DOM
The Document Object Model (DOM) is a well-defined convention that makes it possible
for different platforms and programming languages to interact with HTML documents.

Browsers makes use of the DOM through JavaScript.

# HTML
Writing semantic HTML was made a lot easier in HTML5, which included a lot of new
tags, such as `<nav>` and `<header>`. While most developers have some understanding
of how HTML and the DOM work, there are a good amount of HTML-tags available that are being misused.

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

### Propagation/bubbling
A big gotcha of DOM events it that they propagate to their parent nodes if left
unhandled. This is also known as event bubbling. By calling `.stopPropagation()`
on an event object the events are no longer propagated to parent nodes.

```html
<div class="parent-node">
  <div class="interactable-node"></div>
</div>

<script>
// Wait until the DOM has loaded before using it
  document.addEventListener('DOMContentLoaded', function () {
    document.querySelector('.interactable-node').addEventListener('click', function (event) {
        // don't let the parent node (<div class="parent-node">) know about this!
        event.stopPropagation();
    });
  });
</script>
```

### Event delegation
If you want to add custom events, e.g. click events, on content that is dynamically added
to the HTML page, for example chat messages in a live chat application, one would have to
explicitly add a new event listener for a message when it's added. That's not very effective..

Instead, one could listen to click events on a static element that contains the chat messages,
and from those events pinpoint exactly what was clicked (by e.g. the `.target` property of event objects).

This is known as event delegation.

### Native browser behavior
Another gotcha with the DOM, is that when you're building dynamic web applications
with JavaScript, existing DOM nodes will have default behavior that is
executed. For example: when submitting a form, the default behavior for a browser
is to send the form data in a new request, effectively reloading/redirecting the page
after submission. To prevent this from happening once can use the `.preventDefault()`
function of the [event](https://developer.mozilla.org/en/docs/Web/API/Event) object.

## Reflows & repaints
> A repaint occurs when changes are made to an elements skin that changes
> visibility, but do not affect its layout. Examples of this include outline,
> visibility, or background color. According to Opera, repaint is expensive
> because the browser must verify the visibility of all other nodes in the DOM
> tree.

> A reflow is even more critical to performance because it involves changes
> that affect the layout of a portion of the page (or the whole page).

- [source](http://stackoverflow.com/questions/2549296/whats-the-difference-between-reflow-and-repaint)
- [csstriggers.com](http://csstriggers.com/)

## Buttons
Creating a button can be done in multiple ways (all dependending on style).
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
- [Installable Web Apps with the WebApp Manifest in Chrome for Android](http://updates.html5rocks.com/2014/11/Support-for-installable-web-apps-with-webapp-manifest-in-chrome-38-for-Android)
- [Push notifications on the Open Web](http://updates.html5rocks.com/2015/03/push-notificatons-on-the-open-web)
- [Introduction to service workers](http://www.html5rocks.com/en/tutorials/service-worker/introduction/)
- [Notifications API](https://notifications.spec.whatwg.org/)
- [Push API](http://w3c.github.io/push-api/)

## Open links in new tab
```js
window.open(url, '_blank')
```
- [stackoverflow](http://stackoverflow.com/a/11384018/1541707)

## window.fetch
- Makes [AJAX](https://en.wikipedia.org/wiki/Ajax_(programming)) in the browser a [better experience](https://developers.google.com/web/updates/2015/03/introduction-to-fetch).
- Uses [promises](https://developer.mozilla.org/en/docs/Web/JavaScript/Reference/Global_Objects/Promise).

```js
window.fetch('http://localhost:1337')
  .then(function (res) {
    return res.text();
  })
  .then(function (text) {
    return text.toUpperCase();
  })
  .then(function (upperCaseText) {
    document.querySelector('h1').innerText = upperCaseText;
  })
  .catch(function (err) {
    console.error('Couldn\'t fetch page!', err); 
  });
```

### Caveats
- Only available in latest versions of modern browsers.
- Setting cookie headers is not possible with `fetch`. However, one can
include existing cookies by setting `{ credentials: 'include' }`.
```js
fetch('/something', { credentials: 'include' })
```

#### Read more
- [mdn/fetch#body](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch#Body)
- [mdn/Request.credentials](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)
