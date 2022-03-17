const RustToken721 = artifacts.require("RustToken721");

const owner = "0x290ef15cbfff84fd591b454a4c06d8b487c73321";

contract("RustToken721", (accounts) => {
  it("RustToken721 each method test", async () => {
    const contract = await RustToken721.deployed();
    await contract.setTokenBaseURI("https://test-token.jp");

    const baseURI = await contract.tokenBaseURI();
    assert.equal(baseURI, "https://test-token.jp", "error");

    await contract.mint(owner, "TokenA");
    const tokenURI1 = await contract.tokenURI(1);
    assert.equal(
      tokenURI1,
      "https://test-token.jp/TokenA.metadata.json",
      "error"
    );

    await contract.mint(owner, "TokenB");
    const tokenURI2 = await contract.tokenURI(2);
    assert.equal(
      tokenURI2,
      "https://test-token.jp/TokenB.metadata.json",
      "error"
    );

    try {
      await contract.mint(owner, "TokenA");
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }

    const address = await contract.ownerAddressOf("TokenA");
    assert.equal(
      address,
      "0x290Ef15cbfFf84Fd591b454a4c06D8b487C73321",
      "error"
    );

    const unknownAddress = await contract.ownerAddressOf("TokenC");
    assert.equal(
      unknownAddress,
      "0x0000000000000000000000000000000000000000",
      "error"
    );

    const supply = await contract.currentSupply();
    assert.equal(supply, 2, "error");

    const names = await contract.usedTokenNames();
    assert.equal(names[0], "TokenA", "error");
    assert.equal(names[1], "TokenB", "error");
  });
});
