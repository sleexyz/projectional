def _base_dir_impl(ctx):
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

base_dir = rule(
    implementation = _base_dir_impl,
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

def _dir_test_impl(ctx):
    output = ctx.actions.declare_file("%s.sh" % ctx.attr.name)

    runfiles = ctx.runfiles(
        root_symlinks = depset(
            transitive = [ctx.attr.prev[DefaultInfo].default_runfiles.root_symlinks],
        ),
    )

    command = """
    for file in $RUNFILES_DIR/*.tar; do
        tar -C $RUNFILES_DIR -xf $file
    done

    (cd $RUNFILES_DIR/{dir_path}; {cmd})
    """.format(
        read_only = ctx.attr.read_only,
        dir_path = ctx.attr.prev[DirInfo].path,
        cmd = ctx.attr.cmd,
    ).strip()

    ctx.actions.write(
        output = output,
        content = command,
    )

    return [
        DefaultInfo(executable = output, runfiles = runfiles),
        RunEnvironmentInfo(
            inherited_environment = ctx.attr.env_inherit,
        ),
    ]

_dir_test = rule(
    implementation = _dir_test_impl,
    test = True,
    attrs = {
        "prev": attr.label(allow_files = True),
        "cmd": attr.string(),
        "read_only": attr.bool(default = False),
        "env_inherit": attr.string_list(),
    },
)

def dir_test(name, prev, cmd, srcs = [], **kwargs):
    if (len(srcs) > 0):
        step(
            name = "%s_lib" % name,
            prev = prev,
            cmd = "true",
            srcs = srcs,
        )
        prev = "%s_lib" % name

    _dir_test(
        name = name,
        cmd = cmd,
        prev = prev,
        **kwargs
    )

def _step_impl(ctx):
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
    dep_dirs="{dep_dirs}"

    for dir in $dep_dirs; do
        if [ -z "$dir" ]; then
            continue
        fi

        tar -xf $dir -C $OUTPUT_ROOT
    done

    tar -xf $OUTPUT_ROOT/__overlays__/{last_overlay_index}_{last_overlay}.tar -C $OUTPUT_ROOT/

    if [[ -d {dir_path} ]]; then
        cp -Lr {dir_path}/* $OUTPUT_ROOT/{dir_path}
    fi

    script_path=$(realpath {script_path})

    set +e
    (cd $OUTPUT_ROOT/{dir_path}; exec $script_path)
    CODE=$?
    set -e
    tar -C $OUTPUT_ROOT -cf {output} {dir_path}
    exit $CODE
    '''.format(
        last_overlay = ctx.attr.prev[DirInfo].last_overlay,
        last_overlay_index = last_overlay_index,
        dep_dirs = " ".join(dep_dirs),
        dir_path = dir_path,
        script_path = script.path,
        output = output.path,
        output_root = output.path[:-(len(output_dir) + 1)],
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

_step = rule(
    implementation = _step_impl,
    attrs = {
        "cmd": attr.string(),
        "prev": attr.label(allow_files = True),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
        "env": attr.string_dict(),
    },
)

def _step_no_transitive_deps_impl(ctx):
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

_step_no_transitive_deps = rule(
    implementation = _step_no_transitive_deps_impl,
    attrs = {
        "prev": attr.label(allow_files = True),
    },
)

def step(name, **kwargs):
    transitive_label = "%s.transitive" % name
    _step_no_transitive_deps(name = name, prev = transitive_label)
    _step(name = transitive_label, **kwargs)
