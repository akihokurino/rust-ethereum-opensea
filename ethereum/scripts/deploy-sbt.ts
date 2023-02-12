import { ethers, upgrades } from "hardhat";

async function main() {
  const rustSbt721 = await upgrades.deployProxy(
    await ethers.getContractFactory("RustSbt721", {}),
    ["RustSbt", "RS"]
  );
  console.log("RustSbt721 deployed to:", rustSbt721.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
