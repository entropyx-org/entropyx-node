// @ts-ignore
globalThis.WebSocket = require('websocket').w3cwebsocket; // W3C WebSocket module shim


const path = require('path');
const fs = require('fs');
const entropyx = require('../../../../nodejs/entropyx/entropyx.js');
const {
    Wallet, setDefaultStorageFolder,
    AccountKind, Mnemonic, Resolver,
    entropyxToSompi,
    sompiToEntropyXString,
    Address
} = entropyx;

let storageFolder = path.join(__dirname, '../../../data/wallets').normalize();
if (!fs.existsSync(storageFolder)) {
    fs.mkdirSync(storageFolder);
}

setDefaultStorageFolder(storageFolder);

(async()=>{
    //const filename = "wallet-394";
    const filename = "wallet-395";
    // const FILE_TX = path.join(storageFolder, filename+"-transactions.json");

    // let txs = {};
    // if (fs.existsSync(FILE_TX)){
    //     txs = JSON.parse(fs.readFileSync(FILE_TX)+"")
    // }

    const balance = {};
    //const transactions = {};
    let wallet;

    const chalk = new ((await import('chalk')).Chalk)();

    function log_title(title){
        console.log(chalk.bold(chalk.green(`\n\n${title}`)))
    }
    // function saveTransactions(){
    //     Object.keys(transactions).forEach(id=>{
    //         txs[id] = [...transactions[id].entries()];
    //     })
    //     fs.writeFileSync(FILE_TX, JSON.stringify(txs))
    // }

    async function log_transactions(accountId){
        
        // saveTransactions();
        function value(tx){
            if (tx.data.type == "change"){
                return tx.data.data.changeValue
            }
            if (["external", "incoming"].includes(tx.data.type)){
                return tx.data.data.value
            }
            if (["transfer-outgoing", "transfer-incoming", "outgoing", "batch"].includes(tx.data.type)){
                return tx.data.data.paymentValue?? tx.data.data.changeValue
            }
            
        }
        let transactionsResult = await wallet.transactionsDataGet({
            accountId,
            networkId: "testnet-11",
            start:0,
            end:20
        })
        //console.log("transactions", transactionsResult.transactions)
        let list = [];
        transactionsResult.transactions.forEach((tx)=>{
            // console.log("ID:", id);
            // console.log("type:", tx.data.type, ", value:", value(tx));
            // console.log(chalk.dim("----------------------------"))
            // let addresses = tx.data.data.utxoEntries.map(utxo=>{
            //     return utxo.address.substring(0, 5)+"..."
            // });
            list.push({
                Id: tx.id,
                Type: tx.data.type,
                Value: sompiToEntropyXString(value(tx)||0)
            });
            //console.log("tx.data", tx.id, tx.data)
        });

        log_title("Transactions")
        console.table(list)
        console.log("");

    }

    try {
        
        const walletSecret = "abc";
        wallet = new Wallet({
            resident: false, 
            encoding: "borsh",
            networkId: "mainnet", 
            url: "ws://127.0.0.1:17110"
        });
        //console.log("wallet", wallet)
        // Ensure wallet file
        if (!await wallet.exists()){
            let response = await wallet.walletCreate({
                walletSecret,
            });

            // let response = await wallet.walletImport({
            //     //walletSecret,
            //     walletSecret: "123",
            //     walletData: "4b4153570000000000000000d3010000e3be9a71f8bb656cbaf9bc012c82607c41130574b87700df14b209e6a75976349e7f350c5f594899abcda3ab00fa7243179458e1278a8865d1055537bf4250461357e665c02d6cd53404e8ca97d4c903c1a6d80d4631aa8f393b166858cce07a3d291b0f16a9e07451aa5ccba0e4149b0556ff59d01fe34354ee1f6ee075cab9328ceedb0a2ed54b8a2ade784ba0500d890f4baa3af2c1abf17ac6cb9b43b77a7eddda7f05996a2a8fd50613f28a37defa0d16f75ff5cd69a7b790c284444f8ed3e4b8fb320672943829005c691c390b06858156882d6e2bf87df9cdf8c0400c6b059516b1db79c5797d91606ca8e0900c465bf5f8805f56dc63159530c5e8bab0a2e6380096251b892d6ae80fc695b19e49b531658d94bd00464955764f6ecfbb9bb2fea118b78a2954138f7a8d67a558e27c391379ccc02e2d667f31e64f33da379dce46bc8c0960c85ce761ce94a708fa1af468eacb69d5920da9c802f980dd1653e0fb4ecd4568cc3e1b406905d979f9761fdb6f3d26001af5bdc608d96e5cbe72d9d6fe170a9bda67c54b90c1289e0bf9c5e86656f46d625ba4f668a71ba60615ad41f6e3a2d724afa2869314af4e1ad7e6f71c94213a46dfa2801648bc5f44952eed3387b229b0527f1c8c801e5e8f360000000000"
            //     //walletData: "4b41535700000000000000001304000008027eaaff83c8f0bde79d1d23ead59bda0c3aeb8368efed8f324f063dee068603120ac20626892412cb12fbebcded316624b8a258ee74649d67a9094b125cf30669dc8c485ae1375a1ccc8ac1a1ff59446f307e497f77ea302cf9acc67b5684dc39dd4f700640854bc9e335ba1702a4a3384d49d489c133daced7279ad45c360a089fa89883fd29c0b8a1522a37dcfa0845aba72304bf9315f8cf719ee6b96a57dd1d7f4421aa45ce0c41f29ac85669bd49a916fe3e7f3cf71364d11db45a0ef0fdf42a3c5f7a5451a91875db918f23ca8252fd2a2fd42e66827681ab6d66a800616ee56a66fdcaeccd2e9a3d5f13e1ef31ea54b68b7eff4b69633122167121f7de620e6feca8d3edebce3e33798e01c7fcdb79f9da7362540c20dccaf4a6e7c9ea4ca064b458418a34bd75a9dda6aa2fa8ec0d050f507360eff256a7ea13ceb253fb61422aef9cbbe1ac827adb92993992471827329fd8e71777b598a6467cdf69914d647f679f7b2dc9cbd847572fe7293c4fb569802b8be966572988083f6f59a053488109e638c5badbe5d70136a71ac02a7ec2fed8d29fc1556eeeae06c27ecbf718f0e7e5cd6108ca16f7666f224f6ca0869ae82af6a9ff80eb54f0cbd91d1a3d30bdccad76d754b8162883ca45bc453a5596e9adb9ab376403748346768c49b762668b5c68aaef3e48fe477ca99b84315e4cf875be62013e93744338c29b6e85318bca6a034ae12d85521d20ac33f0edb3bced4480403bd37c53d103b554a456ce48df83e8c1469befbd60f802b28eb5316c19189817104e82955315d5dc570da31649bb1a0dfaf1fbb68851269b383a01ba16abe7178f6ef8ddac5006d839e6a93d839a5cbbc1bb585e3b66d2222e474539c074b3a4ee39a64d0692227767b0f68a1d9acd6bb77474281afe0a73710024765de5187d19dc0451ed5d71eb8f4324706a5be6783bdb3c3ada3768d9ee3f11012ade258fe13d0171f019bb73902d2d2abdd361dc00dd3cd267b3c60dbbb153c6a4ea39e2bfa9a2216de39bf6803c8d32b2c8fe571c6f5ee1c4e1ccd958dfc3f65f41548230c7ed62f9ce3ea210e88ef17b61978292412eea8306159531ab969e970ad2f9bb1dfa03e9e5f88051f8a6422f2ace86f087cc059ebca102503e680405821937c07e527c48e4bc50d25981d605976b2f9736d5a8302bef190db04176e424263aa9b99bf0c4bdf1d04c6585dd70e25072b81b30d287c6d3d98cbd4d0c61a972dfa03c2757f6fdd53a79ec35819f1f4f24ee0266b2dcc1fc2137a0d33d6d8f2a366b2c47a3662386ac8f1fd9100e04059a2c2e148e14730ebd0c3cc8c9381b30dc9bb2bbfe57f6d6ddcb9ce639e5e24bd154128fff89a1e8060eb0aba7e606960a5c4096e6d33608b6b17d8cc2908f8c097710c4675f4637d38e4dffaee42fce7062a2629869eaadac23639c881587fa92f800562a572be8cf670000000000"
            // });

            console.log("walletCreate : response", response)
        }

        wallet.addEventListener(({type, data})=>{

            switch (type){
                case "maturity":
                case "pending":
                case "discovery":
                    //console.log("transactions[data.binding.id]", data.binding.id, transactions[data.binding.id], transactions)
                    // console.log("record.hasAddress :receive:", data.hasAddress(firstAccount.receiveAddress));
                    // console.log("record.hasAddress :change:", data.hasAddress(firstAccount.changeAddress));
                    // console.log("record.data", data.data)
                    // console.log("record.blockDaaScore", data.blockDaaScore)
                    if (data.type != "change"){
                        //transactions[data.binding.id].set(data.id+"", data.serialize());
                        // @ts-ignore
                        log_transactions(data.binding.id)
                    }
                    break;
                case "reorg":
                    //transactions[data.binding.id].delete(data.id+"");
                    // @ts-ignore
                    log_transactions(data.binding.id)
                    break;
                case "balance":
                    balance[data.id] = data.balance;
                    log_title("Balance");
                    let list = [];
                    Object.keys(balance).map(id=>{
                        list.push({
                            Account: id.substring(0, 5)+"...",
                            Mature: sompiToEntropyXString(data.balance.mature),
                            Pending: sompiToEntropyXString(data.balance.pending),
                            Outgoing: sompiToEntropyXString(data.balance.outgoing),
                            MatureUtxo: data.balance.matureUtxoCount,
                            PendingUtxo: data.balance.pendingUtxoCount,
                            StasisUtxo: data.balance.stasisUtxoCount
                        })
                    })
                    console.table(list)
                    console.log("");
                    break;
                case "daa-score-change":
                    if (data.currentDaaScore%1000 == 0){
                        console.log(`[EVENT] ${type}:`, data.currentDaaScore)
                    }
                    break;
                case "server-status":
                case "utxo-proc-start":
                case "sync-state":
                case "account-activation":
                case "utxo-proc-stop":
                case "connect":
                case "stasis":
                    //
                    break;
                default:
                    console.log(`[EVENT] ${type}:`, data)
                
            }
        })

        // Open wallet
        await wallet.walletOpen({
            walletSecret,
            accountDescriptors: false
        });

        // Ensure default account
        await wallet.accountsEnsureDefault({
            walletSecret,
            type: new AccountKind("bip32") // "bip32"
        });

        // // Create a new account
        // // create private key
        // const mnemonic = Mnemonic.random(24).phrase
        const mnemonic = "lock reflect muffin assault two turn bachelor cluster ready universe company resist time grunt gossip another rich diagram dilemma cover faith hill hour fee"; // Mnemonic.random(24).phrase;
        let prvKeyData =  await wallet.prvKeyDataCreate({
            walletSecret,
            mnemonic
        });
        console.log("mnemonic", mnemonic);
        console.log("prvKeyData", prvKeyData);

        const accountsCount = (await wallet.accountsEnumerate({})).accountDescriptors.length;
        if(accountsCount < 2){
            let account = await wallet.accountsCreate({
                walletSecret,
                type:"bip32",
                accountName:"Account-"+ accountsCount,
                prvKeyDataId: prvKeyData.prvKeyDataId
            });

            console.log("new account:", account);
        }

        // Connect to rpc
        await wallet.connect();

        // Start wallet processing
        await wallet.start();

        // List accounts
        let accounts = await wallet.accountsEnumerate({});
        let firstAccount = accounts.accountDescriptors[0];
        let secondAccount = accounts.accountDescriptors[1];

        //console.log("firstAccount:", firstAccount);

        // Activate Account
        await wallet.accountsActivate({
            accountIds:[firstAccount.accountId]
        });

        log_title("Accounts");
        accounts.accountDescriptors.forEach(a=>{
            console.log(`Account: ${a.accountId}`);
            console.log(`   Account type: ${a.kind.toString()}`);
            console.log(`   Account Name: ${a.accountName}`);
            console.log(`   Receive Address: ${a.receiveAddress}`);
            console.log(`   Change Address: ${a.changeAddress}`);
            console.log("")
        });

        // let walletData = await wallet.walletExport({
        //     walletSecret: walletSecret,
        //     includeTransactions: true
        // });

        // console.log("walletData", walletData);

        // // Account sweep/compound transactions
        // let sweepResult = await wallet.accountsSend({
        //     walletSecret,
        //     accountId: firstAccount.accountId
        // });
        // console.log("sweepResult", sweepResult)
        // await new Promise(resolve => setTimeout(resolve, 10000));
        // Send entropyx to address
        const targetAddress = "entropyx:qqnpx5j0f8p3myde92lajqq50nh2xa25q3annpdquvtzz3llsn2uv09ahnt8l"
        let balance = await wallet.rpc.getBalancesByAddresses({
            addresses: [firstAccount.receiveAddress?.toString(), firstAccount.changeAddress?.toString()]
        });
        console.log("balance", balance);
        await new Promise(resolve => setTimeout(resolve, 10000));
        let targetBalance =await wallet.rpc.getBalanceByAddress({
            address: targetAddress
        });
        console.log("targetBalance", targetBalance);
        let sendResult = await wallet.accountsSend({
            walletSecret,
            accountId: firstAccount.accountId,
            priorityFeeSompi: entropyxToSompi("0.001"),
            destination:[{
                address: targetAddress,//secondAccount.changeAddress,
                amount: entropyxToSompi("10")
            }]
        });
        console.log("sendResult", sendResult);

        // 阻塞代码执行30秒
        await new Promise(resolve => setTimeout(resolve, 30000));
        // // Transfer entropyx between accounts
        // let transferResult = await wallet.accountsTransfer({
        //     walletSecret,
        //     sourceAccountId: firstAccount.accountId,
        //     destinationAccountId: secondAccount.accountId,
        //     transferAmountSompi: entropyxToSompi("2.4"),
        // });
        // console.log("transferResult", transferResult);

        while(true) {
            let balance = await wallet.rpc.getBalanceByAddress({
                address: firstAccount.receiveAddress
            });
            console.log("balance", balance);
            await new Promise(resolve => setTimeout(resolve, 30000));
            let targetBalance =await wallet.rpc.getBalanceByAddress({
                address: targetAddress
            });
            console.log("targetBalance", targetBalance);
        }

    } catch(ex) {
        console.error("Error:", ex);
    }
})();