const RustToken1155 = artifacts.require("RustToken1155");

const owner = "0x290ef15cbfff84fd591b454a4c06d8b487c73321";

contract("RustToken1155", (accounts) => {
  it("mint 1", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    await contract.mint(owner, "TokenA", 10);
    const tokenURI = await contract.uri(1);
    assert.equal(
      tokenURI,
      "https://test-token.jp/TokenA.metadata.json",
      "error"
    );
  });

  it("mint 2", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    try {
      await contract.mint(owner, "TokenA", 10);
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }
  });

  it("mintBatch 1", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    await contract.mintBatch(owner, ["TokenB", "TokenC"], [10, 10]);
    const tokenURI1 = await contract.uri(2);
    assert.equal(
      tokenURI1,
      "https://test-token.jp/TokenB.metadata.json",
      "error"
    );
    const tokenURI2 = await contract.uri(3);
    assert.equal(
      tokenURI2,
      "https://test-token.jp/TokenC.metadata.json",
      "error"
    );
  });

  it("mintBatch 2", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    try {
      await contract.mintBatch(owner, ["TokenD", "TokenA"], [10, 10]);
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }
  });

  it("currentSupply 1", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    const supply = await contract.currentSupply();
    assert.equal(supply, 3, "error");
  });

  it("usedTokenNames 1", async () => {
    const contract = await RustToken1155.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "TokenA", "error");
    assert.equal(names[1], "TokenB", "error");
    assert.equal(names[2], "TokenC", "error");
  });
});
