# Developing Gram

See the platform-specific instructions for building Gram from source:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)

## Keychain access

Gram stores secrets in the system keychain.

However, when running a development build of Gram on macOS (and perhaps other
platforms) trying to access the keychain results in a lot of keychain prompts
that require entering your password over and over.

On macOS this is caused by the development build not having a stable identity.
Even if you choose the "Always Allow" option, the OS will still prompt you for
your password again the next time something changes in the binary.

This quickly becomes annoying and impedes development speed.

That is why, by default, when running a development build of Gram an alternative
credential provider is used in order to bypass the system keychain.

> Note: This is **only** the case for development builds. For all non-development
> release channels the system keychain is always used.

If you need to test something out using the real system keychain in a
development build, run Gram with the following environment variable set:

```
GRAM_DEVELOPMENT_USE_KEYCHAIN=1
```

## Performance Measurements

Gram includes a frame time measurement system that can be used to profile how long it takes to render each frame. This is particularly useful when comparing rendering performance between different versions or when optimizing frame rendering code.

### Using GRAM_MEASUREMENTS

To enable performance measurements, set the `GRAM_MEASUREMENTS` environment variable:

```sh
export GRAM_MEASUREMENTS=1
```

When enabled, Gram will print frame rendering timing information to stderr, showing how long each frame takes to render.

### Performance Comparison Workflow

Here's a typical workflow for comparing frame rendering performance between different versions:

1. **Enable measurements:**

   ```sh
   export GRAM_MEASUREMENTS=1
   ```

2. **Test the first version:**

   - Checkout the commit you want to measure
   - Run Gram in release mode and use it for 5-10 seconds: `cargo run --release &> version-a`

3. **Test the second version:**

   - Checkout another commit you want to compare
   - Run Gram in release mode and use it for 5-10 seconds: `cargo run --release &> version-b`

4. **Generate comparison:**

   ```sh
   script/histogram version-a version-b
   ```

The `script/histogram` tool can accept as many measurement files as you like and will generate a histogram visualization comparing the frame rendering performance data between the provided versions.

### Using `util_macros::perf`

For benchmarking unit tests, annotate them with the `#[perf]` attribute from the `util_macros` crate. Then run `cargo
perf-test -p $CRATE` to benchmark them. See the rustdoc documentation on `crates/util_macros` and `tooling/perf` for
in-depth examples and explanations.

