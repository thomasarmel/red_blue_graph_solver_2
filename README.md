# Red-blue graph problem solver - Rust implementation

The problem is the following:

In a directed graph, each node is colored either red or blue. Furthermore, vertices are also colored red or blue.

When a node is deleted, its adjacent nodes are colored the same color as the vertex which made the link with the node.

What is the maximum uninterrupted sequence we can delete, with k nodes of all the same color ? We cannot delete any node of the other color.


### Example

Consider having the following flat graph:

![Flat graph example](images/flat_graph_example.png)

The color to delete is RED, so we get the following output: [2, 3, 5, 4, 6]

If we analyze the output:

- We delete the node 2 -> node 1 stays blue and node 3 becomes red.
- We delete the node 3
- We delete the node 5 -> node 4 becomes red.
- We delete the node 4
- We delete the node 6


### Implementation

The purpose of this program is to test which Rust or C++ is the fastest to solve the problem. So I've just translated the C++ code of the [red-blue graph proglem solver 1](https://github.com/thomasarmel/red_blue_graph_solver_1) to Rust.
The algorithm has a linear complexity, bast case O(n) and worst case O(3n) = O(n).

Thanks to [Marcel](https://github.com/MarcelMARSAIS-LACOSTE) for the idea of the algorithm.