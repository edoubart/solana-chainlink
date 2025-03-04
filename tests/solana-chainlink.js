// NPM Packages
import { web3, AnchorProvider } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';

// Constants
const CHAINLINK_PRICE_FEED_SOL_USD_CONTRACT_ADDRESS =
  '99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR';
const CHAINLINK_PROGRAM_ID = 'HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny';

describe("solana-chainlink", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaChainlink;

  it("Retrieves SOL/USD price feed from Chainlink", async () => {
    const resultAccount = web3.Keypair.generate();

    await program.rpc.execute({
      accounts: {
        chainlinkFeed: CHAINLINK_PRICE_FEED_SOL_USD_CONTRACT_ADDRESS,
        chainlinkProgram: CHAINLINK_PROGRAM_ID,
        resultAccount: resultAccount.publicKey,
        systemProgram: web3.SystemProgram.programId,
        user: provider.wallet.publicKey,
      },
      signers: [ resultAccount ],
    });

    const result = await program.account.resultAccount
      .fetch(resultAccount.publicKey)

    const latestPrice = result.value / 100000000;

    console.log('latestPrice: ', latestPrice);
  });
});
