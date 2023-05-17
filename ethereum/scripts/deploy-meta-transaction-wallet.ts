import { ethers, upgrades } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

async function main(hre: HardhatRuntimeEnvironment) {
  const relayer = ethers.provider.getSigner(1);

  const wallet = await upgrades.deployProxy(
    await ethers.getContractFactory("MetaTransactionWallet", {}),
    [await relayer.getAddress()]
  );
  console.log("deployed wallet to:", wallet.address);

  const receipt1 = await wallet.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt1.transactionHash, 5);
  await hre.run("verify:verify", {
    address: wallet.address,
  });

  const NftContract = await ethers.getContractFactory(
    "MetaTransactionalNft721"
  );
  const nft = await NftContract.deploy(wallet.address);
  await nft.deployed();
  console.log("deployed nft to:", nft.address);

  const receipt2 = await nft.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt2.transactionHash, 5);
  await hre.run("verify:verify", {
    address: nft.address,
    constructorArguments: [wallet.address],
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
