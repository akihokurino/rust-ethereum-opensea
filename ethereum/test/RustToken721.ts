import { expect } from "chai";
import { ethers } from "hardhat";

describe("RustToken721", function () {
  it("should mint and get token url", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A");
    expect(await contract.tokenURI(1)).to.equal("ipfs://A");
  });

  it("should error when mint by not owner", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    expect(contract.connect(other).mint("A")).to.be.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should get name and symbol", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    expect(await contract.name()).to.equal("RustToken");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get latest token id", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A");
    await contract.mint("B");
    expect(await contract.latestTokenId()).to.equal(2);

    await contract.mint("C");
    expect(await contract.latestTokenId()).to.equal(3);
  });

  it("should get total supply", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A");
    await contract.mint("B");
    expect(await contract.totalSupply()).to.equal(2);

    await contract.mint("C");
    expect(await contract.totalSupply()).to.equal(3);
  });

  it("should get total owned", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");
    await contract.mint("B");
    await contract.mint("C");
    expect(await contract.totalOwned()).to.equal(3);

    await contract["safeTransferFrom(address,address,uint256)"](
      owner.address,
      other.address,
      1
    );
    expect(await contract.totalOwned()).to.equal(2);
  });

  it("should get is owner", async () => {
    const Contract = await ethers.getContractFactory("RustToken721");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");
    expect(await contract.isOwner(1, owner.address)).to.equal(true);
    expect(await contract.isOwner(1, other.address)).to.equal(false);
  });
});
