# Modules

## Resources
- [mattdesl/modules](https://gist.github.com/mattdesl/73e3b9f902f9c834b721)
- [ferros/mad-science-modules](https://github.com/feross/mad-science-modules)
- [mafintosh](https://github.com/mafintosh?tab=repositories) - probably contains a solution for your problems

## Graphics
- [stack.gl](http://stack.gl/packages/)

## Audio
- [gl-audio-analyzer](https://github.com/stackgl/gl-audio-analyser) - Pull audio waveform/frequency data into WebGL for realtime audio visualisation
- [soundcloud-badge](https://github.com/hughsk/soundcloud-badge) - A SoundCloud 'now-playing' badge you can just drop into browserify demos

## Tools
- [browserify](http://ghub.io/browserify)
- [errorify](https://github.com/zertosh/errorify) - Browserify plugin to write failed build error messages to the output file
- [watchify](https://github.com/substack/watchify) - watch mode for browserify builds
- [standard](http://ghub.io/standard) - JavaScript standard style linter
- [standard-format](http://ghub.io/standard-format) - JavaScript standard style formatter
- [recess](http://ghub.io/recess) - CSS linter
- [wzrd](http://github.com/maxogden/wzrd) - Super minimal browserify development server
- [changelog-maker](https://github.com/rvagg/changelog-maker) - A git log to CHANGELOG.md tool
- [vtop](https://github.com/MrRio/vtop) - visual `top`
- [pkg-name](https://github.com/sindresorhus/pkg-name) - Check whether a package name is available on npm
- [budo]()
- [garnish]()
- [opnr]()
- [wtch](https://github.com/mattdesl/wtch) - small livereload utility for rapid prototyping
- [rm-modules]()
- [wsnc](https://github.com/substack/wsnc) - websocket netcat
- [taco](https://github.com/maxogden/taco) - a modular deployment system for unix
- [webworkify](https://github.com/substack/webworkify) - launch a web worker that can require() in the browser with browserify

## Client-side MVC
- [wayfarer](http://ghub.io/wayfarer)
- [simple-store](http://github.com/yoshuawuyts/simple-store)
- [barracks](https://github.com/yoshuawuyts/barracks)
- [whatwg-fetch](https://github.com/github/fetch) - A window.fetch JavaScript polyfill
- [webcomponents.js](https://github.com/webcomponents/webcomponentsjs) - A suite of polyfills supporting the HTML Web Components specs

## CSS
- [rework](https://github.com/reworkcss/rework) - Plugin framework for CSS preprocessing in Node.js
- [rework-npm](https://github.com/reworkcss/rework-npm) - Import CSS from npm modules using rework
- [myth](https://github.com/segmentio/myth) - A CSS preprocessor that acts like a polyfill for future versions of the spec
- [css-wipe](https://github.com/yoshuawuyts/css-wipe) - Reset the browser's styles
- [defaultcss](https://github.com/DamonOehlman/defaultcss) - Add a style definition in your HTML

## Server MVC
- [koa](https://github.com/koajs/koa)
- [http](https://github.com/Raynos/http-framework/) - A web framework based purely on require('http')
- [brick-router](https://github.com/yoshuawuyts/brick-router) - Modular router for serving static assets

## DOM
- [virtual-dom](https://github.com/Matt-Esch/virtual-dom) - A Virtual DOM and diffing algorithm
- [vdom-to-html](https://github.com/nthtran/vdom-to-html) - Turn virtual-dom nodes into HTML
- [virtual-html](https://github.com/azer/virtual-html) - Convert given HTML into Virtual DOM object
- [virtual-hyperscript-svg](https://github.com/substack/virtual-hyperscript-svg) - create virtual-dom nodes for svg using hyperscript syntax
- [value-event](https://github.com/Raynos/value-event) - Create DOM event handlers that write to listeners
- [domify](https://github.com/component/domify) - Turn HTML into DOM elements x-browser
- [custom-element](https://github.com/requireio/custom-element) - Convenience wrapper for creating custom element prototypes
- [synthetic-dom-events](https://www.npmjs.com/package/synthetic-dom-events) - create DOM compliant custom events
- [main-loop](https://github.com/Raynos/main-loop) - A rendering loop for diffable UIs

## Data manipulation
- [pbs](https://github.com/mafintosh/pbs) - Streaming protocol buffers encoder/decoder
- [format-data](https://github.com/finnp/format-data) - Format tabular data from an object stream to different standard formats
- [CSV-write-stream](https://github.com/maxogden/csv-write-stream) - A CSV encoder stream that produces properly escaped CSVs
- [CSV-parser](https://github.com/mafintosh/csv-parser) - Streaming csv parser inspired by binary-csv that aims to be faster than everyone else
- [ndjson](https://github.com/maxogden/ndjson) - streaming line delimited json parser + serializer
- [ssejson](https://github.com/finnp/ssejson) - Parse and Serialize object streams over SSE/EventSource
- [trumpet](https://github.com/substack/node-trumpet) - parse and transform streaming html using css selectors
- [hyperstream](https://github.com/substack/hyperstream) - stream html into html at a css selector

## Testing
- [tape](https://github.com/substack/tape) - tap-producing test harness for node and browsers
- [Smokestack](https://github.com/hughsk/smokestack) - Pipe your JavaScript into a browser, logging console output in Node
- [tap-closer](https://github.com/hughsk/tap-closer) - Close process on exit

## P2P
- [signalhub](https://github.com/mafintosh/signalhub) - Simple signalling server that can be used to coordinate handshaking with webrtc or other fun stuff
- [simple-peer](https://github.com/feross/simple-peer) - Simple WebRTC video/voice and data channels
- [bittorent-dht](https://github.com/feross/bittorrent-dht) - Simple, robust, BitTorrent DHT implementation
- [webrtc-swarm](https://github.com/mafintosh/webrtc-swarm) - Create a swarm of p2p connections using webrtc and a signalhub
- [peer-wire-swarm](https://github.com/mafintosh/peer-wire-swarm) - swarm implementation for bittorrent

## Neato functions
- [curry](https://github.com/dominictarr/curry)
- [map-limit](https://github.com/hughsk/map-limit) - async.mapLimit's functionality available as a standalone npm module

## Streams
- [bl](https://github.com/rvagg/bl) - Buffer List: collect buffers and access with a standard readable Buffer interface, streamable too!
- [duplexify](https://github.com/Raynos/duplexer) - Turn a writeable and readable stream into a streams2 duplex stream with support for async initialization and streams1/streams2 input
- [concat-stream](https://github.com/maxogden/concat-stream) - writable stream that concatenates strings or data and calls a callback with the result
- [from2]()
- [through2]()
- [pumpify]()

## CLI
- [minimist](https://github.com/substack/minimist) - parse argument options
- [cliclopts](https://github.com/finnp/cliclopts) - CLI options helper and usage printer
- [subcommand](https://github.com/maxogden/subcommand) - Create CLI tools with subcommands
- [subarg-replace](https://github.com/mattdesl/subarg-replace) - regex replace on subarg list
- [string-editor](https://github.com/mafintosh/string-editor) - Edit a string using $EDITOR from within your node app

## Database
- [content-addressable-blob-store](https://github.com/mafintosh/content-addressable-blob-store) - streamable content addressable blob object store
- [batchdb](https://github.com/substack/batchdb) - leveldb and disk storage for queued batch jobs

## External
- [electron-prebuilt](https://github.com/mafintosh/electron-prebuilt) - install electron prebuilts using npm
- [electron-packager](https://github.com/maxogden/electron-packager) - package and distribute your electron app in OS executables (.app) via JS or CLI
- [npm-execspawn](https://github.com/mafintosh/npm-execspawn) - Spawn locally installed npm dependencies with cross platform env and argument parsing support
- [link-bin](https://github.com/mafintosh/link-bin) - Make bin scripts work local npm dependencies
- [npm-start](https://github.com/mafintosh/npm-start) - 'npm start' written in bash that propagates kill to subprocesses
