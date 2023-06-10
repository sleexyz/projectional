DirInfo = provider(
    fields = {
        "path": "path to the directory from workspace root",
        "transitive_deps_files": "transitive files from deps",
        "non_transitive_runfiles": "runfiles that are not transitive",
    },
)

def _dir_exec_impl(ctx):
    script_filename = "%s.sh" % ctx.attr.name
    output = ctx.actions.declare_file(script_filename)

    runfiles = ctx.runfiles(
        root_symlinks = depset(
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ],
        ),
    )

    command = """
    #!/usr/bin/env bash

    set -e
    export RUNFILES_DIR=${{RUNFILES_DIR-$(dirname $(pwd))}}

    # Runfiles get persisted, so we need to clean up the directory
    # rm -rf $RUNFILES_DIR/{dir_path}

    # TODO: extract these in the right order
    for file in $RUNFILES_DIR/*.dir.tar; do
        tar -C $RUNFILES_DIR --keep-newer-files -xf $file 2>/dev/null >/dev/null
    done

    if [ "$1" == "--no-exec" ]; then
        exit 0
    fi

    export DIR_ROOT=$RUNFILES_DIR
    {env_cmd}

    export START_SCRIPT=$0

    (cd $RUNFILES_DIR/{dir_path}; {cmd})
    """.format(
        output= output.path,
        env_cmd = make_env_cmd(ctx.attr.env),
        dir_path = ctx.attr.path,
        cmd = ctx.attr.cmd,
    ).strip()

    ctx.actions.write(
        output = output,
        content = command,
        is_executable = True,
    )

    ret = [
        DefaultInfo(executable = output, runfiles = runfiles),
    ]
    if (ctx.attr.test):
        ret = ret + [
            RunEnvironmentInfo(
                inherited_environment = ctx.attr.env_inherit,
            ),
        ]
    return ret

def _dir_exec(test):
    return rule(
        implementation = _dir_exec_impl,
        test = test,
        executable = True,
        attrs = {
            "deps": attr.label_list(allow_files = True),
            "cmd": attr.string(),
            "path": attr.string(),
            "test": attr.bool(),
            "env": attr.string_dict(),
            # HACK: Users shouldn't need to specify this.
            "env_inherit": attr.string_list(),
        },
    )

def dir_exec(_rule, name, cmd, test, deps = [], path = None, srcs = [], **kwargs):
    if path == None:
        path = native.package_name()
    if (len(srcs) > 0):
        dir_step(
            name = "%s_lib" % name,
            deps = deps,
            cmd = "true",
            path = path,
            srcs = srcs,
        )
        deps = ["%s_lib" % name]

    _rule(
        name = name,
        cmd = cmd,
        deps = deps,
        path = path,
        test = test,
        **kwargs
    )

_dir_exec_test = _dir_exec(test = True)

def dir_test(name, **kwargs):
    dir_exec(
        _rule = _dir_exec_test,
        name = name,
        test = True,
        **kwargs
    )

_dir_exec_run = _dir_exec(test = False)

def dir_run(name, **kwargs):
    dir_exec(
        _rule = _dir_exec_run,
        name = name,
        test = False,
        **kwargs
    )

def make_env_cmd(env):
    return " ".join([
        "export %s=%s" % (key, value)
        for key, value in env.items()
    ])

