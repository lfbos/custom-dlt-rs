/// Generate default configuration files
/// 
/// This utility generates default configuration files that can be used as templates
/// for configuring the blockchain system.
///
/// Usage:
///   cargo run --bin config_gen [output_file]
///
/// Examples:
///   cargo run --bin config_gen                    # Generates config.default.json
///   cargo run --bin config_gen config.json        # Generates config.json
///   cargo run --bin config_gen config.testnet.json # Generate testnet config

use btclib::config::BlockchainConfig;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let output_file = args.get(1).map(|s| s.as_str()).unwrap_or("config.default.json");
    
    // Generate default configuration
    let config = BlockchainConfig::default();
    
    // Save to file
    match config.save_to_file(output_file) {
        Ok(_) => {
            eprintln!("✓ Generated default configuration:");
            eprintln!("  File: {}", output_file);
            eprintln!();
            eprintln!("To use this configuration:");
            eprintln!("  1. Copy it: cp {} config.json", output_file);
            eprintln!("  2. Edit config.json to customize settings");
            eprintln!("  3. Run your application (it will automatically load config.json)");
            eprintln!();
            eprintln!("Environment variables will still override config file values.");
        }
        Err(e) => {
            eprintln!("✗ Error generating config file: {}", e);
            std::process::exit(1);
        }
    }
}

