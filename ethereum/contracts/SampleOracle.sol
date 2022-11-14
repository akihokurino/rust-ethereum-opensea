// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SampleOracle is ChainlinkClient, Ownable {
    AggregatorV3Interface internal priceFeed;

    using Chainlink for Chainlink.Request;

    struct TimeResponse {
        string raw;
    }
    TimeResponse public timeResponse;

    /**
     * Network: Goerli
     * Aggregator: ETH/USD
     * Address: 0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
     *
     * Link Token: 0x326C977E6efc84E512bB9C30f76E30c160eD06FB
     */
    constructor() {
        priceFeed = AggregatorV3Interface(
            0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e
        );

        setChainlinkToken(0x326C977E6efc84E512bB9C30f76E30c160eD06FB);
    }

    function getChainlinkToken() public view returns (address) {
        return chainlinkTokenAddress();
    }

    // https://docs.chain.link/docs/consuming-data-feeds/
    function getLatestPrice() public view returns (int256) {
        (, int256 price, , , ) = priceFeed.latestRoundData();
        return price;
    }

    function createTimeRequestTo(
        address oracle,
        bytes32 jobId,
        uint256 payment
    ) public onlyOwner returns (bytes32 requestId) {
        Chainlink.Request memory req = buildChainlinkRequest(
            jobId,
            address(this),
            this.fulfillTimeRequest.selector
        );
        req.add("params", "sample time adapter");
        requestId = sendChainlinkRequestTo(oracle, req, payment);
    }

    function fulfillTimeRequest(bytes32 requestId, string calldata resp)
        public
        recordChainlinkFulfillment(requestId)
    {
        timeResponse = TimeResponse({raw: resp});
    }

    function cancelRequest(
        bytes32 _requestId,
        uint256 _payment,
        bytes4 _callbackFunctionId,
        uint256 _expiration
    ) public onlyOwner {
        cancelChainlinkRequest(
            _requestId,
            _payment,
            _callbackFunctionId,
            _expiration
        );
    }

    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
        require(
            link.transfer(msg.sender, link.balanceOf(address(this))),
            "Unable to transfer"
        );
    }
}
