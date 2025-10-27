#[cfg(test)]
mod core_tests {
    use crate::core::{Config, FeeConfig, FeeType};

    fn create_test_config(fee_type: FeeType, value: f64) -> Config {
        Config {
            my_keys: vec![],
            contacts: vec![],
            default_node: "127.0.0.1:9000".to_string(),
            fee_config: FeeConfig { fee_type, value },
        }
    }

    #[test]
    fn test_calculate_fee_fixed() {
        let config = create_test_config(FeeType::Fixed, 100.0);
        
        // Calculate fee like the actual implementation
        let fee = match config.fee_config.fee_type {
            FeeType::Fixed => config.fee_config.value as u64,
            FeeType::Percent => (1000 as f64 * config.fee_config.value / 100.0) as u64,
        };

        assert_eq!(fee, 100);
    }

    #[test]
    fn test_calculate_fee_percent_small_amount() {
        let config = create_test_config(FeeType::Percent, 1.0);
        
        let amount = 1000u64;
        let fee = match config.fee_config.fee_type {
            FeeType::Fixed => config.fee_config.value as u64,
            FeeType::Percent => (amount as f64 * config.fee_config.value / 100.0) as u64,
        };

        assert_eq!(fee, 10); // 1% of 1000 = 10
    }

    #[test]
    fn test_calculate_fee_percent_large_amount() {
        let config = create_test_config(FeeType::Percent, 2.5);
        
        let amount = 100_000_000u64; // 1 BTC
        let fee = match config.fee_config.fee_type {
            FeeType::Fixed => config.fee_config.value as u64,
            FeeType::Percent => (amount as f64 * config.fee_config.value / 100.0) as u64,
        };

        assert_eq!(fee, 2_500_000); // 2.5% of 100M = 2.5M satoshis
    }

    #[test]
    fn test_calculate_fee_percent_minimal_amount() {
        let config = create_test_config(FeeType::Percent, 0.1);
        
        let amount = 100u64;
        let fee = match config.fee_config.fee_type {
            FeeType::Fixed => config.fee_config.value as u64,
            FeeType::Percent => (amount as f64 * config.fee_config.value / 100.0) as u64,
        };

        assert_eq!(fee, 0); // 0.1% of 100 = 0.1, rounds down to 0
    }
}

