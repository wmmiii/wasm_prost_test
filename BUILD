load("@crate_index//:defs.bzl", "aliases")
load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//wasm_bindgen/rules_js:defs.bzl", "js_rust_wasm_bindgen")

proto_library(
    name = "proto_lib",
    srcs = [
        "test.proto",
    ],
)

rust_prost_library(
    name = "rust_proto",
    proto = ":proto_lib",
)

rust_binary(
    name = "rust_bin",
    srcs = ["main.rs"],
    aliases = aliases(),
    crate_features = ["js"],
    edition = "2021",
    deps = [
        "//:rust_proto",
        "@crate_index//:getrandom",
        "@crate_index//:imp",
        "@rules_rust//wasm_bindgen/3rdparty:wasm_bindgen",
    ],
)

js_rust_wasm_bindgen(
    name = "rust_wasm",
    target = "web",
    visibility = ["//editor/src:__pkg__"],
    wasm_file = ":rust_bin",
)
