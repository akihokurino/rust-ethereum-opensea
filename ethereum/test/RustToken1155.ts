import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("RustToken1155", function () {
  it("should mint and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A", 10);
    expect(await contract.uri(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A"
    );
  });

  it("should mintBatch and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mintBatch(owner.address, ["A"], [10]);
    expect(await contract.uri(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A"
    );
  });

  it("should get name and symbol", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();

    expect(await contract.name()).to.equal("RustToken1155");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get currentSupply", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A", 10);
    await contract.mint(other.address, "B", 10);

    const supply = await contract.currentSupply();
    expect(supply).to.equal(2);
  });

  it("should get usedTokenNames", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A", 10);
    await contract.mint(other.address, "B", 10);

    const names: string[] = await contract.usedTokenNames();
    expect(names[0]).to.equal("A");
    expect(names[1]).to.equal("B");
    expect(names.length).to.equal(2);
  });

  it("should upgradable", async () => {
    const contractV1 = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken1155", {}),
      []
    );

    const [owner, other] = await ethers.getSigners();
    await contractV1.mint(owner.address, "A", 10);

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

    expect(await contractV2.name()).to.equal("RustToken1155");
    expect(await contractV2.uri(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A"
    );
    expect(await contractV2.hello()).to.equal("hello v2 world");
  });
});
