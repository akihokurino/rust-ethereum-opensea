import { ethers } from "hardhat";

async function main() {
  const SampleOracle = await ethers.getContractFactory("SampleOracle");
  const sampleOracle = await SampleOracle.deploy();
  await sampleOracle.deployed();
  console.log("SampleOracle deployed to:", sampleOracle.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
