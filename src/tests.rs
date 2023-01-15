use crate::Tree;

// Example tree used for tests. It contains multiple roots.
//
// 0
// ├── 1
// │   └── 2
// ├── 3
// │   ├── 4
// │   │   └── 5
// │   └── 6
// ├── 7
// │   ├── 8
// │   │   ├── 9
// │   │   └── 10
// │   └── 11
// │       ├── 12
// │       └── 13
// └── 14
// 15
// ├── 16
// │   └── 17
// └── 18
//

/// Build example tree.
/// TODO: Multiple roots.
fn build() -> Tree<usize, i32> {
    let mut tree = Tree::with_capacity(15);
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
    tree.up();
    tree.up(); // Up beyond first root.
    tree.push(15);
    tree.push(16);
    tree.push(17);
    tree.up();
    tree.up();
    tree.push(18);
    tree
}

/// Basic checks on tree creation (length etc.)
#[test]
fn create() {
    let tree = build();
    assert_eq!(tree.len(), 19);
    assert_eq!(tree.get(18).unwrap().value, 18);
    assert!(tree.get(19).is_none());
}

/// Test iterating over the nodes in pre-order.
#[test]
fn iter() {
    let tree = build();
    let data: Vec<i32> = tree.into_iter().map(|node| node.value).collect();

    assert_eq!(data, (0..19).collect::<Vec<i32>>());
}

/// Check the number of descendants for each node is correct.
#[test]
fn num_descendants() {
    let tree = build();

    assert!(tree
        .iter()
        .map(|node| node.num_descendants())
        .eq([14, 1, 0, 3, 1, 0, 0, 6, 2, 0, 0, 2, 0, 0, 0, 3, 1, 0, 0,]));
}

/// Check parent iterators give the right sequences.
#[test]
fn parents() {
    let tree = build();

    assert!(tree.parents(0).map(|node| node.value).eq([]));
    assert!(tree.parents(1).map(|node| node.value).eq([0]));
    assert!(tree.parents(2).map(|node| node.value).eq([1, 0]));
    assert!(tree.parents(3).map(|node| node.value).eq([0]));
    assert!(tree.parents(4).map(|node| node.value).eq([3, 0]));
    assert!(tree.parents(5).map(|node| node.value).eq([4, 3, 0]));
    assert!(tree.parents(6).map(|node| node.value).eq([3, 0]));
    assert!(tree.parents(7).map(|node| node.value).eq([0]));
    assert!(tree.parents(8).map(|node| node.value).eq([7, 0]));
    assert!(tree.parents(9).map(|node| node.value).eq([8, 7, 0]));
    assert!(tree.parents(10).map(|node| node.value).eq([8, 7, 0]));
    assert!(tree.parents(11).map(|node| node.value).eq([7, 0]));
    assert!(tree.parents(12).map(|node| node.value).eq([11, 7, 0]));
    assert!(tree.parents(13).map(|node| node.value).eq([11, 7, 0]));
    assert!(tree.parents(14).map(|node| node.value).eq([0]));
    assert!(tree.parents(15).map(|node| node.value).eq([]));
    assert!(tree.parents(16).map(|node| node.value).eq([15]));
    assert!(tree.parents(17).map(|node| node.value).eq([16, 15]));
    assert!(tree.parents(18).map(|node| node.value).eq([15]));
    assert!(tree.parents(19).map(|node| node.value).eq([]));
}

/// Check children iterators give the right sequences.
#[test]
fn children() {
    let tree = build();

    assert!(tree.children(0).map(|node| node.value).eq([1, 3, 7, 14]));
    assert!(tree.children(1).map(|node| node.value).eq([2]));
    assert!(tree.children(2).map(|node| node.value).eq([]));
    assert!(tree.children(3).map(|node| node.value).eq([4, 6]));
    assert!(tree.children(4).map(|node| node.value).eq([5]));
    assert!(tree.children(5).map(|node| node.value).eq([]));
    assert!(tree.children(6).map(|node| node.value).eq([]));
    assert!(tree.children(7).map(|node| node.value).eq([8, 11]));
    assert!(tree.children(8).map(|node| node.value).eq([9, 10]));
    assert!(tree.children(9).map(|node| node.value).eq([]));
    assert!(tree.children(10).map(|node| node.value).eq([]));
    assert!(tree.children(11).map(|node| node.value).eq([12, 13]));
    assert!(tree.children(12).map(|node| node.value).eq([]));
    assert!(tree.children(13).map(|node| node.value).eq([]));
    assert!(tree.children(14).map(|node| node.value).eq([]));
    assert!(tree.children(15).map(|node| node.value).eq([16, 18]));
    assert!(tree.children(16).map(|node| node.value).eq([17]));
    assert!(tree.children(17).map(|node| node.value).eq([]));
    assert!(tree.children(18).map(|node| node.value).eq([]));
    assert!(tree.children(19).map(|node| node.value).eq([]));
}

/// Test first & last.
#[test]
fn first_last() {
    let tree = build();
    assert_eq!(tree.first().unwrap().value, 0);
    assert_eq!(tree.last().unwrap().value, 18);

    let empty = Tree::<usize, i32>::new();
    assert!(empty.first().is_none());
    assert!(empty.last().is_none());
}
