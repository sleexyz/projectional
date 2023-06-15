# Test that steps artifacts are overlaid in the correct order (post-order dep traversal).
# Note that builds are NOT deterministic up to sibling order.

load("//:rules_local/defs.bzl", "local_step", "local_test")

local_step(
    name = "a",
    srcs = ["a.txt"],
    cmd = "cat a.txt > a",
)

local_step(
    name = "b",
    cmd = "cat a > b",
    deps = [":a"],
)

local_step(
    name = "b_1",
    cmd = "echo b_1 > b",
    deps = [":b"],
)

local_step(
    name = "c",
    cmd = "echo c > c",
    deps = [
        ":b_1",
    ] + [
        ":b",
    ],
)

local_test(
    name = "test_1",
    cmd = '''
        set -ex
        [[ $(cat a) == "a" ]]
        [[ $(cat b) == "b_1" ]]
        [[ $(cat c) == "c" ]]
    ''',
    deps = [
        ":c",
    ],
)

local_step(
    name = "d",
    cmd = "echo d > d",
    deps = [
        ":b",
    ] + [
        ":b_1",
    ],
)

local_test(
    name = "test_2",
    cmd = '''
        set -ex
        [[ $(cat a) == "a" ]]
        [[ $(cat b) == "b_1" ]]
        [[ $(cat d) == "d" ]]
    ''',
    deps = [
        ":d",
    ],
)
