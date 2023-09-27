use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	PdaaccountWithOneStaticSeedAndOneFieldTwo,
};

/// Test `rent-payer` using `init` with PDA account that has one static seed, one field and 4 signers
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` signer_1: [AccountInfo] 
/// 2. `[writable, signer]` signer_2: [AccountInfo] 
/// 3. `[signer]` signer_3: [AccountInfo] 
/// 4. `[writable]` account: [PdaaccountWithOneStaticSeedAndOneFieldTwo]
/// 5. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - input_1: [u8] 
pub fn instruction_22(
	program_id: &Pubkey,
	account: &mut AccountPDA<PdaaccountWithOneStaticSeedAndOneFieldTwo>,
	input_1: u8,
) -> ProgramResult {
	account.data.field_1 = input_1;

    Ok(())
}