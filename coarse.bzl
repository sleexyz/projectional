def _base_dir_impl(ctx):
    """
    TODO: support directories of arbitrary depth
    """
    output_dir = "__overlays__/%s" % ctx.attr.name
    output = ctx.actions.declare_directory(output_dir)

    command = """
    mv {path}/* {output}
    """.format(
        path= ctx.attr.path,
        output = output.path,
        output_root = output.path[:-(len(output_dir) + 1)],
    ).strip()

    ctx.actions.run_shell(
        inputs = depset(direct = ctx.files.srcs),
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            ctx.attr.name: output,
        },
    )

    return [
        DefaultInfo(
            files = depset([output]),
            runfiles = runfiles,
        ),
        DirInfo(
            last_overlay = ctx.attr.name,
            path = ctx.attr.path,
            transitive_deps_sources = depset([]),
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
        "transitive_deps_sources": "transitive sources",
        "last_overlay": "the last overlay applied to this directory",
    },
)

def _test_command_impl(ctx):
    output = ctx.actions.declare_file("%s.sh" % ctx.attr.name)

    runfiles = ctx.runfiles(
        root_symlinks = depset(
            transitive = [ctx.attr.dir[DefaultInfo].default_runfiles.root_symlinks],
        ),
    )

    print(runfiles.root_symlinks.to_list())

    command = '''
    if [[ {read_only} == "False" ]]; then
      # Replace with a writeable copy of the directory
      cp -Lr $RUNFILES_DIR/{dir_path} $TEST_TMPDIR/{dir_path}
      chmod -R u+w $TEST_TMPDIR/{dir_path}
      rm -rf $RUNFILES_DIR/{dir_path}
      mv $TEST_TMPDIR/{dir_path} $RUNFILES_DIR/{dir_path}
    fi

    (cd $RUNFILES_DIR/{dir_path}; {cmd})
    '''.format(
        read_only = ctx.attr.read_only,
        dir_path = ctx.attr.dir[DirInfo].path,
        cmd = ctx.attr.cmd,
    ).strip()

    ctx.actions.write(
        output = output,
        content = command,
    )

    return DefaultInfo(executable = output, runfiles = runfiles)

dir_test = rule(
    implementation = _test_command_impl,
    test = True,
    attrs = {
        "read_only": attr.bool(default = False),
        "cmd": attr.string(),
        "dir": attr.label(allow_files = True),
    },
)

def _dir_overlay_impl(ctx):
    """
    A rule that overlays a directory on top of another directory.
    """
    dir_path = ctx.attr.dir[DirInfo].path
    output_dir = "__overlays__/%s" % ctx.attr.name
    output = ctx.actions.declare_directory(output_dir)

    transitive_depsets = [
        dep[DefaultInfo].files
        for dep in ctx.attr.deps
    ] + [
        dep[DirInfo].transitive_deps_sources
        for dep in ctx.attr.deps
    ] + [
        ctx.attr.dir[DirInfo].transitive_deps_sources,
    ]

    dep_dirs = [
        file.path
        for file in depset(transitive = transitive_depsets).to_list()
    ]

    inputs = depset(direct = ctx.attr.dir[DefaultInfo].files.to_list(), transitive = transitive_depsets)

    command = '''
    set -e
    OUTPUT_ROOT={output_root}
    dep_dirs="{dep_dirs}"

    yellow='\033[1;33m'
    clear='\033[0m'
    debug() {{
        echo -e "$yellow $@ $clear"
    }}

    for dir in $dep_dirs; do
        if [ -z "$dir" ]; then
            continue
        fi
        package_name=$(basename $(dirname $(dirname $dir)))
        debug "Linking $dir to $OUTPUT_ROOT/$package_name"
        ln -s "$(realpath $dir)" "$OUTPUT_ROOT/$package_name"
    done

    cp -Lr $OUTPUT_ROOT/__overlays__/{last_overlay} $OUTPUT_ROOT/{dir_path}
    chmod -R u+w $OUTPUT_ROOT/{dir_path}

    (cd $OUTPUT_ROOT/{dir_path}; {cmd})
    mv $OUTPUT_ROOT/{dir_path}/* {output}
    '''.format(
        last_overlay = ctx.attr.dir[DirInfo].last_overlay,
        dep_dirs = " ".join(dep_dirs),
        dir_path = dir_path,
        cmd = ctx.attr.cmd,
        output = output.path,
        output_root = output.path[:-(len(output_dir) + 1)],
    ).strip()

    ctx.actions.run_shell(
        inputs = inputs,
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            ctx.attr.dir[DirInfo].path: output,
        },
    )
    runfiles = ctx.runfiles(
        root_symlinks = depset(
            direct = runfiles.root_symlinks.to_list(),
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ] + [
                ctx.attr.dir[DefaultInfo].default_runfiles.root_symlinks,
            ],
        ),
    )

    return [
        DefaultInfo(
            files = depset([output]),
            runfiles = runfiles,
        ),
        DirInfo(
            path = ctx.attr.dir[DirInfo].path,
            last_overlay = ctx.attr.name,
            transitive_deps_sources = depset(transitive = transitive_depsets),
        ),
    ]

dir_overlay = rule(
    implementation = _dir_overlay_impl,
    attrs = {
        "cmd": attr.string(),
        "dir": attr.label(allow_files = True),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
    },
)
