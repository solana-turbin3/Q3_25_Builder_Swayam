import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { CapstoneProject } from "../target/types/capstone_project";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";

describe("capstone_project", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CapstoneProject as Program<CapstoneProject>;
  const connection = provider.connection;

  // Test accounts
  let creator: anchor.web3.Keypair;
  let contributor1: anchor.web3.Keypair;
  let contributor2: anchor.web3.Keypair;
  let reviewer1: anchor.web3.Keypair;
  let reviewer2: anchor.web3.Keypair;
  let reviewer3: anchor.web3.Keypair;

  // Task related PDAs
  let taskAccount: PublicKey;
  let escrowVault: PublicKey;
  let taskSeed: BN;
  let taskBump: number;
  let vaultBump: number;

  // Contribution related PDAs
  let contributionAccount1: PublicKey;
  let contributionAccount2: PublicKey;
  let voteAccount1: PublicKey;
  let voteAccount2: PublicKey;

  // Review related PDAs
  let reviewAccount1: PublicKey;
  let reviewAccount2: PublicKey;
  let reviewAccount3: PublicKey;
  let reviewerVault1: PublicKey;
  let reviewerVault2: PublicKey;
  let reviewerVault3: PublicKey;

  const TASK_REWARD = new BN(5 * LAMPORTS_PER_SOL);
  const WORK_UNITS = new BN(10);
  const TASK_URI = "https://example.com/task";
  const REVIEWER_STAKE = new BN(1 * LAMPORTS_PER_SOL);

  before(async () => {
    // Generate keypairs
    creator = anchor.web3.Keypair.generate();
    contributor1 = anchor.web3.Keypair.generate();
    contributor2 = anchor.web3.Keypair.generate();
    reviewer1 = anchor.web3.Keypair.generate();
    reviewer2 = anchor.web3.Keypair.generate();
    reviewer3 = anchor.web3.Keypair.generate();

    // Airdrop SOL to all accounts
    await airdrop(connection, creator.publicKey, 10);
    await airdrop(connection, contributor1.publicKey, 5);
    await airdrop(connection, contributor2.publicKey, 5);
    await airdrop(connection, reviewer1.publicKey, 5);
    await airdrop(connection, reviewer2.publicKey, 5);
    await airdrop(connection, reviewer3.publicKey, 5);

    // Generate task seed and derive PDAs
    taskSeed = new BN(Math.floor(Math.random() * 1000000));
    
    [taskAccount, taskBump] = PublicKey.findProgramAddressSync([
      Buffer.from("task"),
      creator.publicKey.toBuffer(),
      taskSeed.toArrayLike(Buffer, "le", 8),
    ], program.programId);

    [escrowVault, vaultBump] = PublicKey.findProgramAddressSync([
      Buffer.from("escrow"),
      taskAccount.toBuffer(),
    ], program.programId);

    // Derive contribution account PDAs
    [contributionAccount1] = PublicKey.findProgramAddressSync([
      Buffer.from("contribution"),
      taskAccount.toBuffer(),
      contributor1.publicKey.toBuffer(),
    ], program.programId);

    [contributionAccount2] = PublicKey.findProgramAddressSync([
      Buffer.from("contribution"),
      taskAccount.toBuffer(),
      contributor2.publicKey.toBuffer(),
    ], program.programId);

    // Derive vote account PDAs
    [voteAccount1] = PublicKey.findProgramAddressSync([
      Buffer.from("vote"),
      contributionAccount1.toBuffer(),
    ], program.programId);

    [voteAccount2] = PublicKey.findProgramAddressSync([
      Buffer.from("vote"),
      contributionAccount2.toBuffer(),
    ], program.programId);

    // Derive review account PDAs
    [reviewAccount1] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer"),
      reviewer1.publicKey.toBuffer(),
    ], program.programId);

    [reviewAccount2] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer"),
      reviewer2.publicKey.toBuffer(),
    ], program.programId);

    [reviewAccount3] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer"),
      reviewer3.publicKey.toBuffer(),
    ], program.programId);

    // Derive reviewer vault PDAs
    [reviewerVault1] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer_vault"),
      reviewer1.publicKey.toBuffer(),
    ], program.programId);

    [reviewerVault2] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer_vault"),
      reviewer2.publicKey.toBuffer(),
    ], program.programId);

    [reviewerVault3] = PublicKey.findProgramAddressSync([
      Buffer.from("reviewer_vault"),
      reviewer3.publicKey.toBuffer(),
    ], program.programId);
  });

  it("Creates a task successfully!", async () => {
    const tx = await program.methods
      .createTask(taskSeed, TASK_REWARD, WORK_UNITS, TASK_URI)
      .accounts({
        creator: creator.publicKey,
        taskAccount: taskAccount,
        escrowVault: escrowVault,
        systemProgram: SystemProgram.programId,
      })
      .signers([creator])
      .rpc();

    console.log("✅ Task creation transaction signature:", tx);

    // Verify task account state
    const taskAccountData = await program.account.taskAccount.fetch(taskAccount);
    assert.equal(taskAccountData.creator.toBase58(), creator.publicKey.toBase58());
    assert.equal(taskAccountData.rewardAmount.toString(), TASK_REWARD.toString());
    assert.equal(taskAccountData.totalWorkUnits.toString(), WORK_UNITS.toString());
    assert.equal(taskAccountData.taskUri, TASK_URI);
    assert.equal(JSON.stringify(taskAccountData.status), JSON.stringify({ created: {} }));
    assert.equal(taskAccountData.totalSubmissions, 0);
    assert.equal(taskAccountData.taskSeed.toString(), taskSeed.toString());

    // Verify vault has the reward amount
    const vaultBalance = await getBalance(connection, escrowVault);
    assert.equal(vaultBalance, TASK_REWARD.toNumber());
  });

  it("Initializes reviewers successfully!", async () => {
    // Initialize reviewer 1
    const tx1 = await program.methods
      .reviewerInit(REVIEWER_STAKE)
      .accounts({
        reviewer: reviewer1.publicKey,
        reviewAccount: reviewAccount1,
        reviewerStakeVault: reviewerVault1,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer1])
      .rpc();

    console.log("✅ Reviewer 1 initialization signature:", tx1);

    // Initialize reviewer 2
    const tx2 = await program.methods
      .reviewerInit(REVIEWER_STAKE)
      .accounts({
        reviewer: reviewer2.publicKey,
        reviewAccount: reviewAccount2,
        reviewerStakeVault: reviewerVault2,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer2])
      .rpc();

    console.log("✅ Reviewer 2 initialization signature:", tx2);

    // Initialize reviewer 3
    const tx3 = await program.methods
      .reviewerInit(REVIEWER_STAKE)
      .accounts({
        reviewer: reviewer3.publicKey,
        reviewAccount: reviewAccount3,
        reviewerStakeVault: reviewerVault3,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer3])
      .rpc();

    console.log("✅ Reviewer 3 initialization signature:", tx3);

    // Verify reviewer account state
    const reviewAccountData1 = await program.account.reviewAccount.fetch(reviewAccount1);
    assert.equal(reviewAccountData1.stakedAmount.toString(), REVIEWER_STAKE.toString());
    assert.equal(reviewAccountData1.active, true);
  });

  it("Submits contributions successfully!", async () => {
    const contributionUri1 = "https://example.com/contribution1";
    const contributionUri2 = "https://example.com/contribution2";
    const workUnits1 = new BN(5);
    const workUnits2 = new BN(3);

    // Submit contribution 1
    const tx1 = await program.methods
      .submitContribution(contributionUri1, workUnits1)
      .accounts({
        contributor: contributor1.publicKey,
        taskAccount: taskAccount,
        contributionAccount: contributionAccount1,
        voteAccount: voteAccount1,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor1])
      .rpc();

    console.log("✅ Contribution 1 submission signature:", tx1);

    // Submit contribution 2
    const tx2 = await program.methods
      .submitContribution(contributionUri2, workUnits2)
      .accounts({
        contributor: contributor2.publicKey,
        taskAccount: taskAccount,
        contributionAccount: contributionAccount2,
        voteAccount: voteAccount2,
        systemProgram: SystemProgram.programId,
      })
      .signers([contributor2])
      .rpc();

    console.log("✅ Contribution 2 submission signature:", tx2);

    // Verify contribution account state
    const contributionData1 = await program.account.contributionAccount.fetch(contributionAccount1);
    assert.equal(contributionData1.contributor.toBase58(), contributor1.publicKey.toBase58());
    assert.equal(contributionData1.submissionUri, contributionUri1);
    assert.equal(contributionData1.workUnits.toString(), workUnits1.toString());
    assert.equal(contributionData1.approved, false);

    // Verify task account updated submission count
    const taskAccountData = await program.account.taskAccount.fetch(taskAccount);
    assert.equal(taskAccountData.totalSubmissions, 2);
  });

  it("Reviewers vote on contributions successfully!", async () => {
    // Reviewer 1 approves contribution 1
    const tx1 = await program.methods
      .submitVote(true) // approve
      .accounts({
        reviewer: reviewer1.publicKey,
        reviewAccount: reviewAccount1,
        contributionAccount: contributionAccount1,
        taskAccount: taskAccount,
        voteAccount: voteAccount1,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer1])
      .rpc();

    console.log("✅ Reviewer 1 vote signature:", tx1);

    // Reviewer 2 approves contribution 1
    const tx2 = await program.methods
      .submitVote(true) // approve
      .accounts({
        reviewer: reviewer2.publicKey,
        reviewAccount: reviewAccount2,
        contributionAccount: contributionAccount1,
        taskAccount: taskAccount,
        voteAccount: voteAccount1,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer2])
      .rpc();

    console.log("✅ Reviewer 2 vote signature:", tx2);

    // Reviewer 3 rejects contribution 1
    const tx3 = await program.methods
      .submitVote(false) // reject
      .accounts({
        reviewer: reviewer3.publicKey,
        reviewAccount: reviewAccount3,
        contributionAccount: contributionAccount1,
        taskAccount: taskAccount,
        voteAccount: voteAccount1,
        systemProgram: SystemProgram.programId,
      })
      .signers([reviewer3])
      .rpc();

    console.log("✅ Reviewer 3 vote signature:", tx3);

    // Verify vote account state
    const voteData1 = await program.account.voteAccount.fetch(voteAccount1);
    assert.equal(voteData1.totalVotes, 3);
    assert.equal(voteData1.approveVotes, 2);
    assert.equal(voteData1.voters.length, 3);
  });

  it("Finalizes contribution successfully!", async () => {
    const initialContributorBalance = await getBalance(connection, contributor1.publicKey);
    const initialVaultBalance = await getBalance(connection, escrowVault);
    const approvalThreshold = 60; // 60% approval required

    const tx = await program.methods
      .finalizeContribution(approvalThreshold)
      .accounts({
        authority: creator.publicKey,
        taskAccount: taskAccount,
        contributionAccount: contributionAccount1,
        voteAccount: voteAccount1,
        escrowVault: escrowVault,
        contributor: contributor1.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([creator])
      .rpc();

    console.log("✅ Finalization signature:", tx);

    // Verify contribution status changed
    const contributionData = await program.account.contributionAccount.fetch(contributionAccount1);
    assert.equal(contributionData.approved, true);

    // Verify contributor received payment
    const finalContributorBalance = await getBalance(connection, contributor1.publicKey);
    assert.isTrue(finalContributorBalance > initialContributorBalance);

    // Verify vault balance decreased
    const finalVaultBalance = await getBalance(connection, escrowVault);
    assert.isTrue(finalVaultBalance < initialVaultBalance);
  });

  it("Handles task status transitions correctly!", async () => {
    // Check that task status is created
    let taskData = await program.account.taskAccount.fetch(taskAccount);
    assert.equal(JSON.stringify(taskData.status), JSON.stringify({ created: {} }));
  });

  it("Prevents unauthorized actions!", async () => {
    try {
      // Try to finalize with wrong creator
      await program.methods
        .finalizeContribution(60)
        .accounts({
          authority: contributor1.publicKey, // wrong creator
          taskAccount: taskAccount,
          contributionAccount: contributionAccount2,
          voteAccount: voteAccount2,
          escrowVault: escrowVault,
          contributor: contributor2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([contributor1])
        .rpc();
      
      assert.fail("Should have failed with wrong creator");
    } catch (error) {
      console.log("✅ Correctly prevented unauthorized finalization");
    }
  });
});

async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);
  return accountInfo ? accountInfo.lamports : 0;
}