// @ts-ignore
import lo from "buffer-layout";
import { Struct, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import borsh from "borsh";
import BN from "bn.js";

enum Instructions {
  Deposit = 0,
  Withdraw = 1,
  Initialize = 2,
}

export class DepositHistory extends Struct {
  get history(): { [s: string]: number } {
    const result = {};
    // @ts-ignore
    for (let [key, item] of this._history) {
      result[new PublicKey(key).toString()] =
        item.toNumber() / LAMPORTS_PER_SOL;
    }
    return result;
  }
}

const depositHistorySchema = new Map([
  [
    DepositHistory,
    {
      kind: "struct",
      fields: [["_history", { kind: "map", key: ["u8", 32], value: "u64" }]],
    },
  ],
]);

export function decodeDepositHistory(data: Buffer): DepositHistory {
  return borsh.deserializeUnchecked(depositHistorySchema, DepositHistory, data);
}

export function encodeDepositIx(amount: number): Buffer {
  const value = new Struct({ id: Instructions.Deposit, amount: amount });
  const schema = new Map([
    [
      Struct,
      {
        kind: "struct",
        fields: [
          ["id", "u8"],
          ["amount", "u64"],
        ],
      },
    ],
  ]);

  return Buffer.from(borsh.serialize(schema, value));
}

export function encodeWithdrawIx(): Buffer {
  const value = new Struct({ id: Instructions.Withdraw });
  const schema = new Map([
    [
      Struct,
      {
        kind: "struct",
        fields: [["id", "u8"]],
      },
    ],
  ]);

  return Buffer.from(borsh.serialize(schema, value));
}

export function encodeInitializeIx(): Buffer {
  const value = new Struct({ id: Instructions.Initialize });
  const schema = new Map([
    [
      Struct,
      {
        kind: "struct",
        fields: [["id", "u8"]],
      },
    ],
  ]);

  return Buffer.from(borsh.serialize(schema, value));
}
