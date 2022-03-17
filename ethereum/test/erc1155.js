const RustToken1155 = artifacts.require("RustToken1155");

const owner = "0x290ef15cbfff84fd591b454a4c06d8b487c73321";

contract("RustToken1155", (accounts) => {
  it("RustToken1155 each method test", async () => {
    const contract = await RustToken1155.deployed();
    await contract.setTokenBaseURI("https://test-token.jp");

    const baseURI = await contract.tokenBaseURI();
    assert.equal(baseURI, "https://test-token.jp", "error");

    await contract.mint(owner, "TokenA", 10);
    const tokenURI1 = await contract.uri(1);
    assert.equal(
      tokenURI1,
      "https://test-token.jp/TokenA.metadata.json",
      "error"
    );

    try {
      await contract.mint(owner, "TokenA", 10);
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    await contract.mintBatch(owner, ["TokenB", "TokenC"], [10, 10]);
    const tokenURI2 = await contract.uri(2);
    assert.equal(
      tokenURI2,
      "https://test-token.jp/TokenB.metadata.json",
      "error"
    );
    const tokenURI3 = await contract.uri(3);
    assert.equal(
      tokenURI3,
      "https://test-token.jp/TokenC.metadata.json",
      "error"
    );

    try {
      await contract.mintBatch(owner, ["TokenD", "TokenA"], [10, 10]);
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    const supply = await contract.currentSupply();
    assert.equal(supply, 3, "error");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "TokenA", "error");
    assert.equal(names[1], "TokenB", "error");
    assert.equal(names[2], "TokenC", "error");
  });
});
