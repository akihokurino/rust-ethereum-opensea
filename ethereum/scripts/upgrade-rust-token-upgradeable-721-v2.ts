import { ethers, upgrades } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

async function main(hre: HardhatRuntimeEnvironment) {
  const address = process.env.RUST_TOKEN_UPGRADEABLE_721_V1_ADDRESS!;

  // await upgrades.forceImport(
  //   address,
  //   await ethers.getContractFactory("RustTokenUpgradeable721_V1")
  // );
  const contract = await upgrades.upgradeProxy(
    address,
    await ethers.getContractFactory("RustTokenUpgradeable721_V2"),
    {
      call: {
        fn: "initialize",
        args: ["world"],
      },
    }
  );
  console.log("deployed to:", contract.address);

  const receipt = await contract.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt.transactionHash, 5);
  await hre.run("verify:verify", {
    address: contract.address,
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
