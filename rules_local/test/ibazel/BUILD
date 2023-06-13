load("//:rules_local/defs.bzl", "local_run", "local_step")

local_step(
    name = "a",
    cmd = "cat a.txt > a",
    srcs = ["a.txt"]
)

local_step(
    name = "b",
    cmd = "cat a > b",
    deps = [
        ":a",
    ],
)

local_step(
    name = "c",
    cmd = "cat b > c",
    deps = [
        ":b",
    ],
)

# Seems like symlink outputs do not propagate in watch mode
local_run(
    name = "debug",
    cmd = "ag .",
    # tags = ["ibazel_notify_changes"],
    deps = [
        ":c",
    ],
)