import { expect } from "chai";
import { Contract, ContractFactory, Signer } from "ethers";
import { ethers, upgrades } from "hardhat";

describe("Market", function () {
  let Nft: ContractFactory;
  let Market: ContractFactory;

  let nft: Contract;
  let market: Contract;

  let owner: Signer;
  let seller: Signer;
  let buyer: Signer;

  beforeEach(async () => {
    Nft = await ethers.getContractFactory("Nft721");
    Market = await ethers.getContractFactory("NftMarket");
    [owner, seller, buyer] = await ethers.getSigners();

    nft = await Nft.connect(seller).deploy("NFT", "NT");
    await nft.deployed();

    market = await upgrades.deployProxy(Market.connect(owner), []);
  });

  it("should list sell order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).mint("B");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));
    await market
      .connect(seller)
      .sellOrder(nft.address, 2, ethers.utils.parseEther("1"));

    const orders = await market.getAllSellOrders();
    expect(orders.length).to.equal(2);
    const keys = await market.getSellOrderKeys();
    expect(keys.length).to.equal(2);
    expect(keys[0].toLowerCase()).to.equal(`${nft.address}#1`.toLowerCase());
    expect(keys[1].toLowerCase()).to.equal(`${nft.address}#2`.toLowerCase());

    expect(orders[0].contractAddress).to.equal(nft.address);
    expect(orders[0].tokenId).to.equal(1);
    expect(orders[0].seller).to.equal(await seller.getAddress());
    expect(orders[0].price).to.equal(ethers.utils.parseEther("1"));
    expect(orders[0].tokenURI).to.equal("ipfs://A");

    const order = await market.getSellOrder(nft.address, 2);
    expect(order.contractAddress).to.equal(nft.address);
    expect(order.tokenId).to.equal(2);
    expect(order.seller).to.equal(await seller.getAddress());
    expect(order.price).to.equal(ethers.utils.parseEther("1"));
    expect(order.tokenURI).to.equal("ipfs://B");
  });

  it("should cancel order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).mint("B");
    await nft.connect(seller).mint("C");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));
    await market
      .connect(seller)
      .sellOrder(nft.address, 2, ethers.utils.parseEther("1"));
    await market
      .connect(seller)
      .sellOrder(nft.address, 3, ethers.utils.parseEther("1"));

    await market.connect(seller).cancelOrder(nft.address, 1);

    const orders = await market.getAllSellOrders();
    expect(orders.length).to.equal(2);
    const keys = await market.getSellOrderKeys();
    expect(keys.length).to.equal(2);
    expect(orders[0].contractAddress).to.equal(nft.address);
    expect(orders[0].tokenId).to.equal(3);
    expect(orders[0].seller).to.equal(await seller.getAddress());
    expect(orders[0].price).to.equal(ethers.utils.parseEther("1"));
    expect(orders[0].tokenURI).to.equal("ipfs://C");
  });

  it("should cancel order by admin", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    await market.connect(owner).cancelOrderByAdmin(nft.address, 1);

    const orders = await market.getAllSellOrders();
    expect(orders.length).to.equal(0);
    const keys = await market.getSellOrderKeys();
    expect(keys.length).to.equal(0);
  });

  it("should buy order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).mint("B");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    expect(await nft.ownerOf(1)).to.equal(await seller.getAddress());

    await market
      .connect(buyer)
      .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") });

    const orders = await market.getAllSellOrders();
    expect(orders.length).to.equal(0);
    const keys = await market.getSellOrderKeys();
    expect(keys.length).to.equal(0);

    expect(await nft.ownerOf(1)).to.equal(await buyer.getAddress());
  });

  it("should sell again", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));
    await market
      .connect(buyer)
      .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") });
    await market
      .connect(buyer)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    const orders = await market.getAllSellOrders();
    expect(orders[0].contractAddress).to.equal(nft.address);
    expect(orders[0].tokenId).to.equal(1);
    expect(orders[0].seller).to.equal(await buyer.getAddress());
    expect(orders[0].price).to.equal(ethers.utils.parseEther("1"));
    expect(orders[0].tokenURI).to.equal("ipfs://A");

    expect(await nft.ownerOf(1)).to.equal(await buyer.getAddress());
  });

  it("should error when sell order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);

    await expect(
      market
        .connect(seller)
        .sellOrder(nft.address, 999, ethers.utils.parseEther("1"))
    ).to.be.revertedWith("ERC721: invalid token ID");

    await expect(
      market
        .connect(buyer)
        .sellOrder(nft.address, 1, ethers.utils.parseEther("1"))
    ).to.be.revertedWith("token not owned");

    await expect(
      market
        .connect(seller)
        .sellOrder(nft.address, 1, ethers.utils.parseEther("0"))
    ).to.be.revertedWith("price must be greater than 0");

    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    await expect(
      market
        .connect(seller)
        .sellOrder(nft.address, 1, ethers.utils.parseEther("1"))
    ).to.be.revertedWith("already sell ordered");
  });

  it("should error when cancel order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    await expect(
      market.connect(seller).cancelOrder(nft.address, 999)
    ).to.be.revertedWith("ERC721: invalid token ID");

    await expect(
      market.connect(buyer).cancelOrder(nft.address, 1)
    ).to.be.revertedWith("token not owned");

    await market
      .connect(buyer)
      .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") });

    await expect(
      market.connect(buyer).cancelOrder(nft.address, 1)
    ).to.be.revertedWith("sell order does not exist");
  });

  it("should error when cancel order by admin", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);
    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    await expect(
      market.connect(seller).cancelOrderByAdmin(nft.address, 1)
    ).to.be.rejectedWith("Ownable: caller is not the owner");
  });

  it("should error when buy order", async function () {
    await nft.connect(seller).mint("A");
    await nft.connect(seller).setApprovalForAll(market.address, true);

    await expect(
      market
        .connect(buyer)
        .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") })
    ).to.be.revertedWith("sell order does not exist");

    await market
      .connect(seller)
      .sellOrder(nft.address, 1, ethers.utils.parseEther("1"));

    await expect(
      market
        .connect(seller)
        .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") })
    ).to.be.revertedWith("token owned");

    await nft["safeTransferFrom(address,address,uint256)"](
      await seller.getAddress(),
      await buyer.getAddress(),
      1
    );

    await expect(
      market
        .connect(owner)
        .buyOrder(nft.address, 1, { value: ethers.utils.parseEther("1") })
    ).to.be.revertedWith("seller is not owned");
  });
});
