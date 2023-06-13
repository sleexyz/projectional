# rules_hybrid

> { Fast, Correct, Easy } -- Choose three? Two and a half?

rules_hybrid is an experiment that tries to answer the following question:

- How easy can we make it to use Bazel if we throw hermiticity out the window?

What guarantees can we still get, even if weakened? How useful is Bazel with these weakened guarantees, if at all?

## Design

What does rules_hybrid do differently?
1. **Persistent build workspaces**: This lets us leverage *incremental compilation* capabilitites of existing toolchains for free.
    1. **Embrace coarse targets**: Because we have incremental compilation, we can define much coarser targets without loss of build speed. Throw all your sources into a glob and call it a day.
1. **No bazelifying toolchains**: Instead, we can lean on *Nixpkgs* to reproducibly provide toolchains.


What do we lose?
1. Isolation of builds across build actions and test invocations.
1. (Much more..?)


What do we still have?
1. Reproducibility post-`bazel clean`

What are we left with?
- Bazel as an **advanced task runner** for **local development** that supports 
    - Parallel builds.
    - Artifact caching
    - Cache invalidation when sources change
    - All of the above when developing interactively via [`ibazel`](https://github.com/bazelbuild/bazel-watcher)

Best practices with persistent build workspaces:

- Build steps should be _commutative_.
  - Building `:a` then `:b` should be functionally equivalent to building `:b` then `:a`.

## FAQ

### How does this compare to [rules_nixpkgs](https://github.com/tweag/rules_nixpkgs)?
rules_nixpkgs uses Nixpkgs to provide toolchains and dependencies to Bazel but in a way that's hermetic from Bazel's POV.

rules_hybrid by default simply lets your PATH bleed into Bazel.

That being said, one can imagine shoring up the hermeticity step by step, of which one step could be plugging the PATH hole and switching to rules_nixpkgs.


## Unanswered questions

1. What about *immutable*, persistent build workspaces? Any easy way to get this?
1. Can we make non-commutative build operations effectively commutative by defining an inverse? Aka, generate a diff per step, and then undo the diff.
1. How does rules_hybrid work with remote build caching?
1. How does rules_hybrid interact with a codebase with traditional bazel rules?
    1. What role can rules_hybrid play for a migration of a codebase to bazel?
1. What if the build workspace was exactly just your development workspace? What could this unlock?