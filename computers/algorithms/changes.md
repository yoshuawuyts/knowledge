# changes
_Also known as the "Shortest Edit Script" or "Diffing Algorithm"_.

The changes algorithm computes the difference between two states.

## Myers Diff
The Myers Diff Algorithm proposes to model the diff as a graph search problem.
It uses breadth-first search.

Say we have two vectors:
- `['a', 'b', 'c', 'a']`
- `['c', 'b', 'a', 'b']`

We could represent this in a graph as:

```txt
    'a' 'b' 'c' 'a'
   o---o---o---o---o 0
'c'|   |   | \ |   |
   o---o---o---o---o 1
'b'|   | \ |   |   |
   o---o---o---o---o 2
'a'| \ |   |   | \ |
   o---o---o---o---o 3
'b'|   | \ |   |   |
   o---o---o---o---o 4
   0   1   2   3   4
```

_The diagonal lines means in a graph traversal passing through this position
would be a no-op. We try and have as many no-ops as possible._

There's a few rules:
- we start at position `0,0`.
- we try go get to `4,4` in the fewest amount of steps possible.
- when in doubt we prefer going right.
- when there's multiple paths possible, we choose the shortest one.

The breadth-first edit graph would be:
```txt
0,0 - 1,0
 |     |
0,1   2,2 - 4,3
 |     |     |
2,4   2,3   4,4
 |
3,4
```

## References
- https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/
- https://blog.jcoglan.com/2017/02/15/the-myers-diff-algorithm-part-2/
