load("//:rules_hybrid/defs.bzl", "hybrid_run", "hybrid_step")

hybrid_step(
    name = "npm_install",
    srcs = [
        "package.json",
        "package-lock.json",
    ],
    cmd = "npm install",
)

filegroup(
    name = "sources",
    srcs =  glob([
        "media/**/*",
        "src/**/*",
    ]) + [
        "webpack.config.js",
        "tsconfig.json",
    ],
)

hybrid_step(
    name = "build",
    srcs = [ ":sources" ],
    cmd = "npm run compile",
    deps = [
        ":npm_install",
    ],
)

hybrid_run(
    name = "run",
    srcs = [ ":sources" ],
    cmd = "npm run watch",
    tags = ["ibazel_notify_changes"],
    deps = [
        ":npm_install",
    ],
)