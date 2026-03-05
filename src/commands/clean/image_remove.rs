use std::fs;
use std::path::Path;

use crate::Framework;

pub fn image_remover(root: &Path, framework: Framework) {
    match framework {
        Framework::React => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
            let _ = fs::remove_file(root.join("src/assets/react.svg"));
        }

        Framework::Vue => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
            let _ = fs::remove_file(root.join("src/assets/logo.svg"));
        }

        Framework::Svelte => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
            let _ = fs::remove_file(root.join("src/assets/svelte.svg"));
        }

        Framework::Lit => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
        }

        Framework::Vanilla => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
        }

        Framework::Preact
        | Framework::Solid
        | Framework::Qwik
        | Framework::Angular
        | Framework::Ember
        | Framework::Marko => {
            let _ = fs::remove_file(root.join("public/vite.svg"));
        }
    }
}
