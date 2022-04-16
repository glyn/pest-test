# Reproduce a problem with the pest parser

Run:
```
$ cargo run
```

This should print out:
```
 --> 1:1
  |
1 |  $
  | ^---
  |
  = expected rootSelector
thread 'main' panicked at 'parse failure as expected: " --> 1:1\n  |\n1 |  $\n  | ^---\n  |\n  = expected rootSelector"', src/main.rs:18:10
```

But, instead, it prints out:
```
Test failed - should have errored already
```