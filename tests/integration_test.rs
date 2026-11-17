// Integration test for SigmaOS-alpm-python
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_alpm_python as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
