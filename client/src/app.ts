import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
  Struct,
} from "@solana/web3.js";
import { readFileSync } from "fs";
import { fileURLToPath } from "url";
import { dirname } from "path";
import {
  decodeDepositHistory,
  DepositHistory,
  encodeDepositIx,
  encodeInitializeIx,
  encodeWithdrawIx,
} from "./serialization";

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export class App {
  static DEPOSIT_HISTORY_SEED = "deposit-history-seed";
  static DEPOSIT_SEED = "deposit";

  admin: Keypair;
  user: Keypair;
  program: Keypair;

  connection: Connection;

  depositHistory: PublicKey;
  deposit: PublicKey;

  constructor() {
    const workingDir = dirname(fileURLToPath(import.meta.url));
    this.admin = App.readKeypairFromPath(
      workingDir + "/../../localnet/admin.json"
    );
    this.user = App.readKeypairFromPath(
      workingDir + "/../../localnet/user.json"
    );
    this.program = App.readKeypairFromPath(
      workingDir + "/../../localnet/program.json"
    );
    this.connection = new Connection(
      "https://api.devnet.solana.com",
      "confirmed"
    );
    this.depositHistory = new PublicKey(0);
    this.deposit = new PublicKey(0);
  }

  async init() {
    this.depositHistory = (
      await PublicKey.findProgramAddress(
        [Buffer.from(App.DEPOSIT_HISTORY_SEED, "utf-8")],
        this.program.publicKey
      )
    )[0];

    this.deposit = (
      await PublicKey.findProgramAddress(
        [Buffer.from(App.DEPOSIT_SEED, "utf-8")],
        this.program.publicKey
      )
    )[0];

    const res = await this.connection.getAccountInfo(this.program.publicKey);
    if (!res) {
      console.error("Program was not deployed. Deploy it first.");
      process.exit(1);
    }

    const account = await this.connection.getAccountInfo(this.deposit);
    if (!account) {
      console.log(
        "Program was not initialised. Sending initialize transaction."
      );
      await this.initializeIx();
    }

    console.log("program", this.program.publicKey.toBase58());
    console.log("admin", this.admin.publicKey.toBase58());
    console.log("user", this.user.publicKey.toBase58());
    console.log("deposit history", this.depositHistory.toBase58());
    console.log("deposit", this.deposit.toBase58());
  }

  private async initializeIx() {
    const initializeIx = new TransactionInstruction({
      programId: this.program.publicKey,
      keys: [
        {
          pubkey: this.admin.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: this.depositHistory, isSigner: false, isWritable: true },
        { pubkey: this.deposit, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: encodeInitializeIx(),
    });

    const tx = new Transaction().add(initializeIx);
    const txHash = await this.connection.sendTransaction(tx, [this.admin], {
      preflightCommitment: "max",
    });
    console.log("initialize program tx", txHash);
    await delay(3000);
  }

  async getDepositedSol(): Promise<number> {
    const account = await this.connection.getAccountInfo(this.deposit);
    if (!account) {
      console.error("deposit account is not found");
      process.exit(1);
    }
    const rent_lamports =
      await this.connection.getMinimumBalanceForRentExemption(0);
    return (account.lamports - rent_lamports) / LAMPORTS_PER_SOL;
  }

  async getAdminSol(): Promise<number> {
    const account = await this.connection.getAccountInfo(this.admin.publicKey);
    if (!account) {
      console.error("deposit account is not found");
      process.exit(1);
    }
    const rent_lamports =
      await this.connection.getMinimumBalanceForRentExemption(0);
    return (account.lamports - rent_lamports) / LAMPORTS_PER_SOL;
  }

  async getDepositHistory(): Promise<DepositHistory> {
    const account = await this.connection.getAccountInfo(this.depositHistory);
    if (!account) {
      console.error("deposit account is not found");
      process.exit(1);
    }
    return decodeDepositHistory(account.data);
  }

  async depositSol(amount: number): Promise<void> {
    const depositLamports = amount * LAMPORTS_PER_SOL;
    const depositIx = new TransactionInstruction({
      programId: this.program.publicKey,
      keys: [
        {
          pubkey: this.user.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: this.deposit, isSigner: false, isWritable: true },
        { pubkey: this.depositHistory, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: encodeDepositIx(depositLamports),
    });

    const tx = new Transaction().add(depositIx);
    const txHash = await this.connection.sendTransaction(tx, [this.user], {
      preflightCommitment: "max",
    });
    console.log(`deposit ${amount} SOL tx`, txHash);
    await delay(3000);
  }

  async withdrawSol(): Promise<void> {
    const withdrawIx = new TransactionInstruction({
      programId: this.program.publicKey,
      keys: [
        {
          pubkey: this.admin.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: this.deposit, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      ],
      data: encodeWithdrawIx(),
    });

    const tx = new Transaction().add(withdrawIx);
    const txHash = await this.connection.sendTransaction(tx, [this.admin], {
      preflightCommitment: "max",
    });
    console.log(`Withdraw all SOL tx`, txHash);
    await delay(3000);
  }

  static readKeypairFromPath(path: string): Keypair {
    const data = JSON.parse(readFileSync(path, "utf-8"));
    return Keypair.fromSecretKey(Buffer.from(data));
  }
}
