import {
  web3,
  utils,
  BN,
  Spl,
  Provider,
  Program,
  workspace,
  setProvider,
} from '@project-serum/anchor'
import { set } from '@project-serum/anchor/dist/cjs/utils/features'

import { EVoting } from '../target/types/e_voting'
import { initializeAccount, initializeMint } from './pretest'

describe('e-voting', () => {
  // Configure the client to use the local cluster.
  const provider = Provider.env()
  setProvider(provider)

  const program = workspace.EVoting as Program<EVoting>

  // Context
  const spl = Spl.token()
  const proposal = new web3.Keypair()
  let proposalTokenAccount: web3.PublicKey
  let treasurer: web3.PublicKey
  const mint = new web3.Keypair()
  let walletTokenAccount: web3.PublicKey
  let receipt: web3.PublicKey

  before(async () => {
    // Init a mint
    await initializeMint(9, mint, provider)
    // Derive treasurer account
    const [treasurerPublicKey] = await web3.PublicKey.findProgramAddress(
      [Buffer.from('treasurer'), proposal.publicKey.toBuffer()],
      program.programId,
    )
    treasurer = treasurerPublicKey
    const [receiptPublicKey] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from('receipt'),
        proposal.publicKey.toBuffer(),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    )
    receipt = receiptPublicKey

    // Derive token account
    walletTokenAccount = await utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: provider.wallet.publicKey,
    })
    proposalTokenAccount = await utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: treasurerPublicKey,
    })

    // Create Token account + Mint to token
    await initializeAccount(
      walletTokenAccount,
      mint.publicKey,
      provider.wallet.publicKey,
      provider,
    )
    await spl.rpc.mintTo(new BN(1_000_000_000_000), {
      accounts: {
        mint: mint.publicKey,
        to: walletTokenAccount,
        authority: provider.wallet.publicKey,
      },
    })
  })

  it('Is initialized!', async () => {
    // Add your test here.
    const now = Math.round(new Date().getTime() / 1000)
    const tx = await program.rpc.initialize(new BN(0), new BN(now + 10), {
      accounts: {
        authority: program.provider.wallet.publicKey,
        proposal: proposal.publicKey,
        treasurer,
        mint: mint.publicKey,
        proposalTokenAccount: proposalTokenAccount,
        // System Program Address
        tokenProgram: utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [proposal],
    })
  })

  it('Is Voted!', async () => {
    // Add your test here.
    const tx = await program.rpc.vote(new BN(1), new BN(0), {
      accounts: {
        authority: program.provider.wallet.publicKey,
        proposal: proposal.publicKey,
        treasurer,
        mint: mint.publicKey,
        proposalTokenAccount: proposalTokenAccount,
        // Wallet accounts
        receipt,
        walletTokenAccount,
        // System Program Address
        tokenProgram: utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [],
    })
  })
  it('Is Fetched Proposal!', async () => {
    const proposalData = await program.account.proposal.fetch(
      proposal.publicKey,
    )
    console.log('acceptedPower', proposalData.acceptedPower.toNumber())
    console.log('rejectedPower', proposalData.rejectedPower.toNumber())
  })
  it('Is close Proposal!', async () => {
    setTimeout(async () => {
      const tx = await program.rpc.close(new BN(1), new BN(0), {
        accounts: {
          authority: program.provider.wallet.publicKey,
          proposal: proposal.publicKey,
          treasurer,
          mint: mint.publicKey,
          proposalTokenAccount: proposalTokenAccount,
          // Wallet accounts
          receipt,
          walletTokenAccount,
          // System Program Address
          tokenProgram: utils.token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
          rent: web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [],
      })
    }, 10000)
  })
})
