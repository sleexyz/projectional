load("//:rules_hybrid/defs.bzl", "hybrid_run", "hybrid_step")

hybrid_step(
    name = "npm_install",
    srcs = [
        "package.json",
        "package-lock.json",
    ],
    cmd = "npm install",
)

hybrid_step(
    name = "build",
    srcs =  glob([
        "media/**/*",
        "src/**/*",
    ]) + [
        "webpack.config.js",
        "tsconfig.json",
    ],
    cmd = "npm run compile",
    deps = [
        ":npm_install",
    ],
)

hybrid_run(
    name = "run",
    cmd = "npm run watch",
    tags = ["ibazel_notify_changes"],
    deps = [
        ":build"
    ],
)