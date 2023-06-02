"""
TODO: work with transitive deps
"""
def _dir_rule_impl(ctx):
    output = ctx.actions.declare_directory(ctx.attr.name)
    dep_dirs = []
    transitive = []
    for dep in ctx.attr.deps:
        transitive.append(dep.files)
        for file in dep.files.to_list():
            dep_dirs.append(file.path)

    command = '''
    set -ex
    OUTPUT_ROOT=$(dirname {output})
    for dir in "{dep_dirs}"; do
        if [ -z "$dir" ]; then
            continue
        fi
        ln -s "$(realpath $dir)" "$OUTPUT_ROOT/$(basename $dir)"
    done
    mv {name}/* {output}
    (cd {output}; {build_cmd})
    '''.format(
        dep_dirs = " ".join(dep_dirs),
        name = ctx.attr.name,
        build_cmd = ctx.attr.build_cmd,
        output = output.path,
    ).strip()

    ctx.actions.run_shell(
        inputs = depset(direct = ctx.files.srcs, transitive = transitive),
        outputs = [output],
        command = command,
        use_default_shell_env = True,
    )

    return DefaultInfo(files = depset([output]))

dir_rule = rule(
    implementation = _dir_rule_impl,
    attrs = {
        "build_cmd": attr.string(),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
    },
)
