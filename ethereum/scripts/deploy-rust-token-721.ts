import { ethers } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

async function main(hre: HardhatRuntimeEnvironment) {
  const Contract = await ethers.getContractFactory("RustToken721");
  const contract = await Contract.deploy("RustToken", "RT");
  await contract.deployed();
  console.log("deployed to:", contract.address);

  const receipt = await contract.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt.transactionHash, 5);
  await hre.run("verify:verify", {
    address: contract.address,
    constructorArguments: ["RustToken", "RT"],
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
