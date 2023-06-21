load("//:rules_hybrid/defs.bzl", "hybrid_step", "hybrid_test")
load("//:cargo.bzl", "cargo_build_env", "cargo_with_tree_sitter_features")

package(default_visibility = ["//visibility:public"])

exports_files([
    "Cargo.toml",
])

hybrid_step(
    name = "generate",
    srcs = [
        "grammar.js",
        "src/scanner.cc",
    ],
    cmd = "tree-sitter generate",
    deps = [
        "//:cargo_workspace",
    ],
)

hybrid_test(
    name = "test_grammar",
    srcs = glob([
        "test/**/*",
    ]),
    cmd = "tree-sitter test",
    env_inherit = [
        "NIX_LDFLAGS",
        "NIX_CC_WRAPPER_TARGET_HOST_aarch64_apple_darwin",
    ],
    deps = [":generate"],
)

hybrid_step(
    name = "build",
    srcs = glob([
        "bindings/**/*",
    ]) + [
        "binding.gyp",
        "package.json",
        "package-lock.json",
    ],
    outs = [
        ".",
        "$CARGO_TARGET_DIR",
    ],
    cmd = cargo_with_tree_sitter_features("build"),
    env = cargo_build_env,
    deps = [
        ":generate",
        "//:cargo_workspace",
    ],
)

hybrid_test(
    name = "test",
    cmd = cargo_with_tree_sitter_features("test"),
    env = cargo_build_env,
    deps = [":build"],
)
