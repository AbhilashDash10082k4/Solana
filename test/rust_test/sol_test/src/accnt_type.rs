use solana_sdk::pubkey::Pubkey;
use solana_sdk::epoch_schedule::Epoch;
pub struct Account {
    pub lamports: u64, //balance of an accnt
    pub executable: bool, //only smart contracts (program accnts) are executable
    pub owner: Pubkey, //this is the address of the program accnt, UNIQUE 32 bit base58 encoded string, this is an ED25519 pub key, this is also called as Program ID of the program that owns this accnt
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub data: Vec<u8>, //program code or data of the program or the address of the accnt containing th edata of the current accnt
    pub rent_epch: Epoch, //rent given by the accnt to store data and is ~ amnt of data stored, this is also the minimum balance maintained by the accnt - this is deprecated
    
}