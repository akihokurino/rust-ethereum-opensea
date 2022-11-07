import { expect } from "chai";
import { ethers } from "hardhat";

describe("RustToken721", function () {
  it("should mint and get token url", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.setTokenBaseURI("https://ipfs.moralis.io:2053/ipfs");
    expect(await contract.tokenBaseURI()).to.equal(
      "https://ipfs.moralis.io:2053/ipfs"
    );

    await contract.mint(owner.address, "A");
    expect(await contract.tokenURI(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A.metadata.json"
    );
  });

  it("should get name and symbol", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    expect(await contract.name()).to.equal("RustToken721");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get currentSupply", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await contract.mint(other.address, "B");

    const supply = await contract.currentSupply();
    expect(supply).to.equal(2);
  });

  it("should get usedTokenNames", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy();
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await contract.mint(other.address, "B");

    const names: string[] = await contract.usedTokenNames();
    expect(names[0]).to.equal("A");
    expect(names[1]).to.equal("B");
    expect(names.length).to.equal(2);
  });
});
