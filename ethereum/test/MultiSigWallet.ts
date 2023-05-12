import { expect } from "chai";
import { Contract, Signer } from "ethers";
import { ethers } from "hardhat";

describe("MultiSigWallet", () => {
  let accounts: Signer[];
  let wallet: Contract;

  beforeEach(async () => {
    accounts = await ethers.getSigners();

    const MultiSigWallet = await ethers.getContractFactory("MultiSigWallet");
    wallet = await MultiSigWallet.deploy(
      [await accounts[0].getAddress(), await accounts[1].getAddress()],
      2
    );
    await wallet.deployed();

    await accounts[0].sendTransaction({
      to: wallet.address,
      value: ethers.utils.parseEther("1.0"),
    });
  });

  it("should transfer Ether to the receiver", async () => {
    const receiver = accounts[2];
    const initialBalance = await ethers.provider.getBalance(
      await receiver.getAddress()
    );

    await wallet
      .connect(accounts[0])
      .submitTransaction(
        await receiver.getAddress(),
        ethers.utils.parseEther("0.1"),
        "0x"
      );
    await wallet.connect(accounts[1]).confirmTransaction(0);

    const finalBalance = await ethers.provider.getBalance(
      await receiver.getAddress()
    );
    expect(finalBalance).to.equal(
      initialBalance.add(ethers.utils.parseEther("0.1"))
    );
  });
});
