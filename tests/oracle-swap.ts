import * as anchor from "@coral-xyz/anchor";
import { assert } from 'chai';
import { OracleSwap } from "../target/types/oracle_swap";
import * as solanaWeb3 from "@solana/web3.js";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { Transaction } from "@solana/web3.js";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
const BN = require('bn.js');

describe("oracle-swap", () => {
  
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const connection = provider.connection;

  const program = anchor.workspace.OracleSwap as anchor.Program<OracleSwap>;

  const admin = new Keypair();
  const user = new Keypair();
  const wallet = new anchor.Wallet(user);

  const [swapMetadata, _] = PublicKey.findProgramAddressSync([Buffer.from('swap_metadata')], program.programId)
  
  let mint: PublicKey;
  let priceFeedIdIncoming: Buffer<ArrayBuffer>;
  let taUser: PublicKey;

  before("setup", async () => {

    await airdropToAccount(connection, user.publicKey, solanaWeb3.LAMPORTS_PER_SOL)
    await airdropToAccount(connection, admin.publicKey, solanaWeb3.LAMPORTS_PER_SOL)

    mint = await splToken.createMint(
      connection,
      user,
      user.publicKey,
      null,
      9, 
      undefined,
      {}, 
      splToken.TOKEN_PROGRAM_ID
    );
    
    const taProgram = await getAssociatedTokenAddress(mint, swapMetadata, true, program.programId, ASSOCIATED_TOKEN_PROGRAM_ID)

    taUser = (await splToken.getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      user.publicKey,
    )).address;
  
    await splToken.mintTo(
      connection,
      user,
      mint,
      taUser,
      user,
      1000 * solanaWeb3.LAMPORTS_PER_SOL,
    );

    priceFeedIdIncoming = Buffer.from("eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a", "hex");
    const tx = await program.methods
      .initialize({
        discountBps: 200,
        feedIdIncoming: [...priceFeedIdIncoming], 
      })
      .accounts({
        admin: admin.publicKey,
        mintIncoming: mint,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    await connection.confirmTransaction(tx);

    const accountInfo = await connection.getAccountInfo(swapMetadata);

    assert(accountInfo.owner.equals(program.programId));
  });

  it("Can Swap!", async () => {
    const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
    const priceUpdateIncoming = pythSolanaReceiver.getPriceFeedAccountAddress(0, priceFeedIdIncoming);
    const priceUpdateSol = pythSolanaReceiver.getPriceFeedAccountAddress(0, "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d");
    const sigSwap = await program.methods
      .swap({
        amountIncoming: new anchor.BN(50)
      })
      .accounts({
        swapper: user.publicKey,
        taSwapper: taUser,
        priceUpdateIncoming, 
        priceUpdateSol,
        mintIncoming: mint,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
    await connection.confirmTransaction(sigSwap);
  })
});


async function airdropToAccount(connection: anchor.web3.Connection, account: PublicKey, amount: number) {
  const sigAirdropAccount = await connection.requestAirdrop(account, amount);  
  await connection.confirmTransaction(sigAirdropAccount);
};