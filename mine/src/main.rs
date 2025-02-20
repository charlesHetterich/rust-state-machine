mod balances;
mod system;

pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    fn new() -> Self {
        let runtime = Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        };

        runtime
    }
}

const NAMES: [&str; 10] = [
    "Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie", "Grace", "Hank", "Ivy", "Judy",
];

fn main() {
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"Alice".to_string(), 100);

    let mut idx = 1;
    loop {
        runtime.system.inc_block_number();
        assert_eq!(runtime.system.block_number(), idx);

        // alice sends 30 tokens to some account
        runtime.system.inc_nonce(&"Alice".to_string());
        let to = NAMES[idx as usize % 10];
        let amount = 30;
        let _ = runtime
            .balances
            .transfer(&"Alice".to_string(), &to.to_string(), amount)
            .map_err(|e| eprintln!("{}", e));
        println!(
            "Block: {}, Alice: {}, {to}: {}",
            runtime.system.block_number(),
            runtime.balances.get_balance(&"Alice".to_string()),
            runtime.balances.get_balance(&to.to_string())
        );

        // sleep 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
        idx += 1;
    }
}
