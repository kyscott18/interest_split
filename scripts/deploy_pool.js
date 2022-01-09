import { LCDClient, MsgInstantiateContract, MsgStoreCode, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import * as fs from 'fs';
import fetch from 'isomorphic-fetch';

// Fetch gas prices and convert to `Coin` format.
const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json();
const gasPricesCoins = new Coins(gasPrices);

const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev/",
  chainID: "bombay-12",
  gasPrices: gasPricesCoins,
  gasAdjustment: "1.5",
  gas: 10000000,
});

const mk = new MnemonicKey({
  mnemonic: 'popular raven ginger mechanic blind celery uncle will upon tilt midnight cannon wheat issue picture grass either family scheme world salad rice obtain auction'
})

// const mk = new MnemonicKey({
//   mnemonic: 'satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn'
// })

// // connect to localterra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

const wallet = terra.wallet(mk);

const storeCode = new MsgStoreCode(
  wallet.key.accAddress,
  fs.readFileSync('./artifacts/core_pool.wasm').toString('base64')
);
const storeCodeTx = await wallet.createAndSignTx({
  msgs: [storeCode],
});
const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

console.log(storeCodeTxResult);

if (isTxError(storeCodeTxResult)) {
  throw new Error(
    `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
  );
}

const {
  store_code: { code_id },
} = storeCodeTxResult.logs[0].eventsByType;

console.log(code_id)

console.log(wallet.key.accAddress, 'addresser')

const instantiate = new MsgInstantiateContract(
  wallet.key.accAddress,
  wallet.key.accAddress,
  +code_id[0], // code ID
  {
    pool_name: "Test",
    beneficiary: "terra1lm3c7tldz9m08duvce3t5f3n6r2r0e33f2ewgu",
    fee_collector: "terra1lm3c7tldz9m08duvce3t5f3n6r2r0e33f2ewgu",
    money_market: "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal",
    dp_code_id: 148,
  }, // InitMsg
);

const instantiateTx = await wallet.createAndSignTx({
  msgs: [instantiate],
});
const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

console.log(instantiateTxResult);

if (isTxError(instantiateTxResult)) {
  throw new Error(
    `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
  );
}

const {
  instantiate_contract: { contract_address },
} = instantiateTxResult.logs[0].eventsByType;

console.log(contract_address)