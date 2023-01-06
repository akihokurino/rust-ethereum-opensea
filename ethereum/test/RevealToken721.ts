import { expect } from "chai";
import { ethers } from "hardhat";

describe("RevealToken721", function () {
  it("should reveal", async () => {
    const Contract = await ethers.getContractFactory("RevealToken721");
    const contract = await Contract.deploy(
      "Test",
      "T",
      "0x326C977E6efc84E512bB9C30f76E30c160eD06FB",
      "0x45585c78a16c62b510E6336fD8B95C61e88039B0",
      "371ddf3b2f034ee2bfea97ebe6398165"
    );
    await contract.deployed();

    await contract.mint("A");
    expect(await contract.getCurrentHour()).to.equal(-1);
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/QmbJswgamjqeuLoH8eFEaGSCehLNqE4C4E4PRUo5ymLzgg"
    );

    await contract.setTimestampForDebug(Math.floor(1675648800000 / 1000));
    expect(await contract.getCurrentHour()).to.equal(2);
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/QmbJswgamjqeuLoH8eFEaGSCehLNqE4C4E4PRUo5ymLzgg"
    );

    await contract.setTimestampForDebug(Math.floor(1675652400000 / 1000));
    expect(await contract.getCurrentHour()).to.equal(3);
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/A"
    );

    await contract.setTimestampForDebug(Math.floor(1675677600000 / 1000));
    expect(await contract.getCurrentHour()).to.equal(10);
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/QmbJswgamjqeuLoH8eFEaGSCehLNqE4C4E4PRUo5ymLzgg"
    );

    await contract.setTimestampForDebug(Math.floor(1675695600000 / 1000));
    expect(await contract.getCurrentHour()).to.equal(15);
    expect(await contract.tokenURI(1)).to.equal(
      "https://akiho-playground.infura-ipfs.io/ipfs/QmbJswgamjqeuLoH8eFEaGSCehLNqE4C4E4PRUo5ymLzgg"
    );
  });
});
