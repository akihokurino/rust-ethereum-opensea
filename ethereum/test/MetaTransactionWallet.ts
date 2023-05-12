import { expect } from "chai";
import { Contract, Signer } from "ethers";
import { ethers } from "hardhat";

describe("MetaTransactionWallet", () => {
  let wallet: Contract;
  let owner: Signer;
  let relay: Signer;
  let destination: Signer;

  beforeEach(async () => {
    [owner, relay, destination] = await ethers.getSigners();

    const MetaTransactionWalletFactory = await ethers.getContractFactory(
      "MetaTransactionWallet"
    );
    wallet = await MetaTransactionWalletFactory.deploy(
      await owner.getAddress()
    );
    await wallet.deployed();

    await owner.sendTransaction({
      to: wallet.address,
      value: ethers.utils.parseEther("1.0"),
    });
  });

  it("should execute meta transaction", async () => {
    const nonce = 0;
    const value = ethers.utils.parseEther("1.0");
    const data = "0x";

    const hash = ethers.utils.solidityKeccak256(
      ["uint256", "address", "uint256", "bytes"],
      [nonce, await destination.getAddress(), value, data]
    );
    const message = ethers.utils.arrayify(hash);
    const signature = await owner.signMessage(message);

    const balanceBefore = await ethers.provider.getBalance(
      await destination.getAddress()
    );

    const tx = await wallet
      .connect(relay)
      .executeMetaTransaction(
        nonce,
        await destination.getAddress(),
        value,
        data,
        signature
      );
    await tx.wait();

    const balanceAfter = await ethers.provider.getBalance(
      await destination.getAddress()
    );

    expect(balanceAfter.sub(balanceBefore)).to.equal(value);
  });
});
