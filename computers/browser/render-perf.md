# render perf
Mostly how to make CSS fast.

## Contain
> The contain property allows an author to indicate that an element and its
> contents are, as much as possible, independent of the rest of the document
> tree. This allows the browser to recalculate layout, style, paint, size, or
> any combination of them for a limited area of the DOM and not the entire
> page.

```css
/* No layout containment. */
contain: none;

/* Turn on containment for layout, style, paint, and size. */
contain: strict;

/* Turn on containment for layout, style, and paint. */
contain: content;

/* Turn on size containment for an element. */
contain: size;

/* Turn on layout containment for an element. */
contain: layout;

/* Turn on style containment for an element. */
contain: style;

/* Turn on paint containment for an element. */
contain: paint;
```

## Will-change
> The will-change CSS property provides a way for authors to hint browsers
> about the kind of changes to be expected on an element, so that the browser
> can set up appropriate optimizations ahead of time before the element is
> actually changed. These kind of optimizations can increase the responsiveness
> of a page by doing potentially expensive work ahead of time before they are
> actually required

```css
/* Keyword values */
will-change: auto;
will-change: scroll-position;
will-change: contents;
will-change: transform;        /* Example of <custom-ident> */
will-change: opacity;          /* Example of <custom-ident> */
will-change: left, top;        /* Example of two <animateable-feature> */

/* Global values */
will-change: inherit;
will-change: initial;
will-change: unset;
```

## See Also
- https://developer.mozilla.org/en-US/docs/Web/CSS/contain
- https://medium.com/outsystems-experts/flip-your-60-fps-animations-flip-em-good-372281598865
- https://medium.com/outsystems-experts/how-to-achieve-60-fps-animations-with-css3-db7b98610108
- https://developer.mozilla.org/en/docs/Web/CSS/will-change
