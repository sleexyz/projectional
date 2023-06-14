"""
Rules optimized for local development with existing toolchains.
"""

DirInfo = provider(
    "Information about a directory.",
    fields = {
        "transitive_deps_files": "Transitive files from deps.",
    },
)

def _local_exec_impl(ctx):
    bootstrap_script = ctx.actions.declare_file("__%s_bootstrap__.sh" % ctx.attr.name)
    script = ctx.actions.declare_file("__%s__" % ctx.attr.name)

    runfiles = ctx.runfiles(
        files = [script, bootstrap_script],
        root_symlinks = depset(
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ],
            order = "postorder",
        ),
    )

    bootstrap_cmd = """
    #!/usr/bin/env bash
    set -ex

    script_path=$(realpath {script_path})

    export DIR_ROOT=$(realpath ../../)
    {env_cmd}

    mkdir -p $DIR_ROOT/{cwd}
    (cd $DIR_ROOT/{cwd}; exec $script_path)
    """.format(
        script_path = script.short_path,
        env_cmd = make_env_cmd(ctx.attr.env),
        cwd = ctx.attr.cwd,
    ).strip()

    ctx.actions.write(
        output = bootstrap_script,
        content = bootstrap_cmd,
        is_executable = True,
    )

    ctx.actions.write(
        output = script,
        content = ctx.attr.cmd,
        is_executable = True,
    )

    ret = [
        DefaultInfo(
            executable = bootstrap_script,
            files = depset([bootstrap_script, script]),
            runfiles = runfiles,
        ),
    ]
    if (ctx.attr.test):
        ret = ret + [
            RunEnvironmentInfo(
                inherited_environment = ctx.attr.env_inherit,
            ),
        ]
    return ret

def _make_local_exec(test):
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

def _local_exec(_rule_impl, name, cmd, test, deps = [], cwd = None, srcs = [], **kwargs):
    """
    A rule that executes a command in a directory.

    Args:
        _rule_impl: The underlying rule implementation.
        name: The name of the rule.
        cmd: The command to execute.
        test: Whether or not this is a test.
        deps: The dependencies of the rule.
        cwd: The directory to execute the command in. Ignored for run rules.
        srcs: The sources of the rule.
        **kwargs: Additional arguments to pass to the underlying rule implementation.
    """
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

    _rule_impl(
        name = name,
        cmd = cmd,
        deps = deps,
        cwd = cwd,
        test = test,
        **kwargs
    )

_local_exec_test = _make_local_exec(test = True)

def local_test(name, **kwargs):
    _local_exec(
        _rule_impl = _local_exec_test,
        name = name,
        test = True,
        **kwargs
    )

_local_exec_run = _make_local_exec(test = False)

