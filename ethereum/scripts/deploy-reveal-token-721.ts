import { ethers } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

// TODO: Goerli以外のサポート
async function main(hre: HardhatRuntimeEnvironment) {
  const Oracle = await ethers.getContractFactory("Oracle");
  const oracle = await Oracle.deploy();
  await oracle.deployed();
  console.log("Oracle deployed to:", oracle.address);
  await oracle.setAuthorizedSenders([process.env.CHAINLINK_NODE_ACCOUNT!]);

  const chainlinkAddress = "0x326C977E6efc84E512bB9C30f76E30c160eD06FB";
  const timeAdapterJobId = "371ddf3b2f034ee2bfea97ebe6398165";

  const Contract = await ethers.getContractFactory("RevealToken721");
  const contract = await Contract.deploy(
    "RevealToken",
    "RT",
    chainlinkAddress,
    oracle.address,
    timeAdapterJobId
  );
  await contract.deployed();
  console.log("deployed to:", contract.address);

  const receipt = await contract.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt.transactionHash, 5);
  await hre.run("verify:verify", {
    address: contract.address,
    constructorArguments: [
      "RevealToken",
      "RT",
      chainlinkAddress,
      oracle.address,
      timeAdapterJobId,
    ],
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
