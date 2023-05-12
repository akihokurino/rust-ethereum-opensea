import { expect } from "chai";
import { ethers } from "hardhat";

describe("Nft1155", function () {
  it("should mint and get token url", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A", 10);
    expect(await contract.uri(1)).to.equal("ipfs://A");
  });

  it("should error when mint by not owner", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await expect(contract.connect(other).mint("A", 10)).to.be.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should get name and symbol", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    expect(await contract.name()).to.equal("RustToken");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get latest token id", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A", 10);
    await contract.mint("B", 10);
    expect(await contract.latestTokenId()).to.equal(2);

    await contract.mint("C", 30);
    expect(await contract.latestTokenId()).to.equal(3);
  });

  it("should get total supply", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    await contract.mint("A", 10);
    await contract.mint("B", 10);
    expect(await contract.totalSupply()).to.equal(20);

    await contract.mint("C", 30);
    expect(await contract.totalSupply()).to.equal(50);
  });

  it("should get total owned", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A", 10);
    await contract.mint("B", 10);
    expect(await contract.totalOwned()).to.equal(20);

    await contract["safeTransferFrom(address,address,uint256,uint256,bytes)"](
      owner.address,
      other.address,
      1,
      5,
      ethers.utils.formatBytes32String("")
    );
    await contract["safeTransferFrom(address,address,uint256,uint256,bytes)"](
      owner.address,
      other.address,
      2,
      3,
      ethers.utils.formatBytes32String("")
    );
    expect(await contract.totalOwned()).to.equal(12);
  });

  it("should get is owner", async () => {
    const Contract = await ethers.getContractFactory("Nft1155");
    const contract = await Contract.deploy("RustToken", "RT");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A", 10);
    expect(await contract.isOwner(1, owner.address)).to.equal(true);
    expect(await contract.isOwner(1, other.address)).to.equal(false);
  });
});
