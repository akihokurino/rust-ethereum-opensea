import { expect } from "chai";
import { Contract, Signer } from "ethers";
import { ethers, upgrades } from "hardhat";

describe("MetaTransactionWallet", () => {
  let wallet: Contract;
  let nft: Contract;
  let owner: Signer;
  let relay: Signer;
  let destination: Signer;

  beforeEach(async () => {
    [owner, relay, destination] = await ethers.getSigners();

    const MetaTransactionWalletFactory = await ethers.getContractFactory(
      "MetaTransactionWallet"
    );
    wallet = await upgrades.deployProxy(MetaTransactionWalletFactory, [
      await owner.getAddress(),
      await relay.getAddress(),
    ]);

    const MetaTransactionalNftFactory = await ethers.getContractFactory(
      "MetaTransactionalNft721"
    );
    nft = await MetaTransactionalNftFactory.deploy(wallet.address);
    await nft.deployed();

    await owner.sendTransaction({
      to: wallet.address,
      value: ethers.utils.parseEther("1.0"),
    });
  });

  it("should execute meta transaction", async () => {
    const nonce = 0;
    const value = ethers.utils.parseEther("1.0");
    const data = "0x";
    const dist = await destination.getAddress();

    const hash = ethers.utils.solidityKeccak256(
      ["uint256", "address", "uint256", "bytes"],
      [nonce, dist, value, data]
    );
    const message = ethers.utils.arrayify(hash);
    const signature = await owner.signMessage(message);

    const balanceBefore = await ethers.provider.getBalance(dist);

    const tx = await wallet
      .connect(relay)
      .executeMetaTransaction(nonce, dist, value, data, signature);
    await tx.wait();

    const balanceAfter = await ethers.provider.getBalance(dist);

    expect(balanceAfter.sub(balanceBefore)).to.equal(value);
  });

  it("should mint nft through the meta transaction", async () => {
    const nonce = 0;
    const value = 0;
    const to = await owner.getAddress();
    const contentHash = "A";
    const dist = nft.address;

    const data = nft.interface.encodeFunctionData("mint", [to, contentHash]);

    const hash = ethers.utils.solidityKeccak256(
      ["uint256", "address", "uint256", "bytes"],
      [nonce, dist, value, data]
    );
    const message = ethers.utils.arrayify(hash);
    const signature = await owner.signMessage(message);

    const tx = await wallet
      .connect(relay)
      .executeMetaTransaction(nonce, dist, value, data, signature);
    await tx.wait();

    expect(await nft.tokenURI(1)).to.equal("ipfs://A");
  });

  it("should error when mint direct", async function () {
    await expect(nft.mint(await owner.getAddress(), "A")).to.be.revertedWith(
      "not an relayer"
    );
  });
});
