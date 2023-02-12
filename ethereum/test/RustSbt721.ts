import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("RustSbt721", function () {
  it("should mint and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustSbt721", {}),
      ["RustSbt", "RS"]
    );

    await contract.mint("A");
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/A"
    );
  });

  it("should get is owner", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustSbt721", {}),
      ["RustSbt", "RS"]
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");
    expect(await contract.isOwner(1, owner.address)).to.equal(true);
    expect(await contract.isOwner(1, other.address)).to.equal(false);
  });

  it("should cannot transfer", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("RustSbt721", {}),
      ["RustSbt", "RS"]
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint("A");

    expect(
      contract.transferFrom(owner.address, other.address, 1)
    ).to.be.revertedWith("Err: token is SOUL BOUND");
    expect(
      contract["safeTransferFrom(address,address,uint256)"](
        owner.address,
        other.address,
        1
      )
    ).to.be.revertedWith("Err: token is SOUL BOUND");
  });
});
