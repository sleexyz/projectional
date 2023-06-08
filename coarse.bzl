def _dir_impl(ctx):
    """
    TODO: support directories of arbitrary depth
    """
    output_dir = "__overlays__/%s_%s.tar" % (0, ctx.attr.name)
    output = ctx.actions.declare_file(output_dir)

    command = """
    mkdir -p {path}
    tar -cf {output} {path}
    """.format(
        path = ctx.attr.path,
        output = output.path,
    ).strip()

    ctx.actions.run_shell(
        inputs = depset(direct = ctx.files.srcs),
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            "%s.tar" % ctx.attr.path: output,
        },
    )

    return [
        DefaultInfo(
            files = depset([output]),
            runfiles = runfiles,
        ),
        DirInfo(
            last_overlay = ctx.attr.name,
            last_overlay_index = 0,
            path = ctx.attr.path,
            transitive_deps_files = depset([]),
            non_transitive_runfiles = runfiles,
        ),
    ]

dir = rule(
    implementation = _dir_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True),
        "path": attr.string(),
    },
)

DirInfo = provider(
    fields = {
        "path": "path to the directory from workspace root",
        "transitive_deps_files": "transitive files from deps",
        "last_overlay": "the last overlay applied to this directory",
        "last_overlay_index": "the index of the last overlay",
        "non_transitive_runfiles": "runfiles that are not transitive",
    },
)

def _dir_exec_impl(ctx):
    output = ctx.actions.declare_file("%s.sh" % ctx.attr.name)

    runfiles = ctx.runfiles(
        root_symlinks = depset(
            transitive = [ctx.attr.prev[DefaultInfo].default_runfiles.root_symlinks],
        ),
    )

    command = """
    #!/usr/bin/env bash
    set -e
    RUNFILES_DIR=${{RUNFILES_DIR-$(dirname $(pwd))}}

    # Runfiles get persisted, so we need to clean up the directory
    rm -rf $RUNFILES_DIR/{dir_path}

    for file in "$RUNFILES_DIR/*.tar"; do
        tar -C $RUNFILES_DIR -xf $file
    done

    export DIR_ROOT=$RUNFILES_DIR
    {env_cmd}

    (cd $RUNFILES_DIR/{dir_path}; {cmd})
    """.format(
        env_cmd = make_env_cmd(ctx.attr.env),
        dir_path = ctx.attr.prev[DirInfo].path,
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
            "prev": attr.label(allow_files = True),
            "cmd": attr.string(),
            "test": attr.bool(),
            "env": attr.string_dict(),
            # HACK: Users shouldn't need to specify this.
            "env_inherit": attr.string_list(),
        },
    )

def dir_exec(_rule, name, prev, cmd, test, srcs = [], **kwargs):
    if (len(srcs) > 0):
        dir_step(
            name = "%s_lib" % name,
            prev = prev,
            cmd = "true",
            srcs = srcs,
        )
        prev = "%s_lib" % name

    _rule(
        name = name,
        cmd = cmd,
        prev = prev,
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
    dir_path = ctx.attr.prev[DirInfo].path
    last_overlay_index = ctx.attr.prev[DirInfo].last_overlay_index
    output_dir = "__overlays__/%s_%s.tar" % (last_overlay_index + 1, ctx.attr.name)
    output = ctx.actions.declare_file(output_dir)

    script = ctx.actions.declare_file("%s.sh" % ctx.attr.name)

    transitive_depsets = [
        dep[DefaultInfo].files
        for dep in ctx.attr.deps
    ]

    transitive_depsets = transitive_depsets + [
        dep[DirInfo].transitive_deps_files
        for dep in ctx.attr.deps
    ] + [
        ctx.attr.prev[DirInfo].transitive_deps_files,
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

    dep_dirs="{dep_dirs}"

    for dir in $dep_dirs; do
        if [ -z "$dir" ]; then
            continue
        fi

        # TODO: make sure we extract these in the right order
        tar -xf $dir -C $OUTPUT_ROOT
    done

    tar -xf $OUTPUT_ROOT/__overlays__/{last_overlay_index}_{last_overlay}.tar -C $OUTPUT_ROOT/

    if [[ -d {dir_path} ]]; then
        cp -r -p {dir_path}/* $OUTPUT_ROOT/{dir_path}
    fi

    script_path=$(realpath {script_path})

    set +e
    (cd $OUTPUT_ROOT/{dir_path}; exec $script_path)
    CODE=$?
    set -e

    REAL_OUTPUT_ROOT=$(realpath $OUTPUT_ROOT)
    raw_outs="{outs}"
    outs=""
    for raw_out in $raw_outs; do
        out="$(cd $OUTPUT_ROOT/{dir_path}; realpath --relative-to=$REAL_OUTPUT_ROOT $raw_out)"
        # echo "raw_out: $raw_out"
        echo "out: $out"
        outs="$outs $out"
    done

    tar -C $OUTPUT_ROOT -cf {output} $outs
    exit $CODE
    '''.format(
        env_cmd = make_env_cmd(ctx.attr.env),
        last_overlay = ctx.attr.prev[DirInfo].last_overlay,
        last_overlay_index = last_overlay_index,
        dep_dirs = " ".join(dep_dirs),
        dir_path = dir_path,
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
        direct = ctx.attr.prev[DefaultInfo].files.to_list() + [script] + ctx.files.srcs,
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
            "%s.tar" % ctx.attr.prev[DirInfo].path: output,
        },
    )
    transitive_runfiles = ctx.runfiles(
        root_symlinks = depset(
            direct = runfiles.root_symlinks.to_list(),
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ] + [
                ctx.attr.prev[DefaultInfo].default_runfiles.root_symlinks,
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
            path = ctx.attr.prev[DirInfo].path,
            last_overlay = ctx.attr.name,
            last_overlay_index = last_overlay_index + 1,
            transitive_deps_files = transitive_deps_files,
            non_transitive_runfiles = runfiles,
        ),
    ]

_dir_step = rule(
    implementation = _dir_step_impl,
    attrs = {
        "cmd": attr.string(),
        "prev": attr.label(allow_files = True),
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
            last_overlay = prev[DirInfo].last_overlay,
            last_overlay_index = prev[DirInfo].last_overlay_index,
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

def dir_step(name, **kwargs):
    transitive_label = "%s.transitive" % name
    _dir_step_no_transitive_deps(name = name, prev = transitive_label)
    _dir_step(name = transitive_label, **kwargs)
