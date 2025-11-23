use crate::files::BUILT_IN_TEMPLATES;

pub fn built_ins() {
    println!("Built-in templates:");
    for (key, _) in BUILT_IN_TEMPLATES {
        println!("  - {}", key);
    }
}
