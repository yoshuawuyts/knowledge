# npm

## show latest tag for package
```sh
$ npm dist-tag ls <package>
```

## create release candidate
The `next` tag is commonly used to specify an rc. The `npm version
[premajor|preminor|prepatch]` commands can be used for this:

```sh
# publish a major RC
$ npm version premajor
$ npm publish --tag=next
$ npm install <package>@latest
```
- https://docs.npmjs.com/cli/dist-tag
- https://docs.npmjs.com/cli/publish
- https://docs.npmjs.com/cli/version
