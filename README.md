# Espalier

Espalier is a very simple library (~300 lines) for flattened trees. While you can store a tree as `struct Node(Vec<Node>)` it can be difficult to work with due to Rust's borrowing rules, and also slow since each node needs a new `Vec`.

The obvious solution is to flatten the tree into a single `Vec`.

Consider this tree:

    0
    ├── 1
    │   └── 2
    ├── 3
    │   ├── 4
    │   │   └── 5
    │   └── 6
    ├── 7
    │   ├── 8
    │   │   ├── 9
    │   │   └── 10
    │   └── 11
    │       ├── 12
    │       └── 13
    └── 14

We can flatten it into this `Vec`:

| `i` | Value | Parent `i` | Number of Descendants |
|-----|-------|------------|-----------------------|
| 0   | 0     | 0          | 14                    |
| 1   | 1     | 0          | 1                     |
| 2   | 2     | 1          | 0                     |
| 3   | 3     | 0          | 3                     |
| 4   | 4     | 3          | 1                     |
| 5   | 5     | 4          | 0                     |
| 6   | 6     | 3          | 0                     |
| 7   | 7     | 0          | 6                     |
| 8   | 8     | 7          | 2                     |
| 9   | 9     | 8          | 0                     |
| 10  | 10    | 8          | 0                     |
| 11  | 11    | 7          | 2                     |
| 12  | 12    | 11         | 0                     |
| 13  | 13    | 11         | 0                     |
| 14  | 14    | 0          | 0                     |

The `Number of Descendants` requires a small amount of extra book-keeping while constructing the tree, but it allows fast iteration over the children of nodes.

    assert!(tree.children(0).map(|node| node.value).eq([1, 3, 7, 14]));

For child iteration at the top of large trees this may be slower than iterating a `struct Node(Vec<Node>)` tree due to poor cache locality.

## Constructing a Tree

There are two methods you need to use to construct a tree

    pub fn push(&mut self, value: V) -> K;
    pub fn up(&mut self);

`push()` adds a new child to the "current" node, and sets the current node to the new one. It returns the ID for the new node. The first time you call this it will create the root node.

`up()` sets the "current" node to its parent. So to create the above tree you run this code:

    tree.push(0);
    tree.push(1);
    tree.push(2);
    tree.up();
    tree.up();
    tree.push(3);
    tree.push(4);
    tree.push(5);
    tree.up();
    tree.up();
    tree.push(6);
    tree.up();
    tree.up();
    tree.push(7);
    tree.push(8);
    tree.push(9);
    tree.up();
    tree.push(10);
    tree.up();
    tree.up();
    tree.push(11);
    tree.push(12);
    tree.up();
    tree.push(13);
    tree.up();
    tree.up();
    tree.up();
    tree.push(14);

That's it!

## Accessing a Tree

You can access nodes using the IDs returned from `push()`. Or you can just make up node IDs - the ID type must be `Into<usize>` and the `usize` is just an index into the flattened tree.

There are also some convenient functions to iterate over a node's children, parents and descendants.

## Performance

I have not benchmarked this but it doesn't do anything stupid so it should be pretty fast. The main performance bottleneck will probably be calling `children()` on nodes with lots of descendants.

If you want to access all descendants of a node, then using `descendants()` will be *much* faster than recursively calling `children()`.

## See Also

This library was inspired by [`tree-flat`](https://github.com/mamcx/tree-flat) which I was going to use, but it has a number of issues:

* You can't have empty trees.
* There's no `children()` method (the one in that library is actually `descendants()` but misnamed).
* The code could be a lot simpler.
* The node ID parameter is not generic, so you cannot use the type system to avoid mixing up node IDs for different trees (see the excellent [`typed-index-collections` crate](https://github.com/zheland/typed-index-collections).
* It unnecessarily uses 3 `Vec`s instead of 2 or 1. I've opted for 1 for simplicity but 2 is a reasonable design too - it can improve cache locality, and also makes `into_iter().map(|node| node.value)` a NOP.
