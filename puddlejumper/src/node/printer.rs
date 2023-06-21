use super::*;

pub struct PrintContext<'out> {
    pub level: usize,
    pub out: &'out mut dyn std::io::Write,
    pub needs_indent: bool,
}
impl Context {
    pub fn pretty_print<'a>(
        &'a self,
        node: NodeId,
        ctx: &mut PrintContext,
    ) -> Result<(), std::io::Error> {
        match &self.arena[node] {
            Node::Document { children, .. } => {
                for child in children {
                    self.pretty_print(*child, ctx)?;
                }
            }
            Node::Node {
                content,
                binding,
                children,
                ..
            } => {
                let mut indent = String::new();
                for _ in 0..ctx.level {
                    indent.push_str("    ");
                }
                write!(ctx.out, "{}", indent)?;
                match binding {
                    Some(binding) => {
                        write!(ctx.out, "@{}:", binding)?;
                    }
                    None => (),
                }
                match content {
                    Some(Content::Content(content)) => {
                        writeln!(ctx.out, "{}", content)?;
                    }
                    Some(Content::Ref(content)) => {
                        writeln!(ctx.out, "{}", content)?;
                    }
                    None => (),
                }
                for child in children {
                    self.pretty_print(
                        *child,
                        &mut PrintContext {
                            level: ctx.level + 1,
                            out: ctx.out,
                            needs_indent: true,
                        },
                    )?;
                }
            }
            Node::Block {
                header,
                binding,
                children,
                ..
            } => {
                let mut indent = String::new();
                for _ in 0..ctx.level {
                    indent.push_str("  ");
                }
                match binding {
                    Some(binding) => {
                        writeln!(ctx.out, "{}{}:", indent, binding)?;
                    }
                    None => (),
                }
                self.pretty_print(
                    *header,
                    &mut PrintContext {
                        level: ctx.level,
                        out: ctx.out,
                        needs_indent: false,
                    },
                )?;
                for child in children {
                    self.pretty_print(
                        *child,
                        &mut PrintContext {
                            level: ctx.level + 1,
                            out: ctx.out,
                            needs_indent: true,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }


}