import { expect } from "chai";
import { ethers } from "hardhat";

describe("Sbt721", function () {
  it("should mint and get token url", async () => {
    const Contract = await ethers.getContractFactory("Sbt721");
    const contract = await Contract.deploy("RustSbt", "RS");
    await contract.deployed();

    await contract.mint("A");
    expect(await contract.tokenURI(1)).to.equal("ipfs://A");
  });

  it("should get is owner", async () => {
    const Contract = await ethers.getContractFactory("Sbt721");
    const contract = await Contract.deploy("RustSbt", "RS");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");
    expect(await contract.isOwner(1, owner.address)).to.equal(true);
    expect(await contract.isOwner(1, other.address)).to.equal(false);
  });

  it("should cannot transfer", async () => {
    const Contract = await ethers.getContractFactory("Sbt721");
    const contract = await Contract.deploy("RustSbt", "RS");
    await contract.deployed();

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");

    await expect(
      contract.transferFrom(owner.address, other.address, 1)
    ).to.be.revertedWith("Err: token is SOUL BOUND");
    await expect(
      contract["safeTransferFrom(address,address,uint256)"](
        owner.address,
        other.address,
        1
      )
    ).to.be.revertedWith("Err: token is SOUL BOUND");
  });
});
