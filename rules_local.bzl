DirInfo = provider(
    fields = {
        "cwd": "Working directory for commands. Defaults to the directory of the BUILD file.",
        "transitive_deps_files": "Transitive files from deps.",
        "non_transitive_runfiles": "Runfiles that are not transitive.",
    },
)

def _local_exec_impl(ctx):
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

    if [ "$1" == "--no-exec" ]; then
        exit 0
    fi

    export DIR_ROOT=$(realpath ../../)
    {env_cmd}

    (cd $DIR_ROOT/{cwd}; {cmd})
    """.format(
        output = output.path,
        env_cmd = make_env_cmd(ctx.attr.env),
        cwd = ctx.attr.cwd,
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

def _local_exec(test):
    return rule(
        implementation = _local_exec_impl,
        test = test,
        executable = True,
        attrs = {
            "deps": attr.label_list(allow_files = True),
            "cmd": attr.string(),
            "cwd": attr.string(),
            "test": attr.bool(),
            "env": attr.string_dict(),
            # HACK: Users shouldn't need to specify this.
            "env_inherit": attr.string_list(),
        },
    )

def local_exec(_rule, name, cmd, test, deps = [], cwd = None, srcs = [], **kwargs):
    if cwd == None:
        cwd = native.package_name()
    if (len(srcs) > 0):
        local_step(
            name = "%s_lib" % name,
            deps = deps,
            cmd = "true",
            cwd = cwd,
            srcs = srcs,
        )
        deps = ["%s_lib" % name]

    _rule(
        name = name,
        cmd = cmd,
        deps = deps,
        cwd = cwd,
        test = test,
        **kwargs
    )

_local_exec_test = _local_exec(test = True)

def local_test(name, **kwargs):
    local_exec(
        _rule = _local_exec_test,
        name = name,
        test = True,
        **kwargs
    )

_local_exec_run = _local_exec(test = False)

def local_run(name, **kwargs):
    local_exec(
        _rule = _local_exec_run,
        name = name,
        test = False,
        **kwargs
    )

def make_env_cmd(env):
    return " ".join([
        "export %s=%s" % (key, value)
        for key, value in env.items()
    ])

def _local_step_impl(ctx):
    """
    A rule that overlays a directory on top of another directory.
    """
    cwd = ctx.attr.cwd
    # TODO: use qualified package name
    output_dir = "__overlays__/%s/%s.dir" % (ctx.attr.cwd, ctx.attr.name)
    output = ctx.actions.declare_symlink(output_dir)

    script = ctx.actions.declare_file("__overlays__/%s/%s.sh" % (ctx.attr.cwd, ctx.attr.name))

    transitive_depsets = [
        dep[DefaultInfo].files
        for dep in ctx.attr.deps
    ] + [
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

    for f in {dep_dirs}; do
        if [ -z "$f" ]; then
            continue
        fi

        if [[ $f == *.dir ]]; then
            rsync -a $f/ $OUTPUT_ROOT/
        else
            mkdir -p $OUTPUT_ROOT/$(dirname $f)
            ln -sf $(realpath $f) $OUTPUT_ROOT/$f
        fi
    done

    for dep in {direct_deps}; do
        mkdir -p $OUTPUT_ROOT/$(dirname $dep)
        cp -p $dep $OUTPUT_ROOT/$dep
    done

    mkdir -p $OUTPUT_ROOT/{cwd}

    script_path=$(realpath {script_path})

    set +e
    (cd $OUTPUT_ROOT/{cwd}; exec $script_path)
    CODE=$?
    set -e

    REAL_OUTPUT_ROOT=$(realpath $OUTPUT_ROOT)
    raw_outs="{outs}"
    outs=""
    for raw_out in $raw_outs; do
        (cd $OUTPUT_ROOT/{cwd}; mkdir -p $raw_out)
        out="$(cd $OUTPUT_ROOT/{cwd}; realpath --relative-to=$REAL_OUTPUT_ROOT $raw_out)"
        outs="$outs $out"
    done

    ln -sfn $REAL_OUTPUT_ROOT {output}
    exit $CODE
    '''.format(
        env_cmd = make_env_cmd(ctx.attr.env),
        dep_dirs = " ".join(dep_dirs),
        cwd = cwd,
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

    # HACK: The only way to create a dangling symlink is via ctx.actions.symlink
    sym_output = ctx.actions.declare_file("%s.sym" % output_dir)
    ctx.actions.symlink(
        output = sym_output,
        target_file = output,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            "%s.dir" % ctx.attr.cwd: sym_output,
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
            cwd = ctx.attr.cwd,
            transitive_deps_files = transitive_deps_files,
            non_transitive_runfiles = runfiles,
        ),
    ]

_local_step = rule(
    implementation = _local_step_impl,
    attrs = {
        "cmd": attr.string(),
        "cwd": attr.string(mandatory = True),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
        "env": attr.string_dict(),
        "outs": attr.string_list(default = ["."]),
    },
)

def _local_step_no_transitive_deps_impl(ctx):
    prev = ctx.attr.prev
    return [
        DefaultInfo(
            files = prev[DefaultInfo].files,
            # Remove transitive runfiles
            runfiles = prev[DirInfo].non_transitive_runfiles,
        ),
        DirInfo(
            cwd = prev[DirInfo].cwd,
            # Remove transitive deps
            transitive_deps_files = depset(),
            non_transitive_runfiles = prev[DirInfo].non_transitive_runfiles,
        ),
    ]

_local_step_no_transitive_deps = rule(
    implementation = _local_step_no_transitive_deps_impl,
    attrs = {
        "prev": attr.label(allow_files = True),
    },
)

def local_step(name, cwd = None, **kwargs):
    if cwd == None:
        cwd = native.package_name()
        if cwd == "":
            cwd = "."
    transitive_label = "%s.transitive" % name
    _local_step_no_transitive_deps(name = name, prev = transitive_label)
    _local_step(
        name = transitive_label,
        cwd = cwd,
        **kwargs
    )
