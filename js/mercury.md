# mercury
Mercury is a modular framework for client-side applications. It provides a sane
mechanism of managing state and re-renders. It concerns itself with rendering
and updating of views and managing data. However, it does not provide a
mechanism of retrieving data from servers. Because of Mercury's modular nature
it's possible to replace any part by another part, providing great flexibility
based around a minimal core.

## Architecture
Though at the time of writing there are no known instances of it being used in
production, the most sane way of using Mercury is as a glue layer only.

Using Mercury as a glue layer capitalizes on the benefits of Mercury without
having to buy into a complete framework (every framework will eventually
always be replaced by another framework).

The architecture is as follows:
- Create stand-alone elements with styling and event handlers using pure JS +
  DOM. The `custom-elements` specification is particularly useful for this, as
  it exposes standardized `create-update-delete` hooks.
- Create light wrappers around those elements to hook them into mercury. A
  wrapper should typically be only a few lines. This can probably be done as a
  module, allowing elements to be wrapped at the view level with minimal
  overhead.
- Create standalone hooks to the API, separating data retrieval into a
  standalone level. Think of it as an inverse API (e.g. client bindings to the
  API).
- Create application-specific views that combine several components into a
  view. These views shouldn't have any styling, as styling should live in
  components. The router should expose a method of passing child components
  into other components, providing composability.
- Wrap the data hooks into Mercury using the global events. The standalone
  components should now have access to these events through the wrappers. The
  the components can now update the application without needing to know the
  event names or the methods behind them.

This particular architecture treats mercury as pure glue; putting it in charge
of swapping views, delivering data and binding events but nothing else. Because
of this loosely coupled structure, both the components and framework can easily
be upgraded, events exist within the application only and modularity +
encapsulation become the baseline, rather than a far-off target.

__note__: Scepticists might argue that `React` / `virtual-dom` provide high
performance components, and thus having vanilla DOM elements live on the leaves
is bad for performance because `vdom`s are fast. Apparently these scepticists
don't realize that `vdom` implementations use native DOM methods under the
hood, and thus will be slower than raw DOM operations 100% of the time. The
main benefit `vdom`s provide is that it makes it easy to reason about whether
or not DOM operations should occur. By using `Mercury` in the glue layer only
we reap all the benefits of using a `vdom` state tree, without losing any perf.

## Update loop
Mercury's components use event delegation to trigger logic. On one side of the
application you declare...
```txt
   DOM
Raw Input  ->  Input
                 ▾
               State
                 ▾
Raw Output <-  Render
```
[source](http://eom.surge.sh/)

## Input
```txt
dom-delegator
value-event/*
```
- [mercury/issues/renaming-handles-and-events](https://github.com/Raynos/mercury/issues/118)

## State
State is probably the least evolved of all problems. Currently Mercury's
application state is a giant immutable object. This is very easy to reason
about, and relatively performant, but in high perf scenarios garbage collection
might become a problem. I've never run into this yet, but it __is__ something to
consider.
```txt
state-atom
state-channel
```
- [mercury/index.js/state](https://github.com/Raynos/mercury/blob/master/index.js#L77-L95)

## Render
__note__: The hyperscript syntax is efficient, but slightly awkward. Work is
needed to make a template-string HTML -> hyperscript transform, preferably
working both as a browserify transform and standalone. We don't want to use JSX
as it's ugly, breaks tooling and can easily be replaced by template strings.

__note__: Routing isn't done yet. `mercury-router` is efficient, but less good
than something like `react-router`. `react-router` itself is bloated, but has
good ideas. Work is needed for a better router (haven't found the time yet).
```txt
virtual-dom/h
vdom-thunk
virtual-raf
```

## Wrapping vanilla DOM nodes
__edit__: `virtual-widget` now exists, providing a simplified interface for
widgets.

Sometimes you want to wrap an external piece of DOM machinery inside your
`virtual-dom` application. This is done using widgets. It allows you to create
a custom.

```js
module.exports = GoogleMapWidget

function GoogleMapWidget(initialPosition) {
  if (!(this instanceof GoogleMapWidget)) {
    return new GoogleMapWidget(initialPosition)
  }
  this.type = 'Widget' // setting the type prevents errors
  this.position = initialPosition
}

GoogleMapWidget.prototype.init = function () {
  var elem = document.createElement('div')
  this.map = GoogleMap(elem)
  this.map.setPosition(this.position)
  return elem
}

GoogleMapWidget.prototype.update = function (prev, elem) {
  this.map = this.map || prev.map
  this.map.setPosition(this.position)
}
```
The first time `init()` is called it should return a DOM element. The DOM
element can be mutated freely as `virtual-dom` will never touch it.

`update()` is called if the widget was available in the previous tree, and
gives a chance to update state through `this`. In practice this means that an
immutable state object or data store can be queried for the latest data and the
new render can take place based on that. So unlike in React, `virtual-dom`
doesn't rely on new properties to flow through to update widgets.

### Styling
Components should by styled using `sheetify` as a `browserify` transform. As of
writing the transform version works, but isn't published yet. Until publishing
`css-next` style stylesheets + stylesheet aggregation should suffice (possibly
with some BEM); post-publish conflicts will be resolved by automatic
namespacing. Don't use `css-modules` as it relies on the filesystem, requires
non standard extensions to the language and thus locks you into a framework.

- [virtual-dom/docs/widgets](https://github.com/Raynos/mercury/blob/master/docs/widgets.md)
- [mercury/docs/how-to-do-custom-rendering](https://github.com/Raynos/mercury/blob/master/docs/faq.md#how-do-i-do-custom-rendering)
