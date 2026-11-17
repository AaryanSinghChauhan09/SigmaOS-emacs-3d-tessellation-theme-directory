//! Integration tests for SigmaOS-emacs-3d-tessellation-theme-directory.

use sigmaos_emacs_3d_tessellation_theme_directory_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
