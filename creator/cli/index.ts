import * as web3 from '@solana/web3.js';

import { createPdaInvokeSigned, createPdaInvoke } from './functions';

const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // await createPdaInvokeSigned(connection);
  // await createPdaInvoke(connection);
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
