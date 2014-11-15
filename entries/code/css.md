# CSS

I've written a bunch of resources over the years, it'll probably be a while
before I pull them all in here. Warning: dutch language ahead (must translate
still).

## See also
- [medium's CSS coding guidelines](https://gist.github.com/fat/a47b882eb5f84293c4ed)
- [medium's CSS post thingy](https://medium.com/@fat/mediums-css-is-actually-pretty-fucking-good-b8e2a6c78b06)

### consistent naming
`.active` -> `.navBar-element_active`

`.nav` / `.navBar` -> kies er een, maar wees consistent

### block-element-modifier
```
.navbar                  block
.navbar-listItem         block-element
.navbar-listItem_active  block-element_modifier
```

### single selector only (or how to avoid specificity issues)
`.foo .foo-bar .foo-bar_baz` -> `.foo-bar_baz`. Door BEM te gebruiken zijn `.foo` en `.foo-bar` overbodig omdat `.foo-bar_baz` al uniek is. Hiermee voorkom je specificity issues.

### comments
Use 'em.

per element groep een comment om aan te duiden dat de volgende begint. Als je te veel verschillende elementen in een file krijgt is het tijd om ze te splitsen.
