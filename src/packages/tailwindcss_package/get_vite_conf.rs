use super::TailwindcssErrors;
use std::env;
use std::{fs, path::PathBuf};

pub fn get_vite_conf() -> Result<PathBuf, TailwindcssErrors> {
    let cd = env::current_dir().map_err(|_| TailwindcssErrors::CurrentDirFailed)?;

    let vite_conf_file = ["vite.config.ts", "vite.config.js", "vite.config.mjs"];

    for file in vite_conf_file {
        let path = cd.join(file);
        if path.exists() {
            print!("found vite config\n\n");
            return Ok(path);
        }
    }

    Err(TailwindcssErrors::NotFoundViteConfig)
}

pub fn add_tailwindcss_vite_conf(path: &PathBuf) -> Result<(), TailwindcssErrors> {
    let mut content =
        fs::read_to_string(path).map_err(|_| TailwindcssErrors::ReadViteConfigFailed)?;

    if !content.contains("import tailwindcss from '@tailwindcss/vite'") {
        content = format!("import tailwindcss from '@tailwindcss/vite'\n{}", content)
    }

    if !content.contains("plugins:[") && !content.contains("tailwindcss()") {
        content = content.replace("plugins: [", "plugins: [tailwindcss(), ")
    }

    let _ = fs::write(path, content).map_err(|_| TailwindcssErrors::EditViteConfigFailed);

    Ok(())
}
