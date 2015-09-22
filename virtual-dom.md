# virtual-dom

## state

## widgets
Sometimes the DOM needs to be manually modified. This is where widgets come in
to play. They're created a bit differently from regular virtual-dom components.

```js
const createElement = require('virtual-dom/create-element')

module.exports = Widget

function Widget (vnode) {
  this.currVnode = vnode
}

Widget.prototype.init = function () {
  const el = createElement(vnode);
  const container = document.createElement('div');
  container.appendChild(el);
  return container;
}

Widget.prototype.update = function () {
  const prevVnode = prev.currVnode;
  const currVnode = this.currVnode;

  const patches = diff(prevVnode, currVnode);
  const rootNode = elem.childNodes[0];
  const newNode = patch(rootNode, patches);
  if (newNode !== elem.childNodes[0]) {
      elem.replaceChild(newNode, elem.childNodes[0]);
  }
}
```

- [virtual-dom/docs/widgets](https://github.com/Raynos/mercury/blob/master/docs/widgets.md)
