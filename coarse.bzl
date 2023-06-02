"""
TODO: work with transitive deps
"""

def _dir_rule_impl(ctx):
    output = ctx.actions.declare_file("%s.dir.tar" % ctx.attr.name)
    tars = []
    transitive = []
    for dep in ctx.attr.deps:
        transitive.append(dep.files)
        for file in dep.files.to_list():
            tars.append(file.path)

    command = '''
    for tar in "{tars}"; do
        tar -xf $tar
    done
    (cd {name}; {build_cmd})
    tar -cf {output} {name}
    '''.format(
        tars = " ".join(tars),
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
        "exclude": attr.string_list(default = []),
    },
)
