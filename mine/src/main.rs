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
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    poe: proof_of_existance::Pallet<Self>,
}
impl system::Config for Runtime {
    type AccountID = types::AccountID;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Tokens = types::Tokens;
}
impl proof_of_existance::Config for Runtime {
    type Content = &'static str;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existance::Call<Runtime>),
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            poe: proof_of_existance::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        // increment block number
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        // all extrinsics in block
        for (i, types::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // increment caller's nonce & dispatch extrinsic call
            // consumes error with `eprint`
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountID;
    type Call = RuntimeCall;
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.poe.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
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
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: to.to_string(),
                    amount,
                }),
            }],
        };

        if rand::random::<f32>() < 0.2 {
            block.extrinsics.push(types::Extrinsic {
                caller: NAMES[rand::random::<u32>() as usize % NAMES.len()].to_string(),
                call: RuntimeCall::ProofOfExistence(proof_of_existance::Call::CreateClaim("hello")),
            });
        }

        // execute block
        runtime.execute_block(block).expect("invalid block");
        println!("{:#?}", runtime);
        std::thread::sleep(std::time::Duration::from_secs(1));
        idx += 1;
    }
}
