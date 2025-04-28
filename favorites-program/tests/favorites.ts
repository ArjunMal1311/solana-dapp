import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { assert } from "chai";

describe("favorites", () => {
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider)

  const user = (provider.wallet as anchor.Wallet).payer
  const someRandomGuy = anchor.web3.Keypair.generate()

  console.log("User:", user.publicKey.toBase58())
  console.log("Some random guy:", someRandomGuy.publicKey.toBase58())

  const program = anchor.workspace.Favorites as Program<Favorites>
  console.log("Program:", program.programId.toBase58())



  const favNumber = new anchor.BN(23);
  const favColor = "Red";
  const favHobbies = ["Coding", "Gaming", "Reading"];


  // no need to airdrop local shit, it gives 85 billion dollars worth of SOL
  // for deployment of test net we need to use faucet

  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey)
    console.log("User balance:", balance)


    const balanceInSOL = balance / anchor.web3.LAMPORTS_PER_SOL
    console.log("User balance in SOL:", balanceInSOL)


    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL)
    console.log("User balance in SOL (formatted):", formattedBalance)
  })

  it("Sample test", async () => {
    console.log("Running sample test")
  })

  it("Set favorites", async () => {

    // we can get methods from program.methods
    // sign the transaction by user
    // send transaction to cluser or RPC
    await program.methods.setFavorites(favNumber, favColor, favHobbies).signers([user]).rpc()


    // finding the PDA for user's fav account
    const favPDAandBump = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('favorites'), user.publicKey.toBuffer()], program.programId);
    console.log("Fav PDA:", favPDAandBump[0].toBase58())
    console.log("Fav Bump:", favPDAandBump[1])


    // Getting data from PDA
    const dataFromPDA = await program.account.favorites.fetch(favPDAandBump[0])


    console.log("Data from PDA:", dataFromPDA)

    assert.equal(dataFromPDA.color, favColor);
    assert.equal(dataFromPDA.number.toString(), favNumber.toString());
    assert.deepEqual(dataFromPDA.hobbies, favHobbies);
  })

  it('Updates the favorites', async () => {
    const newFavoriteHobbies = ['skiing', 'skydiving', 'biking', 'swimming'];
    try {
      await program.methods.setFavorites(favNumber, favColor, newFavoriteHobbies).signers([user]).rpc();
    } catch (error) {
      console.error((error as Error).message);
    }
  });

  it('Rejects transactions from unauthorized signers', async () => {
    try {
      // set_favourites in Rust becomes setFavorites in TypeScript
      // Sign the transaction
      // Send the transaction to the cluster or RPC
      await program.methods.setFavorites(favNumber, favColor, favHobbies).signers([someRandomGuy]).rpc();
    } catch (error) {
      const errorMessage = (error as Error).message;
      assert.isTrue(errorMessage.includes('unknown signer'));
    }
  });

});


// Deploying it to local & devnet
// anchor deploy ---> will deploy this thing to local
// anchor deploy --provider.cluster devnet ---> will deploy this thing to devnet, you can see tx on explorer.solana.com

// solana config get -> to check current cluster


// for testing on local & devnet
// anchor test --provider.cluster testnet --> testing things on devnet
// anchor test --> testing things on local