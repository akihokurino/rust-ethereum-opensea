// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@chainlink/contracts/src/v0.8/ChainlinkClient.sol";
import "@quant-finance/solidity-datetime/contracts/DateTime.sol";

// https://forum.openzeppelin.com/t/how-to-make-a-contract-inheriting-chalinkclient-uups-upgradable/20670
// UpgradableとChainlinkClientは現状共存できないっぽい
contract RevealToken721 is ERC721Enumerable, Ownable, ChainlinkClient {
    using Chainlink for Chainlink.Request;

    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;
    string private _baseUrl;
    struct TimeAdapterResponse {
        string now;
        uint256 timestamp;
    }
    TimeAdapterResponse public timeAdapterResponse;
    uint256 public chainlinkFee;
    address public oracleAddress;
    bytes32 public timeAdapterJobId;

    /**
     * Network: Goerli
     * Chainlink Address: 0x326C977E6efc84E512bB9C30f76E30c160eD06FB
     * Oracle Address: 0x45585c78a16c62b510E6336fD8B95C61e88039B0
     * TimeAdapter JobId: 371ddf3b-2f03-4ee2-bfea-97ebe6398165（セットするときはハイフンなし）
     */
    constructor(
        string memory name,
        string memory symbol,
        address chainlinkAddress,
        address oracle,
        string memory jobId
    ) ERC721(name, symbol) {
        _localTokenId = 0;
        _baseUrl = "https://akiho-playground.infura-ipfs.io/ipfs/";

        setChainlinkToken(chainlinkAddress);
        chainlinkFee = 1 * 10**18;
        oracleAddress = oracle;
        timeAdapterJobId = stringToBytes32(jobId);
    }

    function stringToBytes32(string memory source)
        public
        pure
        returns (bytes32 result)
    {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }

    function getChainlinkTokenString() public view returns (address) {
        return chainlinkTokenAddress();
    }

    function setChainlinkTokenString(address _address) public onlyOwner {
        setChainlinkToken(_address);
    }

    function setChainlinkFee(uint256 fee) public onlyOwner {
        chainlinkFee = fee;
    }

    function setOracleAddress(address oracle) public onlyOwner {
        oracleAddress = oracle;
    }

    function setTimeAdapterJobId(string memory jobId) public onlyOwner {
        timeAdapterJobId = stringToBytes32(jobId);
    }

    function mint(string memory contentHash) public virtual onlyOwner {
        _localTokenId += 1;
        _token2hash[_localTokenId] = contentHash;
        _mint(_msgSender(), _localTokenId);
    }

    function setBaseURI(string memory base) public virtual onlyOwner {
        _baseUrl = base;
    }

    function tokenURI(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        if (timeAdapterResponse.timestamp > 0) {
            uint256 hour = DateTime.getHour(timeAdapterResponse.timestamp);
            if (3 <= hour && hour < 10) {
                string memory contentHash = _token2hash[tokenId];
                return string(abi.encodePacked(_baseUrl, contentHash));
            }
        }

        return
            string(
                abi.encodePacked(
                    _baseUrl,
                    "QmbJswgamjqeuLoH8eFEaGSCehLNqE4C4E4PRUo5ymLzgg"
                )
            );
    }

    function updateTime() public onlyOwner returns (bytes32 requestId) {
        Chainlink.Request memory req = buildChainlinkRequest(
            timeAdapterJobId,
            address(this),
            this.fulfillUpdateTime.selector
        );
        req.add("params", "sample time adapter");
        requestId = sendChainlinkRequestTo(oracleAddress, req, chainlinkFee);
    }

    function fulfillUpdateTime(
        bytes32 requestId,
        string memory _now,
        uint256 timestamp
    ) public recordChainlinkFulfillment(requestId) {
        timeAdapterResponse = TimeAdapterResponse({
            now: _now,
            timestamp: timestamp
        });
    }

    function cancelRequest(
        bytes32 requestId,
        bytes4 callbackFunctionId,
        uint256 expiration
    ) public onlyOwner {
        cancelChainlinkRequest(
            requestId,
            chainlinkFee,
            callbackFunctionId,
            expiration
        );
    }

    function withdrawLink() public onlyOwner {
        LinkTokenInterface link = LinkTokenInterface(chainlinkTokenAddress());
        require(
            link.transfer(msg.sender, link.balanceOf(address(this))),
            "Unable to transfer"
        );
    }

    function getCurrentHour() public view returns (int256) {
        if (timeAdapterResponse.timestamp > 0) {
            uint256 hour = DateTime.getHour(timeAdapterResponse.timestamp);
            return int256(hour);
        }

        return -1;
    }

    function setTimestampForDebug(uint256 timestamp) public onlyOwner {
        timeAdapterResponse = TimeAdapterResponse({
            now: "",
            timestamp: timestamp
        });
    }
}
