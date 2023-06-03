"""
TODO: work with transitive deps
"""

def _dir_impl(ctx):
    output = ctx.actions.declare_directory(ctx.attr.name)
    dep_dirs = []
    transitive = []
    for dep in ctx.attr.deps:
        transitive.append(dep.files)
        for file in dep.files.to_list():
            dep_dirs.append(file.path)

    command = '''
    OUTPUT_ROOT=$(dirname {output})
    for dir in "{dep_dirs}"; do
        if [ -z "$dir" ]; then
            continue
        fi
        ln -s "$(realpath $dir)" "$OUTPUT_ROOT/$(basename $dir)"
    done
    mv {name}/* {output}
    (cd {output}; {cmd})
    '''.format(
        dep_dirs = " ".join(dep_dirs),
        name = ctx.attr.name,
        cmd = ctx.attr.cmd,
        output = output.path,
    ).strip()

    ctx.actions.run_shell(
        inputs = depset(direct = ctx.files.srcs, transitive = transitive),
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            ctx.attr.name: output,
        },
    )
    runfiles = ctx.runfiles(
        root_symlinks = depset(
            direct = runfiles.root_symlinks.to_list(),
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ],
        ),
    )

    return [
        DefaultInfo(
            files = depset([output]),
            runfiles = runfiles,
        ),
        DirInfo(path = ctx.attr.name),
    ]

dir = rule(
    implementation = _dir_impl,
    attrs = {
        "cmd": attr.string(),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
    },
)

DirInfo = provider(
    fields = {
        "path": "path to the directory from workspace root",
    },
)

def _test_command_impl(ctx):
    output = ctx.actions.declare_file("%s.sh" % ctx.attr.name)

    runfiles = ctx.runfiles(
        root_symlinks = depset(
            transitive = [ctx.attr.dir[DefaultInfo].default_runfiles.root_symlinks],
        ),
    )

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
