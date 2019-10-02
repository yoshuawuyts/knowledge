# iterators

Iterators are Rust's way of looping over values inside a struct.

Say have a bookshelf that has books, and we want to get each book from the
bookshelf. The nicest way in Rust to call it would be to iterate over each book
in the bookshelf and print it to the console.

```rust
for book in library {
  println!("Book: {}", book.title);
}
```

Any struct can be iterated over, as long as it implements the `Iterator` trait.

## Implementing an Iterator
For most ergonomic iterators, there'll be 4 moving parts involved:

- An initial struct containing the values we want to iterate over.
- A second struct that can keeps track which index the iterator is currently at.
- An implementation of the [`Iterator trait`] onto the second struct.
- An implementation of the [`IntoIterator trait`] onto the first struct.

Sticking with our initial example: let's implement a library that can be
iterated over to get each book. We need 3 structs (`Book`, `Library`,
`LibraryIterator`), and 3 impl blocks (`Library`, `Iterator for
LibraryIterator`, `IntoIterator for Library`).

Because a library is a public place you borrow books from, we cannot give away
ownership of our books. So instead of giving away `Book`, our iterator gives
`&Book`. This would be similar when implementing a hash map, cache or other
collection.

This is harder than implementing a version where we give away ownership of each
item in the collection, but it should provide a good understanding of what's
going on.

### Implementation
```rust
use std::iter;

#[derive(Debug)]
struct Book {
  pub title: String,
}

#[derive(Debug)]
struct Library {
  pub books: Vec<Book>,
}

#[derive(Debug)]
struct LibraryIterator<'b> {
  /// Keeps track which index we're currently at.
  pub cursor: usize,
  /// Borrow of the Bookshelf we're going to iterate over.
  pub inner: &'b Library,
}

impl Library {
  /// Create an iterator that iterates over all books in the library.
  pub fn iter(&self) -> LibraryIterator {
    LibraryIterator {
      inner: self,
      cursor: 0,
    }
  }
}

impl<'b> iter::Iterator for LibraryIterator<'b> {
  type Item = &'b Book;

  fn next(&mut self) -> Option<Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;

    if cursor >= self.inner.books.len() {
      None
    } else {
      Some(&self.inner.books[cursor])
    }
  }
}

impl<'b> iter::IntoIterator for &'b Library {
  type Item = &'b Book;
  type IntoIter = LibraryIterator<'b>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      cursor: 0,
      inner: self,
    }
  }
}
```

Phew, that's quite a bit of code. Let's now create a new instance of our
library, and fill it with books. We can then iterate over each book in our
library.

```rust
fn main() {
  let library = Library {
    books: vec![
      Book { title: "Das Kapital I".into() },
      Book { title: "Das Kapital II".into() },
      Book { title: "Das Kapital III".into() },
    ],
  };

  for book in &library {
    println!("book {}", book.title);
  }
}
```

And that's all it takes to implement the `Iterator` trait for collections.  I
hope this provides a solid basis to go out and implement your own iterators.
Thanks for reading!

[`IntoIterator trait`]: https://doc.rust-lang.org/1.26.2/std/iter/trait.IntoIterator.html
[`Iterator trait`]: https://doc.rust-lang.org/1.26.2/std/iter/trait.Iterator.html

## Iters
```rust
use std::iter::{IntoIterator, Iterator};

/// A collection of HTTP Headers.
#[derive(Debug)]
pub struct Headers<'a> {
    headers: &'a mut http::HeaderMap,
}

impl<'a> Headers<'a> {
    /// Create a new instance.
    pub(crate) fn new(headers: &'a mut http::HeaderMap) -> Self {
        Self { headers }
    }

    /// Get a header.
    pub fn get(&self, key: &'static str) -> Option<&'_ str> {
        self.headers.get(key).map(|h| h.to_str().unwrap())
    }

    /// Set a header.
    pub fn insert(&mut self, key: &'static str, value: impl AsRef<str>) -> Option<String> {
        let value = value.as_ref().to_owned();
        let res = self.headers.insert(key, value.parse().unwrap());
        res.as_ref().map(|h| h.to_str().unwrap().to_owned())
    }

    /// Iterate over all headers.
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.headers.iter())
    }
}

impl<'a> IntoIterator for Headers<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.headers.iter())
    }
}

/// An iterator over headers in `Headers`.
#[derive(Debug)]
pub struct Iter<'a>(http::header::Iter<'a, http::header::HeaderValue>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, value)| (key.as_str(), value.to_str().unwrap()))
    }
}
```
