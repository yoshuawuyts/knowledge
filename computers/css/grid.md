# CSS Grid

## Fallback support
```css
@supports(display:grid) {
  /* grid code goes here */
}
```

## Holy Grail Layout
```html
<body class="grid-main">
  <header class="area-header">
    <button class="toggle-sidebar">=</button>
  </header>
  <aside class="area-aside transition"></aside>
  <main class="area-main"></main>
  <footer class="area-footer"></footer>
</body>
```

```css
:root {
  --area-header-height: 2rem;
  --area-aside-width: 10rem;
  --area-footer-height: 2rem;
}

.grid-main {
  display: grid;
  height: 100vh;
  overflow: hidden;
  grid-template-rows: var(--area-header-height) 1fr var(--area-footer-height);
  grid-template-columns: 1fr;
  grid-template-areas: 'header'
                       'main'
                       'main';
}

.area-header { grid-area: header; }
.area-aside {
  grid-area: aside;
  height: calc(100vh - var(--area-header-height));
  transform: translateX(-100%);
  position: absolute;
  top: var(--area-header-height);
  width: var(--area-aside-width);
}

.area-footer { grid-area: footer; }
.area-main {
  grid-area: main;
  overflow: auto;
}

@media screen and (min-width: 30em) {
  .grid-main {
    grid-template-columns: var(--area-aside-width) 1fr;
    grid-template-areas: 'header header'
                         'aside main'
                         'footer main';
  }
  
  .area-aside {
    position: static;
    top: 0;
    width: auto;
    transform: translateX(0);
  }
  
  .toggle-sidebar {
    display: none;
  }
}

.toggle-aside {
  transform: translateX(0);
}

.transition {
  transition: transform .3s ease-in-out;
}

header { background-color: tomato }
aside { background-color: papayawhip }
main { background-color: lightblue }
footer { background-color: lightgreen }
body { font-family: sans-serif; margin: 0; background-color: lightblue; }
```

```js
let aside = document.querySelector('.area-aside')
let toggleSidebar = document.querySelector('.toggle-sidebar')

toggleSidebar.addEventListener('click', (e) => {
  aside.classList.toggle('toggle-aside')
})
```

## Links
- https://codepen.io/anon/pen/JxXrqz?editors=1100
