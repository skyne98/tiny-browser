use anyhow::Result;
use js::V8Wrapper;

mod js;

fn main() -> Result<()> {
    let mut wrapper = V8Wrapper::new();

    match wrapper.execute_script("'Hello' + ' World!'") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }

    let wasm_source = r#"
        let bytes = new Uint8Array([
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01,
            0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07,
            0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01,
            0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b
        ]);
        let module = new WebAssembly.Module(bytes);
        let instance = new WebAssembly.Instance(module);
        instance.exports.add(3, 4);
    "#;

    match wrapper.execute_script(wasm_source) {
        Ok(result) => println!("3 + 4 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    wrapper.execute_script("variable = 10")?;
    match wrapper.execute_script("variable") {
        Ok(result) => println!("variable = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // let linkedom_url =
    //     "https://raw.githubusercontent.com/WebReflection/linkedom/refs/heads/main/worker.js";
    // let linkedom_source = reqwest::blocking::get(linkedom_url)?.text()?;
    let linkedom_source = include_str!("../assets/linkedom.js");
    (&mut wrapper).execute_script(linkedom_source)?;

    // Try to read the title of the page
    let content = wrapper.execute_script(
        "(linkedom.parseHTML('<html><body><span>Test</span></body></html>')).document.body.textContent",
    )?;
    println!("Content: {}", content);

    // Try to load https://www.rust-lang.org
    let rust_lang_url = "https://www.rust-lang.org";
    let rust_lang_source = reqwest::blocking::get(rust_lang_url)?.text()?;
    let rust_lang_title = (&mut wrapper).execute_script(&format!(
        "(function () {{ let html = linkedom.parseHTML(`{}`); globalThis.document = html.document; globalThis.window = html.window; return document.title; }})()",
        rust_lang_source
    ))?;
    println!("Rust Title: {}", rust_lang_title.trim());

    // Get a list of all script tags
    let script_tags = wrapper.execute_script(
        "(function () { let scripts = document.querySelectorAll('script'); return Array.from(scripts).map(script => script.src); })()",
    )?;
    println!("Scripts: {:?}", script_tags);

    // Execute all scripts one by one
    for script in script_tags.split(',') {
        if script.is_empty() {
            continue;
        }

        let url = format!("https://www.rust-lang.org{}", script.trim());

        let source = reqwest::blocking::get(&url)?.text()?;
        (&mut wrapper).execute_script(&source)?;

        println!("Loaded: {}", url);

        // Execute the script
        match wrapper.execute_script(&source) {
            Ok(_) => println!("Executed: {}", url),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
