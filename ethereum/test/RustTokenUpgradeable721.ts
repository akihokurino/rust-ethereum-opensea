import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("RustTokenUpgradeable721", function () {
  it("should upgradable", async () => {
    const contractV1 = await upgrades.deployProxy(
      await ethers.getContractFactory("RustTokenUpgradeable721_V1", {}),
      ["RustToken", "RT", "hello"]
    );

    await contractV1.mint("A");
    expect(await contractV1.tokenURI(1)).to.equal("ipfs://A");
    expect(await contractV1.name()).to.equal("RustToken");
    expect(await contractV1.version()).to.equal(1);
    expect(await contractV1.message()).to.equal("hello");

    const contractV2 = await upgrades.upgradeProxy(
      contractV1.address,
      await ethers.getContractFactory("RustTokenUpgradeable721_V2"),
      {
        call: {
          fn: "initialize",
          args: ["world"],
        },
      }
    );

    expect(contractV1.address).to.equal(contractV2.address);

    await contractV2.mint("B");
    expect(await contractV2.tokenURI(1)).to.equal("ipfs://A");
    expect(await contractV2.tokenURI(2)).to.equal("ipfs://B");
    expect(await contractV2.message()).to.equal("helloworld");
    expect(await contractV2.name()).to.equal("RustToken");
    expect(await contractV2.version()).to.equal(2);
  });

  it("should mint error when not initalize", async () => {
    const contractV1 = await upgrades.deployProxy(
      await ethers.getContractFactory("RustTokenUpgradeable721_V1", {}),
      ["RustToken", "RT", ""]
    );

    const contractV2 = await upgrades.upgradeProxy(
      contractV1.address,
      await ethers.getContractFactory("RustTokenUpgradeable721_V2")
    );

    expect(contractV2.mint("A")).to.be.revertedWith("not initialized");
  });
});
