// Sol is required when making a new contract
// but there are many tokens other than SOL
// SPL tokens = everything that's not SOL

// SPL - Solana Program Library

// USDC, and other things
// when an account receives the tokens 
// they need somewhere to store these tokens since
// regular accounts can only store SOL

// an associated token account is just a PDA which takes things
// with seeds made from wallet address and token mint address

// like getting Bob's USDC account by looking up PDA based on his
// wallet address and USDC mint

// metadata - everything about token
// mint authority is the boss of the mint
// sign all the transactions to mint the tokens
// key pair for the mint authority

// `` solana-keygen grind --starts-with bos:1  `` --> this starts with bos
// (BOSS)

// `` solana config set --keypair bosBkAgy6YdwvMcrNNbnfzBVwJs6L1UQMVyYkaPXrX.json``

// `` solana config set --url devnet``

// but we need some SOL, we go to faucet.solana.com

// `` solana balance``
// `` solana address``


// `` solana-keygen grind --starts-with mnt:1``
// (MINT)

//  `` spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
// Token Extension Program ID - TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb


// OUTPUT
// Address:  FTzWSGyjB9Lpxqd7RLMsZSPGMpYdQ1xmbur53w8xp7Ho
// Decimals:  9

// Signature: 4vqcq41Gp2oNwhHPYRrJQWiTrSZodtvtt4KFHASHNEAAs3ozThcLLQ8P1t6dqpAtneW8KJsq6CTrDzdyQNgScbi

// We have Decentralized storage things arweave and other things
// like ipfs thing store just metadata


// ``spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb --enable-metadata

// OUTPUT --> 
// Address:  AuSqa3G1cXyUNcMh9LUTy8NkTViG3dti69ADgZxdLoEn
// Decimals:  9

// Signature: 3scfpzdJJfUBxNAHKbFnGtHVqQRua5jjcTN93M3o3aM6tPpLbMzSYaW1uNcV6WUmYpCuLjJnubvxS925RDUntxxf


// ``spl-token initialize-metadata AuSqa3G1cXyUNcMh9LUTy8NkTViG3dti69ADgZxdLoEn 'Example' 'EX' https://ipfs.io/ipfs/QmZcH4YvBVVRJtdn4RdbaqgspFU8gH6P9vomDpBVpAL3u4/6722``


// Done so what we have now
// token mint
// so we can use this token mint to mint tokens and as we are the authority
// we can mint it to anyone's account


// So we are going to create pda or associated token account
// ``spl-token create-account AuSqa3G1cXyUNcMh9LUTy8NkTViG3dti69ADgZxdLoEn``

// OUTPUT
// Creating account FfdKYRTyXnovTvFC7ZdNvZEpLBmVN26H9ns7t6Q6SDRy
// Signature: 3CBdJrF38NnJtfHmi8wcZDQojmG22Fhp5W15eUxEAQDzk811FpD1ggDMWWSA4322rfLTxytpw2RJVdrmUugkUbRB


// ``spl-token mint AuSqa3G1cXyUNcMh9LUTy8NkTViG3dti69ADgZxdLoEn 1000``
// Minting 1000 tokens
//   Token: AuSqa3G1cXyUNcMh9LUTy8NkTViG3dti69ADgZxdLoEn
//   Recipient: FfdKYRTyXnovTvFC7ZdNvZEpLBmVN26H9ns7t6Q6SDRy

// Signature: 4QEtzomYkVHmZubzgqtTgA5YkaJTSHw6kRZGfnjwsNWByvsv4MHXn7wbsvraFwW62u61KTQN77nE7QFZyA8SZL1r

// check solana explorer 1000 tokens will be there


// we can transfer burn do anything