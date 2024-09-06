use zed_extension_api as zed;

struct CfmlExtension {
    // ... state
}

impl zed::Extension for CfmlExtension {
    fn new() -> Self {
        Self {
        }
    }
}

zed::register_extension!(CfmlExtension);