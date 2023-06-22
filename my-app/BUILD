load("//:rules_hybrid/defs.bzl", "hybrid_run", "hybrid_step")
load("//:cargo.bzl", "cargo_build_env")

package(default_visibility = ["//visibility:public"])

exports_files(["Cargo.toml"])

hybrid_step(
    name = "npm_install",
    srcs = [
        "package.json",
        "package-lock.json",
    ],
    workspace_symlinks = [
        "node_modules",
    ],
    cmd = "npm install",
)

hybrid_step(
    name = "wasm_pack_build",
    srcs = glob([
        "src/**/*",
    ]),
    outs = [
        ".",
        "$CARGO_TARGET_DIR",
    ],
    cmd = "wasm-pack build --out-name index --dev",
    workspace_symlinks = [
        "pkg",
    ],
    env = cargo_build_env,
    no_propagate_changes = True,
    target_compatible_with = [
        "@platforms//cpu:wasm32",
    ],
    deps = [
        "//puddlejumper:build",
    ],
)

hybrid_step(
    name = "build",
    srcs = glob([
        "js/**/*",
    ]) + [
        "index.html",
        "launch_editor.sh",
        "tsconfig.json",
        "tsconfig.node.json",
        "vite.config.ts",
    ],
    workspace_symlinks = [
        "dist",
    ],
    cmd = "npm run build",
    deps = [
        ":npm_install",
        ":wasm_pack_build",
    ],
)

hybrid_run(
    name = "run",
    cmd = "npm run start",
    tags = ["ibazel_notify_changes"],
    deps = [":build"],
)
