extern crate sass_rs;
extern crate sha2;

use std::fs;
use std::sync::{Arc, Mutex};
use sha2::{Digest, Sha256};

pub struct ScssCompiler {
    scss_path: String,
    css_path: String,
    last_hash: Arc<Mutex<Option<String>>>,
}

impl ScssCompiler {
    pub fn new(scss_path: &str, css_path: &str) -> Self {
        ScssCompiler {
            scss_path: scss_path.to_string(),
            css_path: css_path.to_string(),
            last_hash: Arc::new(Mutex::new(None)),
        }
    }

    pub fn compile_if_needed(&self) {
        let scss_content = fs::read_to_string(&self.scss_path).expect("Unable to read SCSS file");
        let current_hash = self.calculate_hash(&scss_content);

        let mut last_hash = self.last_hash.lock().unwrap();
        if last_hash.as_ref() != Some(&current_hash) {
            let css_content = sass_rs::compile_string(&scss_content, sass_rs::Options {
                output_style: sass_rs::OutputStyle::Compressed,
                ..Default::default()
            })
                .expect("Failed to compile SCSS");
            fs::write(&self.css_path, css_content).expect("Unable to write CSS file");
            *last_hash = Some(current_hash);
        } else {
        }
    }

    fn calculate_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn clone(&self) -> Self {
        ScssCompiler::new(&self.scss_path, &self.css_path)
    }
}