# forms

## Validation
```js
if (input.value != primaryEmail) {
  // the provided value doesn't match the primary email address
  input.setCustomValidity('The two email addresses must match.');
  console.log("E-mail addresses do not match", primaryEmail, input.value)
} else {
  // input is valid -- reset the error message
  input.setCustomValidity('')
}
```

- submit the form or use `form.reportValidity()`
- if you submit the form, make sure it doesn't actually submit

Prevent forms from submitting if not all fields are valid
```js
form.addEventListener('submit', function(e) {
  e.preventDefault()
  if (!e.target.checkValidity()) return false
  emit('form:submit', new window.FormData(e.target))
})
```

## Native validation
```js
css`
  input:invalid {
    border-color: red;
  }
  input,
  input:valid {
    border-color: #ccc;
  }
`
html`
  <form action="somefile.php" method="POST">
    <input name="username" id="username"
      type="text"
      placeholder="Username"
      pattern="[a-z]{1,15}"
      oninvalid=${oninvalid}
      title="Username should only contain lowercase letters. e.g. john"
    >
  </form>
`

function oninvalid (event) {
  event.target.setCustomValidity(event.target.title)
}
```

## Only allow certain filetypes
This restricts selection to only certain filetypes too.
```html
<input type="file" name="pic" id="pic" accept="image/gif, image/jpeg" />
```
- https://stackoverflow.com/questions/181214/file-input-accept-attribute-is-it-useful
- https://stackoverflow.com/questions/7575482/restrict-file-upload-selection-to-specific-types

## Buttons
Buttons are treated as `type="submit"` implicitely.
```html
<button type="button">Text here</button>
```

## See Also
- https://developers.google.com/web/fundamentals/design-and-ui/input/forms/
- https://dev.w3.org/html5/spec-preview/constraints.html#constraint-validation
- https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity
- https://webdesign.tutsplus.com/tutorials/html5-form-validation-with-the-pattern-attribute--cms-25145
