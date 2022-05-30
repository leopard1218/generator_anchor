use anchor_lang::prelude::*;
// use anyhow::Result;
// use std::time::{SystemTime, UNIX_EPOCH};
// use secp256k1::{rand::rngs, PublicKey, SecretKey};
// use tiny_keccak::keccak256;
// use web3::{
//     types::{Address},
// };
declare_id!("EsoBphphHjTKPwHLEGiv9zHgiXu1oDb2dYo2nVrabhda");



// pub fn get_nstime() -> u64 {
//     let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     dur.as_secs() << 30 | dur.subsec_nanos() as u64
// }

// pub fn generate_keypair() -> (SecretKey, PublicKey) {
//     let secp = secp256k1::Secp256k1::new();
//     let mut rng = rngs::JitterRng::new_with_timer(get_nstime);
//     secp.generate_keypair(&mut rng)
// }

// pub fn public_key_address(public_key: &PublicKey) -> Address {
//     let public_key = public_key.serialize_uncompressed();
//     debug_assert_eq!(public_key[0], 0x04);
//     let hash = keccak256(&public_key[1..]);

//     Address::from_slice(&hash[12..])
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let (secret_key, pub_key) = generate_keypair();
//     println!("secret key: {}", &secret_key.to_string());
//     let pub_address = public_key_address(&pub_key);
//     println!("public address: {:?}", pub_address);
//     Ok(())
// }




#[program]
mod generator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    pub data_list: Vec<String>,
}