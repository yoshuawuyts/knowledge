# design patterns

## Formal design
Formal design is the practice of designing systems so all logic can be analyzed
and tested against. It's in order to define programs before they're run.
Writing specs basically. The TLA+ language is commonly used for this. While
this approach is relatively slow, it can definitely solidify a formal spec;
generally should be used to verify if algorithms work the way they should in
production.

While TSA+ is based on discrete math; `PlusCal` reads more like a C-style
pramming language which can directly be translated to TLA+. This should help
programmers get around better, and works as a direct replacement for
pseudocode.
- [TLA+](https://en.wikipedia.org/wiki/TLA%2B) formal design specification
- [pluscal](https://en.wikipedia.org/wiki/PlusCal)
- [formal methods at amazon](http://research.microsoft.com/en-us/um/people/lamport/tla/formal-methods-amazon.pdf)

## Functional programming
-  http://www.haskellforall.com/2012/09/the-functor-design-pattern.html
- http://www.haskellforall.com/2012/08/the-category-design-pattern.html
- http://www.haskellforall.com/2012/06/you-could-have-invented-free-monads.html
- Studying the comonad, contravariant and free packages (well, blog posts)
