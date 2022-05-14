# elematch-rs

A solution to an interesting programming problem proposed by a friend of mine:

> Given a set of **N** elements `[1, 2, 3, 4, 5, ..., N]`, divide the set into **M** 
> groups such that each group is the same size (eg. pairs, triplets).
> You are also given a set of rules for which elements can be in the same group.
> If a set of rules makes grouping impossible, an error should be returned.


### Example

```
Elements: [1, 2, 3, 4, 5, 6], 3 groups of 2

Rules:
[1, 6]
[2, 4]
[4, 5]

Validity matrix:
  1 2 3 4 5 6
1 x         x
2   x   x
3     x
4   x   x x
5       x x
6 x         x

Possible Groupings:
[1, 2], [3, 4], [5, 6]
[1, 2], [3, 5], [4, 6]
[1, 2], [3, 6], [4, 5] <- invalid [4, 5]
[1, 3], [2, 4], [5, 6] <- invalid [2, 4]
[1, 3], [2, 5], [4, 6]
[1, 3], [2, 6], [4, 5] <- invalid [4, 5]
[1, 4], [2, 3], [5, 6]
[1, 4], [2, 5], [3, 6]
[1, 4], [2, 6], [3, 5]
[1, 5], [2, 3], [4, 6]
[1, 5], [2, 4], [3, 6] <- invalid [2, 4]
[1, 5], [2, 6], [3, 5]
[1, 6], [2, 3], [4, 5] <- invalid [1, 6], [4, 5]
[1, 6], [2, 4], [3, 5] <- invalid [1, 6], [2, 4]
[1, 6], [2, 5], [3, 4] <- invalid [1, 6]
```