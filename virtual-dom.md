# virtual-dom

## state

## widgets
Sometimes the DOM needs to be manually modified. This is where widgets come in
to play. They're created a bit differently from regular virtual-dom components.

```js
const createElement = require('virtual-dom/create-element')
const patch = require('virtual-dom/patch')
const diff = require('virtual-dom/diff')

module.exports = Widget

function Widget (vnode) {
  this.currVnode = vnode
}

Widget.prototype.init = function () {
  const el = createElement(this.currVnode)
  const container = document.createElement('div')
  container.appendChild(el)
  return container
}

Widget.prototype.update = function (prev, el) {
  const prevVnode = prev.currVnode
  const currVnode = this.currVnode

  const patches = diff(prevVnode, currVnode)
  const rootNode = el.childNodes[0]
  const newNode = patch(rootNode, patches)
  if (newNode !== el.childNodes[0]) {
    el.replaceChild(newNode, el.childNodes[0])
  }
}
```

- [virtual-dom/docs/widgets](https://github.com/Raynos/mercury/blob/master/docs/widgets.md)
