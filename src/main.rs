use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[arg(help = "Handlebars template file")]
    template: String,

    #[arg(help = "json that will provide information for the template")]
    source: String,
}

fn main() -> anyhow::Result<()> {
    if let Err(e) = work() {
        println!("Error: {e}")
    };
    Ok(())
}

fn work() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let template = fs::read_to_string(opts.template)?;

    let source = fs::read_to_string(opts.source)?;

    let source_value: serde_json::Value = serde_json::from_str(&source)?;

    let mut tmpl = handlebars::Handlebars::new();
    tmpl.register_escape_fn(|s| s.into());

    println!("{}", tmpl.render_template(&template, &source_value)?);

    Ok(())
}
