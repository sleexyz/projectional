load("//:rules_hybrid/defs.bzl", "hybrid_run", "hybrid_step")

hybrid_step(
    name = "a",
    srcs = ["a.txt"],
    cmd = "cat a.txt > a",
)

hybrid_step(
    name = "b",
    cmd = "cat a > b",
    deps = [
        ":a",
    ],
)

hybrid_step(
    name = "c",
    cmd = "cat b > c",
    deps = [
        ":b",
    ],
)

# Seems like symlink outputs do not propagate in watch mode
hybrid_run(
    name = "debug",
    cmd = "echo $PWD",
    # tags = ["ibazel_notify_changes"],
    deps = [
        ":c",
    ],
)
