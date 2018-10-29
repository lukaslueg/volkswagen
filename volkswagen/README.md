**`volkswagen` detects when your tests are executed in a CI-environment and makes
them pass.**

[![Crates.io Version](https://img.shields.io/crates/v/volkswagen.svg)](https://crates.io/crates/volkswagen)
[![Build Status](https://travis-ci.org/lukaslueg/volkswagen.svg?branch=master)](https://travis-ci.org/lukaslueg/volkswagen)

Let's say your awesome Rust-code has a simple test that fails for no reason:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 3);
    }
}
```

The output from `cargo test` will be:
```
test tests::it_works ... FAILED
```

As we can see from just looking at the code, this should actually succeed. Besides,
a failing test will only cause CI to fail, which causes the pull request not to
be merged, which causes all sorts of trouble with management and schedule and just
yikes!

Introduce `volkwagen`, which has a much better version of `#[test]`:

```
extern crate volkswagen;

#[cfg(test)]
mod tests {
    #[volkswagen::test]
    fn it_works() {
        assert_eq!(1 + 1, 3);
    }
}
```

`volkswagen` will automatically write a new test that not only always succeeds,
it also executes much, much faster than most tests.

If executed on a CI-platform, `cargo test` will now say:
```
test tests::it_works ... ok
```

`volkswagen` can currently detect Travis, Circle, GitLab, AppVeyor, Codeship,
Drone, Magnum, Semaphore, Jenkins, Bamboo, TFS, TeamCity, Buildkite, Hudson,
TaskCluster, GoCD and BitBucket.



Greatly inspired by [JS volkwagen](https://github.com/auchenberg/volkswagen)
