pub mod grid;
pub mod intcode;

pub use grid::*;
pub use intcode::*;

use std::io::Write;
use std::sync::LazyLock;
use terminfo::{capability, Database};

static INFO: LazyLock<Database> = LazyLock::new(|| Database::from_env().unwrap());

pub fn move_up_and_clear_lines(mut term: impl Write, n: u32) {
    if n != 0 {
        INFO.get::<capability::ParmUpCursor>().unwrap().expand().count(n).to(&mut term).unwrap();
        INFO.get::<capability::ParmDeleteLine>().unwrap().expand().count(n).to(term).unwrap();
    }
}
