# virtual-dom

## state

## widgets
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
gives a chance to update state through `this`.

- [virtual-dom/docs/widgets](https://github.com/Raynos/mercury/blob/master/docs/widgets.md)
- [mercury/docs/how-to-do-custom-rendering](https://github.com/Raynos/mercury/blob/master/docs/faq.md#how-do-i-do-custom-rendering)
