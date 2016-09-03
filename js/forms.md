# forms

## Naming things
There's a hierarchy in elements within an application. Forms should be composed
of:
- `<input>` - elements in a form a user can interact with (buttons included)
- `<label>` - descriptors for the fields
- `<fieldset>` - groupings of `<input>` and `<label>` into sections
- `<form>` - the complete form

```html
<form>
  <fielset>
    <label>username</label>
    <input type="text" name="username" placeholder="username">
  </fielset>
  <fielset>
    <label>password</label>
    <input type="password" name="password" placeholder="password">
  </fielset>
</form>
```

For validation it's useful if every single field in the form is unique. Use
prefixes if you must; but doing validation on the form level is probably what
you want to do, or else risk separating at the wrong level which is bad bad not
good.

## prevent form data loss on window close
```js
// Get the text field that we're going to track
var field = document.getElementById("field");

// See if we have an autosave value
// (this will only happen if the page is accidentally refreshed)
if (sessionStorage.getItem("autosave")) {
  // Restore the contents of the text field
  field.value = sessionStorage.getItem("autosave");
}

// Listen for changes in the text field
field.addEventListener("change", function() {
  // And save the results into the session storage object
  sessionStorage.setItem("autosave", field.value);
});
```
- [mdn/sessionStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
