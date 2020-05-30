# Errors

## Creating a new error

```rust
use std::error::Error;
use std::fmt::{self, Display};

/// An error returned when failing to convert into a status code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConversionError {
    _private: (),
}

impl Error for ConversionError {}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Error".fmt(f)
    }
}
```

## Failure Boilerplate

```rust
use failure::{self, Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::result;

/// A specialized [`Result`] type for this crate's operations.
///
/// This is generally used to avoid writing out [Error] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
/// [`Error`]: std.struct.Error.html
pub type Result<T> = result::Result<T, failure::Error>;

/// A list enumerating the categories of errors in this crate.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] struct.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug, Fail)]
pub enum ErrorKind {
  /// Any error not part of this list.
  #[fail(display = "Generic error.")]
  Other,
}

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
#[derive(Debug)]
pub struct Error {
  inner: Context<ErrorKind>,
}

impl Error {
  /// Access the [`ErrorKind`] member.
  ///
  /// [`ErrorKind`]: enum.ErrorKind.html
  pub fn kind(&self) -> &ErrorKind {
    &*self.inner.get_context()
  }
}

impl Fail for Error {
  fn cause(&self) -> Option<&dyn Fail> {
    self.inner.cause()
  }

  fn backtrace(&self) -> Option<&Backtrace> {
    self.inner.backtrace()
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.inner, f)
  }
}

impl From<ErrorKind> for Error {
  fn from(kind: ErrorKind) -> Error {
    let inner = Context::new(kind);
    Error { inner }
  }
}

impl From<Context<ErrorKind>> for Error {
  fn from(inner: Context<ErrorKind>) -> Error {
    Error { inner }
  }
}
```

__Usage__
```rust
perform_some_io().context(ErrorKind::NetworkFailure)?;
Err(ErrorKind::DomainSpecificError)?
```
