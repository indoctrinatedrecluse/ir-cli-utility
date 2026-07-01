use crate::EnvOptions;

pub fn env_action(var_name: Option<&str>, options: EnvOptions) {
    if let Some(target) = var_name {
        // Look up a specific variable case-insensitively
        let target_lower = target.to_lowercase();
        let found_var = std::env::vars().find(|(k, _)| k.to_lowercase() == target_lower);

        match found_var {
            Some((key, value)) => {
                if key.to_uppercase() == "PATH" {
                    // Split paths according to platform rules
                    for path in std::env::split_paths(&value) {
                        println!("{}", path.display());
                    }
                } else {
                    println!("{}", value);
                }
            }
            None => {
                eprintln!("Error: Environment variable '{}' not found.", target);
                std::process::exit(1);
            }
        }
    } else {
        // List all environment variables
        let mut vars: Vec<(String, String)> = std::env::vars().collect();
        
        // Filter if search query is active
        if let Some(ref query) = options.search {
            let query_lower = query.to_lowercase();
            vars.retain(|(k, v)| {
                k.to_lowercase().contains(&query_lower) || v.to_lowercase().contains(&query_lower)
            });
        }

        // Sort alphabetically by key name
        vars.sort_by(|a, b| a.0.cmp(&b.0));

        for (k, v) in vars {
            println!("{}={}", k, v);
        }
    }
}
