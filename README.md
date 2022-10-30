[`cucumber`] steps as a library
=========================

This repo is a basic example of how to use [`cucumber`] steps as a separate library.

- `/src` contains library to test
- `/steps` contains common steps as a library
- `/tests/cucumber.rs` contains additional steps
- `/tests/features` contains features to test library with

```commandline
cargo test --test cucumber
```

See [`cucumber#236`] for more info.




[`cucumber`]: https://docs.rs/cucumber
[`cucumber#236`]: https://github.com/cucumber-rs/cucumber/discussions/236
