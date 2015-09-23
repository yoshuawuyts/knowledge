# virtual-dom

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
```txt
state-atom
state-channel
```

- [mercury/index.js/state](https://github.com/Raynos/mercury/blob/master/index.js#L77-L95)

## Render
```txt
virtual-dom/h
vdom-thunk
virtual-raf
```

## Wrapping vanilla DOM nodes
Sometimes you want to wrap an external piece of DOM machinery inside your
`virtual-dom` application. This is done using widgets. It allows you to create
a custom.

```js
module.exports = GoogleMapWidget

function GoogleMapWidget(initialPosition) {
  if (!(this instanceof GoogleMapWidget)) {
    return new GoogleMapWidget(initialPosition)
  }
  this.type = 'Widget'
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

- [virtual-dom/docs/widgets](https://github.com/Raynos/mercury/blob/master/docs/widgets.md)
- [mercury/docs/how-to-do-custom-rendering](https://github.com/Raynos/mercury/blob/master/docs/faq.md#how-do-i-do-custom-rendering)
