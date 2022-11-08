import { ethers } from "hardhat";

async function main() {
  const RustToken721 = await ethers.getContractFactory("RustToken721");
  const rustToken721 = await RustToken721.deploy();
  await rustToken721.deployed();
  console.log("RustToken721 deployed to:", rustToken721.address);

  const RustToken1155 = await ethers.getContractFactory("RustToken1155");
  const rustToken1155 = await RustToken1155.deploy();
  await rustToken1155.deployed();
  console.log("RustToken1155 deployed to:", rustToken1155.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
