# rendering
Rendering in the browser is somewhat of a fine art. Knowing when and how to
break up render calls is important.

## Animations
### will-change attribute
To mark that an element will be animated, the `will-change: transform` property
in CSS is useful. The browser already tries and figure this out bey default,
but in the case a (delayed) animation is causing a performance bottleck, this
should help optimize. It can cause visual changes, however.
- https://developer.mozilla.org/en/docs/Web/CSS/will-change

## Forced layout
### Element
##### Box metrics
- `elem.offsetLeft`, `elem.offsetTop`, `elem.offsetWidth`, `elem.offsetHeight`, `elem.offsetParent`
- `elem.clientLeft`, `elem.clientTop`, `elem.clientWidth`, `elem.clientHeight`
- `elem.getClientRects()`, `elem.getBoundingClientRect()`

##### Scroll stuff
- `elem.scrollBy()`, `elem.scrollTo()`
- `elem.scrollIntoView()`, `elem.scrollIntoViewIfNeeded()`
- `elem.scrollWidth`, `elem.scrollHeight`
- `elem.scrollLeft`, `elem.scrollTop` also, setting them

##### Focus
- `elem.focus()` can trigger a *double* forced layout
  ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/dom/Element.cpp&q=updateLayoutIgnorePendingStylesheets%20-f:out%20-f:test&sq=package:chromium&l=2369&ct=rc&cd=4&dr=C))

##### Also…
- `elem.computedRole`, `elem.computedName`
- `elem.innerText`
  ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/dom/Element.cpp&q=updateLayoutIgnorePendingStylesheets%20-f:out%20-f:test&sq=package:chromium&l=2626&ct=rc&cd=4&dr=C))

### getComputedStyle
`window.getComputedStyle()` will typically force style recalc
([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/dom/Document.cpp&sq=package:chromium&type=cs&l=1860&q=updateLayoutTreeForNodeIfNeeded))

`window.getComputedStyle()` will force layout, as well, if any of the following
is true:

1. The element is in a shadow tree
1. There are media queries (viewport-related ones). Specifically, one of the following: ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/css/MediaQueryExp.cpp&sq=package:chromium&type=cs&l=163&q=MediaQueryExp::isViewportDependent))
  * `min-width`, `min-height`, `max-width`, `max-height`, `width`, `height`
  * `aspect-ratio`, `min-aspect-ratio`, `max-aspect-ratio`
  * `device-pixel-ratio`, `resolution`, `orientation`
1. The property requested is one of the following:  ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/css/CSSComputedStyleDeclaration.cpp&sq=package:chromium&l=457&dr=C&q=isLayoutDependent))
  * `height`, `width`
  * `top`, `right`, `bottom`, `left`
  * `margin` [`-top`, `-right`, `-bottom`, `-left`, or *shorthand*] only if the margin is fixed.
  * `padding` [`-top`, `-right`, `-bottom`, `-left`, or *shorthand*] only if the padding is fixed.
  * `transform`, `transform-origin`, `perspective-origin`
  * `translate`, `rotate`, `scale`
  * `webkit-filter`, `backdrop-filter`
  * `motion-path`, `motion-offset`, `motion-rotation`
  * `x`, `y`, `rx`, `ry`

### window
- `window.scrollX`, `window.scrollY`
- `window.innerHeight`, `window.innerWidth`
- `window.getMatchedCSSRules()` only forces style

### Forms
- `inputElem.focus()`
- `inputElem.select()`, `textareaElem.select()` ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/html/HTMLTextFormControlElement.cpp&q=updateLayoutIgnorePendingStylesheets%20-f:out%20-f:test&sq=package:chromium&l=192&dr=C))

### Mouse events
- `mouseEvt.layerX`, `mouseEvt.layerY`, `mouseEvt.offsetX`, `mouseEvt.offsetY` ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/events/MouseRelatedEvent.cpp&q=f:mouserelatedevent%20computeRelativePosition&sq=package:chromium&type=cs&l=132))

### document
- `doc.scrollingElement` only forces style

### Range
- `range.getClientRects()`, `range.getBoundingClientRect()`

### SVG
- Quite a lot; haven't made an exhaustive list , but [Tony Gentilcore's 2011
  Layout Triggering
  List](http://gent.ilcore.com/2011/03/how-not-to-trigger-layout-in-webkit.html)
  pointed to a few.


### contenteditable
- Lots & lots of stuff, …including copying an image to clipboard
  ([source](https://code.google.com/p/chromium/codesearch#chromium/src/third_party/WebKit/Source/core/editing/Editor.cpp&sq=package:chromium&l=420&dr=C&rcl=1442532378))

## See Also
- https://developers.google.com/web/fundamentals/performance/rendering/avoid-large-complex-layouts-and-layout-thrashing
- https://developers.google.com/web/fundamentals/performance/rendering/stick-to-compositor-only-properties-and-manage-layer-count
- https://gist.github.com/paulirish/5d52fb081b3570c81e3a
- https://csstriggers.com/
