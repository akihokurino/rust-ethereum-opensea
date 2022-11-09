import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("RustToken721", function () {
  it("should mint and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken721", {}),
      [],
      {
        initializer: "initialize",
      }
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    expect(await contract.tokenURI(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A"
    );
  });

  it("should get name and symbol", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken721", {}),
      [],
      {
        initializer: "initialize",
      }
    );

    const [owner, other] = await ethers.getSigners();

    expect(await contract.name()).to.equal("RustToken721");
    expect(await contract.symbol()).to.equal("RT");
  });

  it("should get currentSupply", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken721", {}),
      [],
      {
        initializer: "initialize",
      }
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await contract.mint(other.address, "B");

    const supply = await contract.currentSupply();
    expect(supply).to.equal(2);
  });

  it("should get usedTokenNames", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken721", {}),
      [],
      {
        initializer: "initialize",
      }
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await contract.mint(other.address, "B");

    const names: string[] = await contract.usedTokenNames();
    expect(names[0]).to.equal("A");
    expect(names[1]).to.equal("B");
    expect(names.length).to.equal(2);
  });

  it("should upgradable", async () => {
    const contractV1 = await upgrades.deployProxy(
      await ethers.getContractFactory("RustToken721", {}),
      [],
      {
        initializer: "initialize",
      }
    );

    const [owner, other] = await ethers.getSigners();
    await contractV1.mint(owner.address, "A");

    const contractV2 = await upgrades.upgradeProxy(
      contractV1.address,
      await ethers.getContractFactory("RustToken721_V2")
    );

    expect(await contractV2.name()).to.equal("RustToken721");
    expect(await contractV2.tokenURI(1)).to.equal(
      "https://ipfs.moralis.io:2053/ipfs/A"
    );
    expect(await contractV2.hello()).to.equal("hello v2");
  });
});
