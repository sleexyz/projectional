load("//:rules_hybrid/defs.bzl", "hybrid_step", "hybrid_test")

hybrid_step(
    name = "foo.txt",
    cmd = "echo foo > foo.txt",
    workspace_symlinks = ["foo.txt"],
)

hybrid_step(
    name = "some_dir",
    cmd = """
    mkdir -p some_dir
    echo bar > some_dir/bar.txt
    """,
    workspace_symlinks = ["some_dir"],
)

hybrid_test(
    name = "test",
    cmd = '''
    set -e
    DIR=$(dirname $(readlink -f BUILD))
    [[ $(cat $DIR/foo.txt) == foo ]]
    [[ $(cat $DIR/some_dir/bar.txt) == bar ]]
    ''',
    srcs = [
        "BUILD",
    ],
    deps = [
        ":foo.txt",
        ":some_dir",
    ],
)

hybrid_step(
    name = "asdf.txt",
    cwd = "%s/some_other_dir" % package_name(),
    cmd = "echo asdf > asdf.txt",
    workspace_symlinks = ["asdf.txt"],
)

hybrid_test(
    name = "test_cwd",
    cwd = "%s/some_other_dir" % package_name(),
    cmd = '''
    set -e
    DIR=$(dirname $(readlink -f ../BUILD))
    [[ $(cat $DIR/some_other_dir/asdf.txt) == asdf ]]
    ''',
    srcs = [
        "BUILD",
    ],
    deps = [
        ":asdf.txt",
    ],
)