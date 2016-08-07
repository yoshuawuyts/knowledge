# algorithms
Algorithms === data structures.

# Linear algebra
- [LAPACK package](http://en.wikipedia.org/wiki/LAPACK)

## Markov chains

##### Markov-montecarlo
- [source](http://en.wikipedia.org/wiki/Markov_chain_Monte_Carlo)

## Metropolis-Hastings
- [my favorite algorithm: Metropolis-hastings](http://flynnmichael.com/2015/06/01/my-favorite-algorithm-metropolis-hastings/)

## Groth-sahai
non interactive zero knowledge proof.
- [wikipedia](http://en.wikipedia.org/wiki/Non-interactive_zero-knowledge_proof)

## Radix tree
Aka radix trie. Like a regular trie, but multiple chars are allowed per node.
Even better when coupled with immutability.

- [wikipedia](http://en.wikipedia.org/wiki/Radix_tree)
- [go implementation](https://github.com/hashicorp/go-immutable-radix/blob/master/iradix.go)

## Machine learning
- [linear regression](http://en.wikipedia.org/wiki/Linear_regression)
- [multiple linear regression](http://en.wikipedia.org/wiki/Linear_regression#Simple_and_multiple_regression)
- [linear least squares (the normal equation)](http://en.wikipedia.org/wiki/Linear_least_squares_(mathematics))
- [gradient descent algorithm](http://en.wikipedia.org/wiki/Gradient_descent)
- [model based machine learning](http://www.mbmlbook.com/toc.html)
- [neural network implementation](http://peterroelants.github.io/posts/neural_network_implementation_part01/)
- [inceptionism: going deeper into Neural Networks](http://googleresearch.blogspot.nl/2015/06/inceptionism-going-deeper-into-neural.html)

## Bloom filter
Check if something is possibly in a set, or definitely not.
- [what is the advantage of using bloom filter](http://stackoverflow.com/questions/4282375/what-is-the-advantage-to-using-bloom-filters)

## Block chains
Merkle-DAG.

## Merge trees
- [log structured merge trees](http://www.benstopford.com/2015/02/14/log-structured-merge-trees/)

## K-d tree
Used to organize points on a multi-dimensional tree. Often used for higher
domensions. Works pretty well for 2d in some cases hah. Nearest neighbour
lookups for lat-long cords should be done in 2d. This is because the earth is a
sphere, lol. It's using a data-structure for 3d points, but running it in 2d.

Split a plane up into zones. Find which zone the point is located in.
- [wikipedia](https://en.wikipedia.org/wiki/K-d_tree)

## Streaming algorithms
Streaming algorithms are useful to operate on values that are too big to keep
in memory at any given time. Unlike windowing and sampling techniques,
streaming algorithms operate on the whole dataset in realtime.
- [streaming algorithms](https://www.youtube.com/watch?v=e9httij2RG0)

## See Also
- [bost.ocks.org/algorithms](http://bost.ocks.org/mike/algorithms/)
- [a tour of algorithms](http://machinelearningmastery.com/a-tour-of-machine-learning-algorithms/)
- [complete guide to blockchain technology](http://blockstrap.com/en/a-complete-beginners-guide-to-blockchain-technology/)
