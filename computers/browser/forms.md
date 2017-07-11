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
form.addEventListener('submit', function(evt) {
  if (form.checkValidity() === false) {
    evt.preventDefault()
    return false
  }
});
```

## See Also
- https://developers.google.com/web/fundamentals/design-and-ui/input/forms/
- https://dev.w3.org/html5/spec-preview/constraints.html#constraint-validation
- https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity
