use std::io;

use testutils::os_cmd::{RunnableCommand, presets::CargoDoc};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default().run()
}
