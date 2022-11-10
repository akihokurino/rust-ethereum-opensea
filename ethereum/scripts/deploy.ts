import { ethers, upgrades } from "hardhat";

async function main() {
  const rustToken721 = await upgrades.deployProxy(
    await ethers.getContractFactory("RustToken721", {}),
    []
  );
  console.log("RustToken721 deployed to:", rustToken721.address);

  const rustToken1155 = await upgrades.deployProxy(
    await ethers.getContractFactory("RustToken1155", {}),
    []
  );
  console.log("RustToken1155 deployed to:", rustToken1155.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
