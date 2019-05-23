extern crate r2d2_lib;

mod utils;

use tempfile;
use tempfile::NamedTempFile;
use utils::*;

#[test]
fn commiting_operations() {
    setup();
}
