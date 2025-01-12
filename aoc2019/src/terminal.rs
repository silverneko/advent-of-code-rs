use std::io::Write;
use std::sync::LazyLock;
use terminfo::{capability, Database};

static INFO: LazyLock<Database> = LazyLock::new(|| Database::from_env().unwrap());

pub fn clear(mut term: impl Write) {
    INFO.get::<capability::ClearScreen>().unwrap().expand().to(&mut term).unwrap();
}

pub fn home(mut term: impl Write) {
    INFO.get::<capability::CursorHome>().unwrap().expand().to(&mut term).unwrap();
}
