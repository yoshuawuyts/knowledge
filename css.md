# CSS
New school CSS best practices. No SASS, just CSS. Best used with
[css-next](https://cssnext.github.io/) or comparable.

## Naming
Naming things is hard, and with specificity in the mix it becomes even harder.
By using the naming scheme below, many, if not all naming issues will be
resolved. Short words and dashes between them are key.
```
.navbar              block
.navbar-item         block-element
.navbar-item-active  block-element_modifier
```

## Specificity
```css
.foo .foo-bar .foo-bar_baz {}
.foo-bar_baz {}
[foo="bar"] {}
```
By using the second or third form, specificity issues will never happen.

## Comments
Comments are useful to indicate new sections. Often times groups of selectors
belong together, and comments are a great way to indicate that.

## Variables
- have a main color
- light colors
- dark colors
- inverted colors
- err, warn, success, action
- have a complimentary color
- head, body, mono fonts
- z-indeces, colors, fonts are always a variable
- variables for everything
- all variables in one file

## Flexbox
Flex container (parent):
```txt
display           flex, inline-flex
flex-direction    row, row-reverse, column
flex-wrap         nowrap, wrap, wrap-reverse
flex-flow         <flex-direction> <flex-wrap>
justify-content   flex-start, flex-end, center, space-between, space-around
align-items       flex-start, flex-end, center, stretch, baseline
align-content     flex-start, flex-end, center, stretch, space-between, space-around
```

Flex items (children):
```txt
order             <integer>
flex-grow         <integer:0>
flex-schrink      <ufloat:1.0>
flex-basis        <length> | auto
flex              none | <flex-grow> <flex-shrink> || <flex-basis>
align-self        auto, flex-start, flex-end, center, stretch, baseline
```

## See also
- [medium's CSS coding guidelines](https://gist.github.com/fat/a47b882eb5f84293c4ed)
- [medium's CSS post thingy](https://medium.com/@fat/mediums-css-is-actually-pretty-fucking-good-b8e2a6c78b06)
