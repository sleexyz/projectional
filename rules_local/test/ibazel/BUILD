load("//:rules_local/defs.bzl", "local_run", "local_step")

local_step(
    name = "a",
    srcs = ["a.txt"],
    cmd = "cat a.txt > a",
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
    cmd = "echo $PWD",
    # tags = ["ibazel_notify_changes"],
    deps = [
        ":c",
    ],
)
