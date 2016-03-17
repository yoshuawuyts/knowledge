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
- z-indices, colors, fonts are always a variable
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
flex-shrink       <ufloat:1.0>
flex-basis        <length> | auto
flex              none | <flex-grow> <flex-shrink> || <flex-basis>
align-self        auto, flex-start, flex-end, center, stretch, baseline
```

## Files
Storing CSS in the right files is just as important as correctly naming
classes. There are a few base files every site wants to start out with:
```text
index.css       imports
vars.css        variable declarations
base.css        body classes & helpers, usually super tiny
typography.css  base text styles
buttons.css     base button styles
forms.css       base form styles
lists.css       base list styles
```

## Attribute selectors
```css
.show-grid [class*="span"] {} /* search children for class="span"*/
div[class^="something"] {}    /* search children for "begins with..." */
div[class$="something"] {}    /* search children for "ends with..." */
```

- [30 selectors you must memorize](http://code.tutsplus.com/tutorials/the-30-css-selectors-you-must-memorize--net-16048)
- [CSS3 selector substring matching](http://www.impressivewebs.com/css3-attribute-selectors-substring-matching/)

## Print stylesheets
There are 2 approaches: blacklisting and whitelisting. Whitelisting is unwieldy
to implement using CSS level 4, so blacklisting is the way to go.

### whitelisting
Pick the elements you want to hide (sidebar, footer, etc) and set them to
`display: none`.

- [print stylesheet approaches](https://css-tricks.com/print-stylesheet-approaches-blacklist-vs-whitelist)

## Style input placholders
Supported by autoprefixer.
```css
input::placeholder { }
```
[source](https://github.com/postcss/autoprefixer/issues/44)

## Shadow
```txt
box-shadow: [horizontal offset] [vertical offset]
            [blur radius] [optional spread radius] [color];
```

## natural looking components
- flexy, resizable things
- grab the border between 1 and 2, change the `flex-grow` property.
- components need a certain order (`flex -> relative -> absolute`) or else the
  full width won't be covered.
```
---- ----
| 1| |2 |
---- ----
```

## Web fonts
```css
@font-face {
  font-family: 'Maison Neue Mono';
  src: url('assets/maison-mono.woff') format('woff');
}
```
- https://css-tricks.com/snippets/css/using-font-face/

## Media Queries
```css
@media (min-width: 700px) {
}

@media all and (min-width: 700px) {
}
```

## Set selection color
```css
*::selection {
  background: hotpink;
}
```

## See also
- [medium's CSS coding guidelines](https://gist.github.com/fat/a47b882eb5f84293c4ed)
- [medium's CSS post thingy](https://medium.com/@fat/mediums-css-is-actually-pretty-fucking-good-b8e2a6c78b06)
