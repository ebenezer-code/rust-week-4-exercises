use thiserror::Error;

// Custom errors for Bitcoin operations
#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Invalid transaction format")]
    InvalidTransaction,
    #[error("Invalid script format")]
    InvalidScript,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
        // TODO: Implement constructor for Point
    }
}

// Custom serialization for Bitcoin transaction
pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8>;
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        LegacyTransactionBuilder::default()
        // TODO: Return a new builder for constructing a transaction
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        Self {
            version: 1,
            inputs: vec![],
            outputs: vec![],
            lock_time: 0,
        }
        // TODO: Implement default values
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        Self::default()
        // TODO: Initialize new builder by calling default
    }

    pub fn version(mut self, version: i32) -> Self {
        self.version = version;
        self
        // TODO: Set the transaction version
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        self.inputs.push(input);
        self
        // TODO: Add input to the transaction
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        self.outputs.push(output);
        self
        // TODO: Add output to the transaction
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        self.lock_time = lock_time;
        self
        // TODO: Set lock_time for transaction
    }

    pub fn build(self) -> LegacyTransaction {
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
        // TODO: Build and return the final LegacyTransaction
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    if args.is_empty() {
        return Err(BitcoinError::ParseError("No command provided".into()));
    }

    match args[0].as_str() {
        "send" => {
            if args.len() != 3 {
                return Err(BitcoinError::ParseError(
                    "Usage: send <amount> <address>".into(),
                ));
            }

            let amount = args[1]
                .parse::<u64>()
                .map_err(|_| BitcoinError::InvalidAmount)?;
            let address = args[2].clone();

            Ok(CliCommand::Send { amount, address })
        }
        "balance" => {
            if args.len() != 1 {
                return Err(BitcoinError::ParseError("Usage: balance".into()));
            }

            Ok(CliCommand::Balance)
        }
        _ => Err(BitcoinError::ParseError("Unknown command".into())),
    }
    // TODO: Match args to "send" or "balance" commands and parse required arguments
}

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            return Err(BitcoinError::InvalidTransaction);
        }

        let version = i32::from_le_bytes(data[0..4].try_into().unwrap());
        let input_count = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let output_count = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let lock_time = u32::from_le_bytes(data[12..16].try_into().unwrap());

        Ok(LegacyTransaction {
            version,
            inputs: Vec::with_capacity(input_count as usize),
            outputs: Vec::with_capacity(output_count as usize),
            lock_time,
        })
        // TODO: Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.version.to_le_bytes());
        bytes.extend(&self.lock_time.to_le_bytes());
        bytes
        // TODO: Serialize only version and lock_time (simplified)
    }
}
