#[cfg(windows)]
extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("src/umamusume_open.rc").unwrap();
}
