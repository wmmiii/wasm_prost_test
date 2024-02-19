# WASM Prost Test

This repo is intended to demonstrate an error with the transitive dependency loading of rust_proto specifically with rust_prost* rules.

Note that the following command works correctly:
```
bazel build //:rust_bin
```
(You can even `run` it if you want!)

However, when trying to build the binary for a `wasm` target the following error occurs:
```
$ bazel build //:rust_wasm
INFO: Analyzed target //:rust_wasm (0 packages loaded, 0 targets configured).
ERROR: /home/user/.cache/bazel/_bazel_user/2f06e3936a9a01a1d5e3adde0ef0c906/external/rules_rust_prost__getrandom-0.2.10/BUILD.bazel:13:13: Compiling Rust rlib getrandom v0.2.10 (31 files) failed: (Exit 1): process_wrapper failed: error executing Rustc command (from target @@rules_rust_prost__getrandom-0.2.10//:getrandom) bazel-out/k8-opt-exec-ST-13d3ddad9198/bin/external/rules_rust/util/process_wrapper/process_wrapper --subst 'pwd=${pwd}' -- ... (remaining 23 arguments skipped)

Use --sandbox_debug to see verbose messages from the sandbox and retain the sandbox build root for debugging
error: the wasm*-unknown-unknown targets are not supported by default, you may need to enable the "js" feature. For more information see: https://docs.rs/getrandom/#webassembly-support
   --> external/rules_rust_prost__getrandom-0.2.10/src/lib.rs:285:9
    |
285 | /         compile_error!("the wasm*-unknown-unknown targets are not supported by \
286 | |                         default, you may need to enable the \"js\" feature. \
287 | |                         For more information see: \
288 | |                         https://docs.rs/getrandom/#webassembly-support");
    | |________________________________________________________________________^

error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> external/rules_rust_prost__getrandom-0.2.10/src/lib.rs:341:9
    |
341 |         imp::getrandom_inner(dest)?;
    |         ^^^ use of undeclared crate or module `imp`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
Target //:rust_wasm failed to build
Use --verbose_failures to see the command lines of failed build steps.
INFO: Elapsed time: 0.373s, Critical Path: 0.17s
INFO: 19 processes: 18 internal, 1 linux-sandbox.
ERROR: Build did NOT complete successfully
```