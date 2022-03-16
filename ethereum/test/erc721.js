const RustToken721 = artifacts.require("RustToken721");

const owner = "0x290ef15cbfff84fd591b454a4c06d8b487c73321";

contract("RustToken721", (accounts) => {
  it("mint 1", async () => {
    const contract = await RustToken721.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    await contract.mint(owner, "TokenA");
    const tokenURI = await contract.tokenURI(1);
    assert.equal(
      tokenURI,
      "https://test-token.jp/TokenA.metadata.json",
      "error"
    );
  });

  it("mint 2", async () => {
    const contract = await RustToken721.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    try {
      await contract.mint(owner, "TokenA");
      throw new Error();
    } catch (error) {
      assert.equal(error.reason, "already mint", "error");
    }
  });

  it("ownerAddressOf 1", async () => {
    const contract = await RustToken721.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    const address = await contract.ownerAddressOf("TokenA");
    assert.equal(
      address,
      "0x290Ef15cbfFf84Fd591b454a4c06D8b487C73321",
      "error"
    );
  });

  it("ownerAddressOf 2", async () => {
    const contract = await RustToken721.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    const address = await contract.ownerAddressOf("TokenB");
    assert.equal(
      address,
      "0x0000000000000000000000000000000000000000",
      "error"
    );
  });

  it("currentSupply 1", async () => {
    const contract = await RustToken721.deployed();
    contract.setTokenBaseURI("https://test-token.jp");

    const supply = await contract.currentSupply();
    assert.equal(supply, 1, "error");
  });
});
