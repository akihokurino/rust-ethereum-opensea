// https://github.com/dea-sg/meta-tx/blob/main/test/forward.ts
import {
  SignTypedDataVersion,
  TypedMessage,
  signTypedData,
} from "@metamask/eth-sig-util";
import { expect } from "chai";
import { toBuffer } from "ethereumjs-util";
import { Contract, Signer, Wallet } from "ethers";
import { ethers, upgrades } from "hardhat";

interface MessageTypeProperty {
  name: string;
  type: string;
}

interface MessageTypes {
  EIP712Domain: MessageTypeProperty[];
  [additionalProperties: string]: MessageTypeProperty[];
}

type Message = {
  from: string;
  to: string;
  value: number;
  gas: number;
  nonce: number;
  data: string;
};

const getChainId = async (): Promise<number> => {
  const network = await ethers.provider.getNetwork();
  const { chainId } = network;
  return chainId;
};

const createMessageParam = async (
  message: Message,
  forwarderAddress: string
): Promise<TypedMessage<MessageTypes>> => {
  const chainId = await getChainId();
  return {
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      ForwardRequest: [
        { name: "from", type: "address" },
        { name: "to", type: "address" },
        { name: "value", type: "uint256" },
        { name: "gas", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "data", type: "bytes" },
      ],
    },
    primaryType: "ForwardRequest",
    domain: {
      name: "MetaTransactionWallet",
      version: "0.0.1",
      chainId,
      verifyingContract: forwarderAddress,
    },
    message,
  } as TypedMessage<MessageTypes>;
};

describe("MetaTransactionWallet", () => {
  let wallet: Contract;
  let nft: Contract;
  let relay: Signer;
  let destination: Signer;
  let userWallet: Wallet;

  beforeEach(async () => {
    [relay, destination] = await ethers.getSigners();
    userWallet = Wallet.createRandom();

    const MetaTransactionWalletFactory = await ethers.getContractFactory(
      "MetaTransactionWallet"
    );
    wallet = await upgrades.deployProxy(MetaTransactionWalletFactory, [
      await relay.getAddress(),
    ]);

    const MetaTransactionalNftFactory = await ethers.getContractFactory(
      "MetaTransactionalNft721"
    );
    nft = await MetaTransactionalNftFactory.deploy(wallet.address);
    await nft.deployed();

    await relay.sendTransaction({
      to: wallet.address,
      value: ethers.utils.parseEther("1.0"),
    });
  });

  it("should execute meta transaction", async () => {
    const value = 1000000000;
    const dist = await destination.getAddress();
    const nonce = await wallet.getNonce(userWallet.address);

    const message: Message = {
      from: userWallet.address,
      to: dist,
      value: value,
      gas: 3000000,
      nonce: nonce.toNumber(),
      data: "0x",
    };

    const msgParams = await createMessageParam(message, wallet.address);
    const signature = signTypedData({
      privateKey: toBuffer(userWallet.privateKey),
      data: msgParams,
      version: SignTypedDataVersion.V4,
    });

    const balanceBefore = await ethers.provider.getBalance(dist);

    const tx = await wallet.execute(message, signature);
    await tx.wait();

    const balanceAfter = await ethers.provider.getBalance(dist);

    expect(balanceAfter.sub(balanceBefore)).to.equal(value);
  });

  it("should mint nft through the meta transaction", async () => {
    const nonce = await wallet.getNonce(userWallet.address);
    const contentHash = "A";

    const data = nft.interface.encodeFunctionData("mint", [
      await destination.getAddress(),
      contentHash,
    ]);

    console.log(`from: ${userWallet.address}`);
    const message: Message = {
      from: userWallet.address,
      to: nft.address,
      value: 0,
      gas: 3000000,
      nonce: nonce.toNumber(),
      data,
    };

    const msgParams = await createMessageParam(message, wallet.address);
    const signature = signTypedData({
      privateKey: toBuffer(userWallet.privateKey),
      data: msgParams,
      version: SignTypedDataVersion.V4,
    });

    const tx = await wallet.connect(relay).execute(message, signature);
    await tx.wait();

    expect(await nft.tokenURI(1)).to.equal("ipfs://A");
  });

  it("should error when mint direct", async function () {
    await expect(nft.mint(await relay.getAddress(), "A")).to.be.revertedWith(
      "not an relayer"
    );
  });
});
