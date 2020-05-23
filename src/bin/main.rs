use esbuild_rs::logging::MsgCounts;
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
}
