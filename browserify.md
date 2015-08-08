# browserify
Browserify has undergone a lot of iteration, only becoming stable in recent
years. In terms of design there's very little it gets wrong.  Probably the only
critique is that its documentation doesn't lend itself well to beginners.

This guide is meant to give you a pragmatic introduction to `Browserify`. If
you're looking for a more complete overview, check out
[browserify-handbook](https://github.com/substack/browserify-handbook) by
James Halliday (Substack).

- [what is browserify?](#what-is-browserify)
- [how to use browserify](#how-to-use-browserify)
- [transforms](#transforms)
- [what is the difference with other bundlers?](#what-is-the-difference-with-other-bundlers)

## What is Browserify?
`Browserify` lets you run CommonJS (Node) code in the browser. This reduces the
drawback of switching contexts between browser and server, and allows sharing
of modules between the two. Generally only native addons (`C` / `C++` code) and
`fs` won't work (though [you can cheat](https://github.com/substack/brfs)).

Browserify follows the unix philosophy, and only does 1 thing: make your
JavaScript code run in the browser just like it would in Node. By using tiny
modules called "transforms", `Browserify` can be extended to do all sorts of
stuff such as: static asset inlining, minification, dependency replacement,
bundle splitting and more!

Browserify can be called from both the command line and Node.

__shell__
```sh
# bundle index.js into bundle.js
$ browserify index.js -o bundle.js
```

__js__
```js
const browserify = require('browserify')
const fs = require('fs')

// bundle index.js into bundle.js
browserify('index.js')
  .bundle()
  .pipe(fs.createWriteStream('bundle.js')
```

About 90% of the time you'll be using the CLI, so don't worry too much about
learning the Node API.

## How to use browserify
As noted above, you generally don't want to use the Node Browserify API unless
you're building higher level tools on top of it. Don't let yourself be fooled
by all the folks running Browserify from within Grunt of Gulp: they're
overcomplicating things.

Instead of using an opinionated task runner, it's common to use `npm scripts`
to automate tasks:

__browserify__
```json
{
  "start": "npm run watch",
  "build": "browserify browser.js -o bundle.js",
  "watch": "watchify browser.js -o bundle.js --debug --verbose",
}
```
This setup is nice to get started with Browserify.

- __npm start__: run `npm run watch`
- __npm run build__: create JS bundle
- __npm run watch__: run `watchify` in debug + verbose mode

__browserify + cssnext__
```json
{
  "start": "npm run watch",
  "watch": "(npm run watch:ify & npm run watch:css & serve . -SJ)",
  "watch:ify": "watchify index.js -o bundle.js --debug --verbose",
  "watch:css": "cssnext --watch index.css bundle.css",
}
```
Browserify has well defined boundaries, and as such isn't concerned with CSS.
Here is an example of how to setup, serve and watch JS, HTML and CSS files
using `Browserify`, `cssnext` and `serve`.
- __npm start__: run `npm run watch`
- __npm run watch__: run `npm run watch:ify`, `npm run watch:css` and serve
  files
- __npm run watch:ify__: run `watchify` in debug + verbose mode
- __npm run watch:css__: run `cssnext` in watch mode

## Transforms
We've been mentioned before that transforms are Browserify's way of extending
Browserify's capabilities. There are 3 ways of using transforms: from the CLI,
`package.json` and JS API. The most convenient way is by including them in
`package.json`:

```json
{
  "name": "mypkg",
  "version": "1.2.3",
  "main": "main.js",
  "browserify": {
    "transform": [ "brfs" ]
  }
}
```
Now [`brfs`](https://github.com/substack/brfs) will be called on all code
that's part of `mypkg`. By including it in `package.json`, the transform
will be run on `mypkg` when `mypkg` is required by a parent package that is
being bundled with Browserify.

Some useful transforms are:
- [brfs](https://github.com/substack/brfs) - Browserify fs.readFileSync()
  static asset inliner
- [babelify](https://www.npmjs.com/package/babelify) - turn ES201{5,6,7} code
  into ES5
- [envify](https://github.com/hughsk/envify) - Selectively replace Node-style
  environment variables with plain strings
- [uglifyify](https://github.com/hughsk/uglifyify) - A Browserify transform
  which minifies your code using UglifyJS2

Browserify plugins have full access to the bundle and can do more advanced things:
- [errorify](https://github.com/zertosh/errorify) - write
  failed build error messages to the output file
- [factor-bundle](https://github.com/substack/factor-bundle) - factor
  browser-pack bundles into common shared bundles
- [proxyquireify](https://github.com/thlorenz/proxyquireify) - mock
  modules for testing by overriding `require`

There are also a number of useful modules that integrate via a CLI or Unix piping:
- [disc](https://github.com/hughsk/disc) - Visualise the module tree of
  Browserify project bundles and track down bloat
- [exorcist](https://github.com/thlorenz/exorcist) - Externalizes the source
  map found inside a stream to an external .js.map file

## What's the difference with other bundlers?
Browserify is different from other bundlers because of its simplicity. Where
other bundlers add more features over time, `Browserify` focuses on stability
and interoperability with the Node API, leaving the experimentation to
transforms.

Using Browserify comes with a set of guarantees:

- __transparancy__: Browserify is mostly considered done. Any changes made
  strictly follow semver, and comprehensive release notes are released with
  every patch.
- __portability__: `fs` and native addons aside, Browserify gives full
  interoperability between the browser and Node. For example: Browserify won't overload `require()` and change its semantics just
  to inline assets. Instead it uses [a
  transform](https://github.com/substack/brfs) that will work with any module
  loader.
- __ecosystem__: a vibrant ecosystem of transforms created that add just about
  any feature imaginable. There are transforms available for minification,
  asset inlining, bundle splitting, compiling templates and more.
- __extensibility__: higher level tools can consume Browserify as a dependency.
  For example: `budo` is a command line application server that bundles JS, CSS
  and HTML into an application, allowing devs to spend their time working on a
  demo, rather than the build infrastucture.
- __readability__: Browserify's source is small, thoroughly tested and relies
  on external modules to break down complexity. If you're curious how a feature
  works it's easy to dive in and poke around.
- __shareability__: Browserify works great for teams. Contrary to other loaders
  it doesn't require lengthy configuration files to run tasks. When a new
  member joins a team, all they need to do is glance over the list of
  transforms to determine what the bundler is doing, greatly speeding up the
  onboarding process.

Though not every other bundler fails on all points, only Browserify ticks all
boxes.

## See Also
- [browserify-handbook](https://github.com/substack/Browserify-handbook)
- [browserify](https://github.com/substack/node-Browserify)
