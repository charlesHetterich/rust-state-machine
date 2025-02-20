mod balances;
mod system;

mod types {
    pub type AccountID = String;
    pub type Tokens = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

// configure our runtime

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}
impl system::Config for Runtime {
    type AccountID = types::AccountID;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Tokens = types::Tokens;
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

// use runtime in main logic

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

        // sleep 1 second
        println!("{:#?}", runtime);
        std::thread::sleep(std::time::Duration::from_secs(1));
        idx += 1;
    }
}
