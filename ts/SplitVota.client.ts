/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, ExecMsg, Addr, Uint128, QueryMsg, QueryMsg1, AdminListResp } from "./SplitVota.types";
export interface SplitVotaReadOnlyInterface {
  contractAddress: string;
  adminList: () => Promise<AdminListResp>;
}
export class SplitVotaQueryClient implements SplitVotaReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.adminList = this.adminList.bind(this);
  }

  adminList = async (): Promise<AdminListResp> => {
    return this.client.queryContractSmart(this.contractAddress, {
      admin_list: {}
    });
  };
}
export interface SplitVotaInterface extends SplitVotaReadOnlyInterface {
  contractAddress: string;
  sender: string;
  split: ({
    amounts
  }: {
    amounts: Addr[][];
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawRemains: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class SplitVotaClient extends SplitVotaQueryClient implements SplitVotaInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.split = this.split.bind(this);
    this.withdrawRemains = this.withdrawRemains.bind(this);
  }

  split = async ({
    amounts
  }: {
    amounts: Addr[][];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      split: {
        amounts
      }
    }, fee, memo, _funds);
  };
  withdrawRemains = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_remains: {}
    }, fee, memo, _funds);
  };
}