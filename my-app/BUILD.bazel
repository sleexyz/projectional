load("//:rules_local/defs.bzl", "local_run", "local_step")
load("//:cargo.bzl", "cargo_build_env")

package(default_visibility = ["//visibility:public"])

exports_files(["Cargo.toml"])

local_step(
    name = "npm_install",
    srcs = [
        "package.json",
        "package-lock.json",
    ],
    cmd = "npm install",
)

local_step(
    name = "wasm_pack_build",
    srcs = glob([
        "src/**/*",
    ]),
    outs = [
        ".",
        "$CARGO_TARGET_DIR",
    ],
    cmd = "wasm-pack build --out-name index --dev",
    env = cargo_build_env,
    no_propagate_changes = True,
    target_compatible_with = [
        "@platforms//cpu:wasm32",
    ],
    deps = [
        "//puddlejumper:build",
    ],
)

local_step(
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
    cmd = "npm run build",
    deps = [
        ":npm_install",
        ":wasm_pack_build",
    ],
)

local_run(
    name = "run",
    cmd = "npm run start",
    tags = ["ibazel_notify_changes"],
    deps = [":build"],
)
