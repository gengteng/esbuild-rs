use esbuild_rs::ast::{join_all_with_comma, Expr, ExprKind};
use esbuild_rs::logging::MsgCounts;
use std::collections::HashSet;
use std::fmt::Debug;

trait DebugPrintln: Debug {
    fn debug_println(&self) {
        println!("{:?}", self);
    }
}

impl<T: Debug> DebugPrintln for T {}

fn main() {
    let msg_count = MsgCounts {
        errors: 0,
        warnings: 0,
    };

    println!("{}", msg_count);

    if let Some((width, height)) = terminal_size::terminal_size() {
        println!("{} {}", width.0, height.0);
    } else {
        println!("no terminal size");
    }

    if let Some(expr) = {
        let mut v = Vec::<Expr>::new();
        v.push(Expr {
            location: 0,
            data: Box::new(ExprKind::Null),
        });
        v.push(Expr {
            location: 1,
            data: Box::new(ExprKind::String { value: vec![1, 2] }),
        });
        // v.push(Expr {
        //     location: 1,
        //     data: Box::new(ExprKind::String { value: vec![1, 2] }),
        // });
        // v.push(Expr {
        //     location: 1,
        //     data: Box::new(ExprKind::String { value: vec![1, 2] }),
        // });
        //

        join_all_with_comma(v.into_iter())
    } {
        println!("{:?}", expr);
    } else {
        println!("no expr");
    }
}
