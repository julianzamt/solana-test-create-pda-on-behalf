# solana-create-pda-on-behalf
This repo was created to test that a pda account cannot be created from on program on behalf of another.

> Spoiler: It is **NOT** possible.

An account must sign its own creation, next to the account that will debited.

There are two different ways to invoke a TX from within a program (cpi): either with 
- invoke (which will be signed by the signers of the original tx)
- invoke_signed (which adds to the original signers a pda that acts as prove of the caller signing the tx as well).

With invoke, since an account must sign its own creation, it is not possible because because a PDA has no private key, 
thus cannot sign the original tx.

With invoke_signed, again the PDA to be created on behalf cannot be the one that signs. 
This is because its pubkey is obtained using Receiver program as programId, and invoke_signed 
enforces that the signing PDA is derived from the caller id.


## Usage
Pre-Requirements:
* solana cli
* nodeJS

### From the root folder:
1 - cd creator/cli && npm install && cd ../..
2 - solana-test-validator -r (This will start a local validator)  

### In another terminal:
3 - `cd creator/ && cargo build-sbf && solana program deploy target/deploy/creator.so --url localhost && cd ..`  
4 - Paste the returned programId into the PROGRAM_ID field in creator/cli/constants.ts   
5 - In creator/cli/constant.ts, insert your test private key. Try: $cat ~/.config/solana/id.json (If you don't have one yet, run `solana-keygen new` first)  
6 - Open creator/cli/index.ts. You'll find function calls to send txs to the deployed contracts.  
7 - Comment and uncomment fns to have a taste of what you can and cannot do.  
