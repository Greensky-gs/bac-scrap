mod files;
mod tools;
mod result;

use crate::files::Metadata;

fn main() {
    let _ = std::fs::create_dir("data");
    let mut meta = Metadata::new(&"data/errors.txt".to_string(), &"data/results.json".to_string(), &"data/indexes".to_string());

    meta.start_fetch();    
}
