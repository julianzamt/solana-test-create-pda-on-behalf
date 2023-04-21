import * as web3 from '@solana/web3.js';
import { Buffer } from 'buffer';
import { programId, SIGNER } from './constants';
import * as utils from './utils';

export const createPdaInvokeSigned = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let receiverPubkey = new web3.PublicKey("HuDo7Ukzrp3EXLKefwzbtz6FMuLLH2SLRu4ejanmCyXE");

  let instructionNumber = 0;

  let dataBuffer = Buffer.from('');

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);

  let [pdaAddress, _counterBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pda")],
    receiverPubkey
  );

  let createPdaIx = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: pdaAddress, isSigner: false, isWritable: false },
      { pubkey: receiverPubkey, isSigner: false, isWritable: false },
      { pubkey: signer.publicKey, isSigner: true, isWritable: true },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: dataBuffer,
  });

  let txReceipt = await web3.sendAndConfirmTransaction(
    connection,
    new web3.Transaction().add(createPdaIx),
    [signer]
  );
  return txReceipt;
};

export const createPdaInvoke = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let receiverPubkey = new web3.PublicKey("HuDo7Ukzrp3EXLKefwzbtz6FMuLLH2SLRu4ejanmCyXE");

  let instructionNumber = 1;

  let dataBuffer = Buffer.from('');

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);

  let [pdaAddress] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("pda")],
    receiverPubkey
  );

  let [other_pda_signer] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("signer")],
    programId
  );

  let createPdaIx = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: pdaAddress, isSigner: false, isWritable: true },
      { pubkey: receiverPubkey, isSigner: false, isWritable: false },
      { pubkey: other_pda_signer, isSigner: false, isWritable: false },
      { pubkey: signer.publicKey, isSigner: true, isWritable: true },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: dataBuffer,
  });

  let txReceipt = await web3.sendAndConfirmTransaction(
    connection,
    new web3.Transaction().add(createPdaIx),
    [signer]
  );
  return txReceipt;
};

/******* GETTERS ********/

// export const getPDA = async (
//   counterAddress: web3.PublicKey,
//   connection: web3.Connection,
//   signer: web3.Keypair = SIGNER
// ) => {
//   let counterInfo = await connection.getAccountInfo(
//     counterAddress,
//     'processed'
//   );

//   let data = counterInfo ? counterInfo.data : null;
//   if (!data) {
//     throw new Error('No data retrieved');
//   }

//   let counterStruct = Counter.decode(data);
//   return counterStruct;
// };
