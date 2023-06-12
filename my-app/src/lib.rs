use ::puddlejumper::puddlejumper::node::{Context, NodeId};
use puddlejumper::puddlejumper;
use std::io::{Error, ErrorKind};
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let Some((context, foo)) = parse("Hello\n\tP1\n\t\thullo") else { return Ok(()) };

    let mut out = Vec::new();
    context.pretty_print(
        foo,
        &mut puddlejumper::node::printer::PrintContext {
            level: 0,
            needs_indent: true,
            out: &mut out,
        },
    );
    unsafe { console::log_1(&JsValue::from_str(&String::from_utf8(out).unwrap())) };
    Ok(())
}

fn parse(code: &str) -> Option<(Context, NodeId)> {
    let p = puddlejumper::parser::Parser::new(code.into());
    p.load_document()
}

fn print_prioritized(code: &str) -> Option<String> {
    let p = puddlejumper::parser::Parser::new(code.into());
    let mut output = Vec::new();
    p.load_document()
        .ok_or(Error::new(ErrorKind::Other, "Error parsing file"))
        .and_then(|(mut ctx, node)| {
            let list = ctx.make_prioritized_list(node);
            return ctx.pretty_print(
                list,
                &mut puddlejumper::node::printer::PrintContext {
                    level: 0,
                    needs_indent: true,
                    out: &mut output,
                },
            );
        })
        .ok()
        .and_then(|_| Some(String::from_utf8(output).unwrap()))
}
