import { ethers } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

async function main(hre: HardhatRuntimeEnvironment) {
  const Contract = await ethers.getContractFactory("RustSbt721");
  const contract = await Contract.deploy("RustSbt", "RS");
  await contract.deployed();
  console.log("deployed to:", contract.address);

  const receipt = await contract.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt.transactionHash, 5);
  await hre.run("verify:verify", {
    address: contract.address,
    constructorArguments: ["RustSbt", "RS"],
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
