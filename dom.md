# dom
While most develoeprs have some understanding of how HTML and the DOM work,
there are a good amount of tags available that are most likely being misused. I
don't care about non-evergreen browsers, so that's the only guarantee about
compatibility I'm able to make.

## Tags
```txt
<nav>       the main navigation controls
<header>    the introductory block at the start of the page
<main>      the main content, should cover most of the page
<section>   the different parts of main
<footer>    page end, credits and stuff
<aside>     the sidebar, if any exists
```

## Events
A big gotcha of `dom` events it that they propegate to their parent nodes if left
unhandled. By setting `this.stopPropegation()` the events are no longer
propegated to their parents.

Another gotcha with the `dom`, is that when you're building dynamic systems
with JavaScript, existing dom nodes will have default behavior that is
executed. For example: when submitting a form, the default behavior is to
refresh the page after submission. To prevent this from happening you have to
use the `this.preventDefault()` method within the form element.
