const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("generator", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  it("It initializes the account", async () => {
    const program = anchor.workspace.Generator;
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.initialize("Hello World", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    // console.log('Data: ', account.data);
    console.log("ethereum address is 0xf5daf07280adaed3801b5c217f9dbd7c57ce4958")
    console.log("ethereum privatekey is ae97e5c9f0121ae55de8e855a2c8b6211543fab1337b6c7890cedf059bc61079")
    assert.ok(account.data === "Hello World");
    _baseAccount = baseAccount;

  });

  it("Updates a previously created account", async () => {
    const baseAccount = _baseAccount;
    const program = anchor.workspace.Generator;

    await program.rpc.update("Some new data", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });
  
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Updated data: ', account.data)
    assert.ok(account.data === "Some new data");
    console.log('all account data:', account)
    console.log('All data: ', account.dataList);
    assert.ok(account.dataList.length === 2);
  });
});