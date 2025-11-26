# Cli

## Testing

You can test your changes to the `cli` crate by first building the main tehanu binary:

```
cargo build -p tehanu
```

And then building and running the `cli` crate with the following parameters:

```
 cargo run -p cli -- --tehanu ./target/debug/tehanu.exe
```
