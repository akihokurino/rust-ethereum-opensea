// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SampleOracle is ChainlinkClient, Ownable {
    AggregatorV3Interface internal priceFeed;

    using Chainlink for Chainlink.Request;

    struct GetTimeResponse {
        string now;
        int256 timestamp;
    }
    GetTimeResponse public getTimeResponse;

    uint256 public fee;
    address public oracleAddress;
    bytes32 public getTimeJobId;

    /**
     * Network: Goerli
     * Aggregator: ETH/USD
     * Address: 0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
     *
     * Link Token: 0x326C977E6efc84E512bB9C30f76E30c160eD06FB
     * Oracle Address: 0x45585c78a16c62b510E6336fD8B95C61e88039B0
     * GetTime JobId: b51574fb-06e3-4cba-9bb3-596de9f07a64
     */
    constructor() {
        priceFeed = AggregatorV3Interface(
            0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
        );

        setChainlinkToken(0x326C977E6efc84E512bB9C30f76E30c160eD06FB);

        fee = 1 * 10**18;
        oracleAddress = 0x45585c78a16c62b510E6336fD8B95C61e88039B0;
        getTimeJobId = "b51574fb06e34cba9bb3596de9f07a64";
    }

    function getChainlinkToken() public view returns (address) {
        return chainlinkTokenAddress();
    }

    // https://docs.chain.link/docs/consuming-data-feeds/
    function getLatestPrice() public view returns (int256) {
        (, int256 price, , , ) = priceFeed.latestRoundData();
        return price;
    }

    function setGetTimeJobId(bytes32 id) public onlyOwner {
        getTimeJobId = id;
    }

    function createGetTimeRequestTo()
        public
        onlyOwner
        returns (bytes32 requestId)
    {
        Chainlink.Request memory req = buildChainlinkRequest(
            getTimeJobId,
            address(this),
            this.fulfillGetTimeRequest.selector
        );
        req.add("params", "sample time adapter");
        requestId = sendChainlinkRequestTo(oracleAddress, req, fee);
    }

    function fulfillGetTimeRequest(
        bytes32 requestId,
        string memory _now,
        int256 _timestamp
    ) public recordChainlinkFulfillment(requestId) {
        getTimeResponse = GetTimeResponse({now: _now, timestamp: _timestamp});
    }

    function cancelRequest(
        bytes32 requestId,
        bytes4 callbackFunctionId,
        uint256 expiration
    ) public onlyOwner {
        cancelChainlinkRequest(requestId, fee, callbackFunctionId, expiration);
    }

    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
        require(
            link.transfer(msg.sender, link.balanceOf(address(this))),
            "Unable to transfer"
        );
    }
}