def _dir_step_impl(ctx):
    """
    A rule that overlays a directory on top of another directory.
    """
    dir_path = ctx.attr.path
    output_dir = "__overlays__/%s.dir.tar" % ctx.attr.name
    output = ctx.actions.declare_file(output_dir)

    script = ctx.actions.declare_file("__overlays__/%s.sh" % ctx.attr.name)

    transitive_depsets = [
        dep[DefaultInfo].files
        for dep in ctx.attr.deps
    ]

    transitive_depsets = transitive_depsets + [
        dep[DirInfo].transitive_deps_files
        for dep in ctx.attr.deps
        if DirInfo in dep
    ]

    dep_dirs = [
        file.path
        for file in depset(transitive = transitive_depsets).to_list()
    ]

    command = '''
    set -e
    OUTPUT_ROOT={output_root}

    export DIR_ROOT="$(realpath $OUTPUT_ROOT)"
    {env_cmd}

    # TODO: extract these in the right order
    for f in {dep_dirs}; do
        if [ -z "$f" ]; then
            continue
        fi

        if [[ $f == *.dir.tar ]]; then
            tar -C $OUTPUT_ROOT -xf $f
        else
            mkdir -p $OUTPUT_ROOT/$(dirname $f)
            cp -p $f $OUTPUT_ROOT/$f
        fi
    done

    for dep in {direct_deps}; do
        mkdir -p $OUTPUT_ROOT/$(dirname $dep)
        cp -p $dep $OUTPUT_ROOT/$dep
    done

    mkdir -p $OUTPUT_ROOT/{dir_path}

    script_path=$(realpath {script_path})

    set +e
    (cd $OUTPUT_ROOT/{dir_path}; exec $script_path)
    CODE=$?
    set -e

    REAL_OUTPUT_ROOT=$(realpath $OUTPUT_ROOT)
    raw_outs="{outs}"
    outs=""
    for raw_out in $raw_outs; do
        (cd $OUTPUT_ROOT/{dir_path}; mkdir -p $raw_out)
        out="$(cd $OUTPUT_ROOT/{dir_path}; realpath --relative-to=$REAL_OUTPUT_ROOT $raw_out)"
        outs="$outs $out"
    done

    tar -C $OUTPUT_ROOT --exclude="__overlays__" -cf {output} $outs
    exit $CODE
    '''.format(
        env_cmd = make_env_cmd(ctx.attr.env),
        dep_dirs = " ".join(dep_dirs),
        # TODO: retire
        dir_path = dir_path,
        direct_deps = " ".join([file.path for file in ctx.files.srcs]),
        script_path = script.path,
        output = output.path,
        output_root = output.path[:-(len(output_dir) + 1)],
        outs = " ".join(ctx.attr.outs),
    ).strip()

    ctx.actions.write(
        output = script,
        content = ctx.attr.cmd,
        is_executable = True,
    )

    inputs = depset(
        direct = [script] + ctx.files.srcs,
        transitive = transitive_depsets,
    )

    ctx.actions.run_shell(
        inputs = inputs,
        env = ctx.attr.env,
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            "%s.dir.tar" % ctx.attr.path: output,
        },
    )
    transitive_runfiles = ctx.runfiles(
        root_symlinks = depset(
            direct = runfiles.root_symlinks.to_list(),
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ],
        ),
    )

    transitive_deps_files = depset(transitive = transitive_depsets)

    return [
        DefaultInfo(
            files = depset([output]),
            runfiles = transitive_runfiles,
        ),
        DirInfo(
            path = ctx.attr.path,
            transitive_deps_files = transitive_deps_files,
            non_transitive_runfiles = runfiles,
        ),
    ]

_dir_step = rule(
    implementation = _dir_step_impl,
    attrs = {
        "cmd": attr.string(),
        "path": attr.string(mandatory = True),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
        "env": attr.string_dict(),
        "outs": attr.string_list(default = ["."]),
    },
)

def _dir_step_no_transitive_deps_impl(ctx):
    prev = ctx.attr.prev
    return [
        DefaultInfo(
            files = prev[DefaultInfo].files,
            # Remove transitive runfiles
            runfiles = prev[DirInfo].non_transitive_runfiles,
        ),
        DirInfo(
            path = prev[DirInfo].path,
            # Remove transitive deps
            transitive_deps_files = depset(),
            non_transitive_runfiles = prev[DirInfo].non_transitive_runfiles,
        ),
    ]

_dir_step_no_transitive_deps = rule(
    implementation = _dir_step_no_transitive_deps_impl,
    attrs = {
        "prev": attr.label(allow_files = True),
    },
)

def dir_step(name, path = None, **kwargs):
    if path == None:
        path = native.package_name()
        if path == "":
            path = "."
    transitive_label = "%s.transitive" % name
    _dir_step_no_transitive_deps(name = name, prev = transitive_label)
    _dir_step(
        name = transitive_label,
        path = path,
        **kwargs
    )
