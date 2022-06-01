use klask::Settings;

fn load_font() -> Option<Vec<u8>> {
    let fc = fontconfig::Fontconfig::new()?;
    let font = fc.find("sans", None)?;
    let path = font.path;
    std::fs::read(path).ok()
}

fn main() {
    let setting = if let Some(font) = load_font() {
        Settings {
            custom_font: Some(font.leak()),
            ..Settings::default()
        }
    } else {
        Settings::default()
    };
    klask::run_derived(setting, |opts| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let r = runtime.block_on(async move { dantalian::run(opts).await });
        if let Err(e) = r {
            eprintln!("failed: {e}");
        }
    })
}
