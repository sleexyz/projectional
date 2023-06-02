"""
"""
def dir(name, build, deps = [], exclude = []):
    native.genrule(
        name = name,
        srcs = [name + "_files"] + deps,
        outs = [name + ".dir.tar"],
        cmd = """
        NAME=%s
        for src in $(SRCS); do
            if [[ $$src == bazel-out/* && $$src == *.dir.tar ]]; then
                echo "Untarring file: $$src"
                tar -xf "$$src"
            fi
        done
        # Run the command:
        (cd $$NAME; %s)
        tar -cf $@ $$NAME
        """ % (name, cmd),
    )

    native.filegroup(
        name = name + "_files",
        srcs = native.glob([
            "**/*",
        ], exclude = exclude),
    )