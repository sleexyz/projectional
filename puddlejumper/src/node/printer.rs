use super::*;

pub struct PrintContext<'out> {
    pub level: usize,
    pub out: &'out mut dyn std::io::Write,
    pub needs_indent: bool,
}

impl <'out> PrintContext<'out> {
    pub fn new(out: &'out mut dyn std::io::Write) -> Self {
        Self {
            level: 0,
            out,
            needs_indent: false,
        }
    }
}

impl PrintContext<'_> {
    pub fn pretty_print<'a>(
        &mut self,
        node: NodeId,
        ctx: &'a Context,
    ) -> Result<(), std::io::Error> {
        match &ctx.arena[node] {
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
                for _ in 0..self.level {
                    indent.push_str("    ");
                }
                write!(self.out, "{}", indent)?;
                match binding {
                    Some(binding) => {
                        write!(self.out, "@{}:", binding)?;
                    }
                    None => (),
                }
                match content {
                    Some(Content::Content(content)) => {
                        writeln!(self.out, "{}", content)?;
                    }
                    Some(Content::Ref(content)) => {
                        writeln!(self.out, "{}", content)?;
                    }
                    None => (),
                }
                {
                    let mut pc = PrintContext {
                        level: self.level + 1,
                        out: self.out,
                        needs_indent: true,
                    };
                    for child in children {
                        pc.pretty_print( *child, ctx)?;
                    }
                }
            }
            Node::Block {
                header,
                binding,
                children,
                ..
            } => {
                let mut indent = String::new();
                for _ in 0..self.level {
                    indent.push_str("  ");
                }
                match binding {
                    Some(binding) => {
                        writeln!(self.out, "{}{}:", indent, binding)?;
                    }
                    None => (),
                }
                {
                    let mut pc = PrintContext {
                        level: self.level,
                        out: self.out,
                        needs_indent: false,
                    };
                    pc.needs_indent = false;
                    pc.pretty_print(*header, ctx)?;
                }
                {
                    let mut pc = PrintContext {
                        level: self.level + 1,
                        out: self.out,
                        needs_indent: true,
                    };
                    for child in children {
                        pc.pretty_print( *child, ctx)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn fmt(node: NodeId, context: &Context) -> String {
        let mut out = Vec::new();
        let mut pc = PrintContext::new(&mut out);
        pc.pretty_print(node, context).unwrap();
        String::from_utf8(out).unwrap()
    }
}