use support::Dispatch;

mod balances;
mod proof_of_existance;
mod support;
mod system;

mod types {
    pub type AccountID = String;
    pub type Tokens = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;

    pub type Extrinsic = crate::support::Extrinsic<AccountID, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// configure our runtime

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existance: proof_of_existance::Pallet<Self>,
}
impl system::Config for Runtime {
    type AccountId = types::AccountID;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Tokens = types::Tokens;
}
impl proof_of_existance::Config for Runtime {
    type Content = &'static str;
}

// use runtime in main logic

const NAMES: [&str; 10] = [
    "Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie", "Grace", "Hank", "Ivy", "Judy",
];

fn main() {
    // initialize runtime
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"Alice".to_string(), 100);

    let mut idx = 1;
    loop {
        // define block content:
        // - alice sends 30 tokens to some account
        let to = NAMES[idx as usize % 10];
        let amount = 30;
        let mut block = types::Block {
            header: types::Header { block_number: idx },
            extrinsics: vec![types::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: to.to_string(),
                    amount,
                }),
            }],
        };

        if rand::random::<f32>() < 0.2 {
            block.extrinsics.push(types::Extrinsic {
                caller: NAMES[rand::random::<u32>() as usize % NAMES.len()].to_string(),
                call: RuntimeCall::proof_of_existance(proof_of_existance::Call::create_claim {
                    claim: "Hello, World!",
                }),
            });
        }

        // execute block
        runtime.execute_block(block).expect("invalid block");
        println!("{:#?}", runtime);
        std::thread::sleep(std::time::Duration::from_secs(1));
        idx += 1;
    }
}
