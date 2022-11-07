import { expect } from "chai";
import { ethers } from "hardhat";

describe("RustToken1155", function () {
  it("should mint and get token url", async () => {
    const Contract = await ethers.getContractFactory("RustToken1155");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.setTokenBaseURI("https://ipfs.moralis.io:2053/ipfs");
    expect(await contract.tokenBaseURI()).to.equal(
      "https://ipfs.moralis.io:2053/ipfs"
    );

    await contract.mint(owner.address, "A", 10);
    expect(await contract.uri(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A.metadata.json"
    );
  });

  it("should mintBatch and get token url", async () => {
    const Contract = await ethers.getContractFactory("RustToken1155");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.setTokenBaseURI("https://ipfs.moralis.io:2053/ipfs");
    expect(await contract.tokenBaseURI()).to.equal(
      "https://ipfs.moralis.io:2053/ipfs"
    );

    await contract.mintBatch(owner.address, ["A"], [10]);
    expect(await contract.uri(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A.metadata.json"
    );
  });

  it("should get name and symbol", async () => {
    const Contract = await ethers.getContractFactory("RustToken1155");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    expect(await contract.name()).to.equal("RustToken1155");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get currentSupply", async () => {
    const Contract = await ethers.getContractFactory("RustToken1155");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A", 10);
    await contract.mint(other.address, "B", 10);

    const supply = await contract.currentSupply();
    expect(supply).to.equal(2);
  });

  it("should get usedTokenNames", async () => {
    const Contract = await ethers.getContractFactory("RustToken1155");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A", 10);
    await contract.mint(other.address, "B", 10);

    const names: string[] = await contract.usedTokenNames();
    expect(names[0]).to.equal("A");
    expect(names[1]).to.equal("B");
    expect(names.length).to.equal(2);
  });
});
