use schemaschlopf::{run_assistant_export, run_take_bootstrap, seal_source_manifest};

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    let project_root = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("[SchemaSchlopf take] FEHLER: current directory unavailable: {error}");
            std::process::exit(1);
        }
    };

    if args.len() == 1 && args[0].to_str() == Some("--seal") {
        match seal_source_manifest(&project_root) {
            Ok(hash) => {
                println!("[SchemaSchlopf seal] GreenBase tree sealed");
                println!("[SchemaSchlopf seal] SOURCE_MANIFEST SHA-256: {hash}");
                return;
            }
            Err(error) => {
                eprintln!("[SchemaSchlopf seal] FEHLER: {error}");
                std::process::exit(1);
            }
        }
    }

    if args.len() == 1 && args[0].to_str() == Some("--assistant-export") {
        match run_assistant_export(&project_root) {
            Ok(summary) => {
                println!("[SchemaSchlopf assistant-export] GreenBase seal verified");
                println!(
                    "[SchemaSchlopf assistant-export] Sealed source files: {}",
                    summary.source_file_count
                );
                println!(
                    "[SchemaSchlopf assistant-export] SOURCE_MANIFEST SHA-256: {}",
                    summary.source_manifest_sha256
                );
                println!(
                    "[SchemaSchlopf assistant-export] ZIP: {}",
                    summary.zip_path.display()
                );
                println!(
                    "[SchemaSchlopf assistant-export] ZIP SHA-256: {}",
                    summary.zip_sha256
                );
                return;
            }
            Err(error) => {
                eprintln!("[SchemaSchlopf assistant-export] FEHLER: {error}");
                std::process::exit(1);
            }
        }
    }

    if !args.is_empty() {
        eprintln!("usage: schemaschlopf_take_bootstrap [--seal | --assistant-export]");
        std::process::exit(2);
    }

    if let Err(error) = run_take_bootstrap(&project_root) {
        eprintln!("[SchemaSchlopf take] FEHLER: {error}");
        std::process::exit(1);
    }
}
