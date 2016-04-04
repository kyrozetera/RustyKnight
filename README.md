# RustyKnight
Simple Knight's Tour algorithm in rust for finding paths with a known set of checkpoints,
implementing backtracking to output a list of all possible paths from the starting position.

If there are not enough known checkpoints, this approach will struggle with larger board sizes.

To run with default options, simply clone this repository and run `cargo run`

It also allows you to customize the board, size, and checkpoints through commandline options:
(All options are in JSON format)

```bash
$ cargo build
...
$ ./target/debug/rusty_knight --size '{"x":7,"y":7}' --start '{"x":6,"y":6}' --checkpoints '{11:{"x":5,"y":3}}'
```

Checkpoints are known positions the knight must hit on a given move number.
In the above example, the knight must land on square (5,3) on his 11th move.

Run `rusty_knight -h` to get more usage information.