def local_run(name, **kwargs):
    _local_exec(
        _rule_impl = _local_exec_run,
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
    output_dir = "__overlays__/%s/%s.dir" % (ctx.attr.package_name, ctx.attr.name)
    output = ctx.actions.declare_symlink(output_dir)
    output_phony = ctx.actions.declare_file("%s.phony" % output_dir)

    script = ctx.actions.declare_file("__overlays__/%s/%s.sh" % (ctx.attr.cwd, ctx.attr.name))

    transitive_depsets = [
        dep[DirInfo].transitive_deps_files
        for dep in ctx.attr.deps
        if DirInfo in dep
    ] + [
        dep[DefaultInfo].files
        for dep in ctx.attr.deps
    ]

    dep_dirs = [
        file.path
        for file in depset(transitive = transitive_depsets, order = "postorder").to_list()
    ]

    command = '''
    set -e
    OUTPUT_ROOT={output_root}
    OUTPUT_ROOT_ABSOLUTE=$(realpath $OUTPUT_ROOT)
    export DIR_ROOT="$(realpath $OUTPUT_ROOT)"
    {env_cmd}

    for f in {dep_dirs}; do
        if [[ $f == *.dir ]]; then
            if [[ $(realpath $f) != $(realpath $OUTPUT_ROOT) ]]; then
                echo "f: $f"
                YELLOW='\033[1;33m'
                CLEAR='\033[0m'
                # set -x
                # echo -e "${{YELLOW}}"
                # (cd $(realpath $f); find  .)
                # rsync -avv --force $f/ $OUTPUT_ROOT/
                rsync -a $f/ $OUTPUT_ROOT/
                # lndir $(realpath $f) $OUTPUT_ROOT/
                # cp -as $f $OUTPUT_ROOT/
                # echo -e "${{CLEAR}}"
                # set +x
            fi
        elif [[ $f == *.dir.sym ]]; then
            true
        else
            mkdir -p $OUTPUT_ROOT/$(dirname $f)
            ln -sf $(realpath $f) $OUTPUT_ROOT/$f
        fi
    done

    for dep in {direct_deps}; do
        mkdir -p $OUTPUT_ROOT/$(dirname $dep)
        cp -p $dep $OUTPUT_ROOT/$dep
        # ln -sfn $(realpath $dep) $OUTPUT_ROOT/$dep
        # rsync -a --force $(realpath $f)/ $OUTPUT_ROOT/
    done

    mkdir -p $OUTPUT_ROOT/{cwd}

    script_path=$(realpath {script_path})

    # set +e
    # (cd $OUTPUT_ROOT/; find .)
    (cd $OUTPUT_ROOT/{cwd}; exec $script_path)
    CODE=$?
    # set -e

    raw_outs="{outs}"

    OUTPUT_ABSOLUTE=$(realpath {output})
    # TODO: Do some filtering here
    # outs=""
    for raw_out in $raw_outs; do
        (cd $OUTPUT_ROOT/{cwd}; mkdir -p $raw_out)
        out="$(cd $OUTPUT_ROOT/{cwd}; realpath --relative-to=$OUTPUT_ROOT_ABSOLUTE $raw_out)"
        # outs="$outs $out"
        # (cd $OUTPUT_ROOT/{cwd}; rsync -avv --force $(realpath $raw_out)/ $OUTPUT_ABSOLUTE/$out)
        # (cd $OUTPUT_ROOT/{cwd}; cp -as $(realpath $raw_out) $OUTPUT_ABSOLUTE/$out)
        # (cd $OUTPUT_ROOT/{cwd}; ln -sfn $(realpath $raw_out) $OUTPUT_ABSOLUTE/$out)
    done

    rm -f {output}
    ln -sfn $OUTPUT_ROOT_ABSOLUTE {output}
    echo $RANDOM > {output_phony}
    exit $CODE
    '''.format(
        env_cmd = make_env_cmd(ctx.attr.env),
        dep_dirs = " ".join(dep_dirs),
        cwd = cwd,
        direct_deps = " ".join([file.path for file in ctx.files.srcs]),
        script_path = script.path,
        output = output.path,
        output_phony = output_phony.path,
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
        order = "postorder",
    )

    # HACK: This artifact is only used to declare the symlink in the build graph.
    # This artifact is otherwise ignored.
    sym_output = ctx.actions.declare_file("%s.sym" % output_dir)

    ctx.actions.run_shell(
        inputs = inputs,
        env = ctx.attr.env,
        outputs = [output, output_phony],
        command = command,
        use_default_shell_env = True,
    )

    ctx.actions.symlink(
        output = sym_output,
        target_file = output,
    )

    runfiles = ctx.runfiles(
        root_symlinks = {
            # "%s.dir" % ctx.attr.cwd: sym_output,
            # output_dir: output,
            "%s.sym" % output_dir: sym_output,
        },
    )
    runfiles = ctx.runfiles(
        root_symlinks = depset(
            direct = runfiles.root_symlinks.to_list(),
            transitive = [
                dep[DefaultInfo].default_runfiles.root_symlinks
                for dep in ctx.attr.deps
            ],
            order = "postorder",
        ),
    )

    transitive_deps_files = depset(transitive = transitive_depsets, order = "postorder")

    files = [output]
    if not ctx.attr.no_propagate_changes:
        files.append(output_phony)

    return [
        DefaultInfo(
            files = depset(files),
            runfiles = runfiles,
        ),
        DirInfo(
            transitive_deps_files = transitive_deps_files,
        ),
    ]

_local_step = rule(
    implementation = _local_step_impl,
    attrs = {
        "cmd": attr.string(),
        "no_propagate_changes": attr.bool(default = False),
        "cwd": attr.string(mandatory = True),
        "package_name": attr.string(mandatory = True),
        "srcs": attr.label_list(allow_files = True),
        "deps": attr.label_list(allow_files = True),
        "env": attr.string_dict(),
        "outs": attr.string_list(default = ["."]),
    },
)

def local_step(name, cwd = None, **kwargs):
    """
    Run a command.

    Args:
        name: A descriptive name for this step.
        cwd: The working directory to run the command in.
        **kwargs: Additional arguments to pass to the underlying rule.
    """
    if cwd == None:
        cwd = native.package_name()
        if cwd == "":
            cwd = "."
    _local_step(
        name = name,
        cwd = cwd,
        package_name = native.package_name(),
        **kwargs
    )
