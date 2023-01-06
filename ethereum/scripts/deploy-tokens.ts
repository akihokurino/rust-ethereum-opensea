import { ethers } from "hardhat";

async function main() {
  // const rustToken721 = await upgrades.deployProxy(
  //   await ethers.getContractFactory("RustToken721", {}),
  //   ["RustToken", "RT"]
  // );
  // console.log("RustToken721 deployed to:", rustToken721.address);

  // const rustToken1155 = await upgrades.deployProxy(
  //   await ethers.getContractFactory("RustToken1155", {}),
  //   ["RustToken", "RT"]
  // );
  // console.log("RustToken1155 deployed to:", rustToken1155.address);

  const RevealToken721 = await ethers.getContractFactory("RevealToken721");
  const revealToken721 = await RevealToken721.deploy(
    "RevealToken",
    "RT",
    "0x326C977E6efc84E512bB9C30f76E30c160eD06FB",
    "0x45585c78a16c62b510E6336fD8B95C61e88039B0",
    "371ddf3b2f034ee2bfea97ebe6398165"
  );
  await revealToken721.deployed();
  console.log("RevealToken721 deployed to:", revealToken721.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
