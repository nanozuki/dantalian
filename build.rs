use clap::IntoApp;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
include!("src/options.rs");

fn main() {
    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or_else(|| std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = Opts::command();
    app.set_bin_name("dantalian");
    generate_to(Zsh, &mut app, "dantalian", &outdir).expect("zsh completion generation failed.");
    generate_to(Bash, &mut app, "dantalian", &outdir).expect("bash completion generation failed.");
    generate_to(Fish, &mut app, "dantalian", &outdir).expect("fish completion genertaion failed.");
}
