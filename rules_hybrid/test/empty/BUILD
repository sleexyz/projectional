load("//:rules_hybrid/defs.bzl", "hybrid_run", "hybrid_step")

hybrid_run(
    name = "test",
    cmd = "true",
    deps = [
        "no_op",
    ],
)

hybrid_step(
    name = "no_op",
    cmd = "true",
)
