# forms

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
