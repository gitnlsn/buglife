# buglife
Rust implementation of Buglife problem at spoj

Project was compiled with rustc v1.40.0 (73528e339 2019-12-16)

Github uses v1.33. So compilation issues are expected.

```
error[E0502]: cannot borrow `*self` as mutable because `self.bugs` is also borrowed as immutable
   --> prog.rs:163:21
    |
149 |         let bug = self.bugs.get(bug_id).unwrap();
    |                   --------- immutable borrow occurs here
...
163 |                 if !self.is_consistent (
    |                     ^^^^ mutable borrow occurs here
...
173 |     }
    |     - immutable borrow ends here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
```
