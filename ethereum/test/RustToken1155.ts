import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("RustToken1155", function () {
  it("should mint and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    await contract.mint("A", 10);
    expect(await contract.uri(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/A"
    );
  });

  it("should error when mint by not owner", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    const [owner, other] = await ethers.getSigners();

    expect(contract.connect(other).mint("A", 10)).to.be.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should get name and symbol", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    expect(await contract.name()).to.equal("RustToken");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should set base uri", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    await contract.mint("A", 10);
    await contract.setBaseURI(
      "https://akiho-playground.infura-ipfs.io/ipfs/edit/"
    );

    expect(await contract.uri(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/edit/A"
    );
  });

  it("should error when set base uri by not owner", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    const [owner, other] = await ethers.getSigners();

    expect(
      contract
        .connect(other)
        .setBaseURI("https://akiho-playground.infura-ipfs.io/ipfs/edit/")
    ).to.be.revertedWith("Ownable: caller is not the owner");
  });

  it("should get latest token id", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    await contract.mint("A", 10);
    await contract.mint("B", 10);
    expect(await contract.latestTokenId()).to.equal(2);

    await contract.mint("C", 30);
    expect(await contract.latestTokenId()).to.equal(3);
  });

  it("should get total supply", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    await contract.mint("A", 10);
    await contract.mint("B", 10);
    expect(await contract.totalSupply()).to.equal(20);

    await contract.mint("C", 30);
    expect(await contract.totalSupply()).to.equal(50);
  });

  it("should get total owned", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

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
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A", 10);
    expect(await contract.isOwner(1, owner.address)).to.equal(true);
    expect(await contract.isOwner(1, other.address)).to.equal(false);
  });

  it("should upgradable", async () => {
    const contractV1 = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      ["RustToken", "RT"]
    );

    await contractV1.mint("A", 10);

    const contractV2 = await upgrades.upgradeProxy(
      contractV1.address,
      await ethers.getContractFactory("RustToken1155_V2"),
      {
        call: {
          fn: "initializeV2",
          args: ["world"],
        },
      }
    );

    expect(await contractV2.name()).to.equal("RustToken");
    expect(await contractV2.uri(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/A"
    );
    expect(await contractV2.hello()).to.equal("hello v2 world");
  });
});
