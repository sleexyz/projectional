# Test that steps artifacts are overlaid in the correct order (post-order dep traversal).
# Note that builds are NOT deterministic up to sibling order.

load("//:rules_hybrid/defs.bzl", "hybrid_step", "hybrid_test")

hybrid_step(
    name = "a",
    srcs = ["a.txt"],
    cmd = "cat a.txt > a",
)

hybrid_step(
    name = "b",
    cmd = "cat a > b",
    deps = [":a"],
)

hybrid_step(
    name = "b_1",
    cmd = "echo b_1 > b",
    deps = [":b"],
)

hybrid_step(
    name = "c",
    cmd = "echo c > c",
    deps = [
        ":b_1",
    ] + [
        ":b",
    ],
)

hybrid_test(
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

hybrid_step(
    name = "d",
    cmd = "echo d > d",
    deps = [
        ":b",
    ] + [
        ":b_1",
    ],
)

hybrid_test(
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
