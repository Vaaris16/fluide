use std::path::Path;

use crate::Framework;

pub mod image_remove;

pub fn setup_cleaner(root: &Path, framework: Framework) {
    image_remove::image_remover(root, framework);
}
