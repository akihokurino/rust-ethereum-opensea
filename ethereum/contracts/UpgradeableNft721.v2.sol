// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

contract UpgradeableNft721_V2 is ERC721Upgradeable, OwnableUpgradeable {
    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;
    string private _messageForV1;
    string private _messageForV2;

    modifier initialized() {
        require(version() == 2, "not initialized");
        _;
    }

    function initialize(string memory _message) public reinitializer(2) {
        _messageForV2 = _message;
    }

    function version() public view virtual returns (uint256) {
        return _getInitializedVersion();
    }

    function mint(
        string memory contentHash
    ) public virtual onlyOwner initialized {
        _localTokenId += 1;
        _token2hash[_localTokenId] = contentHash;
        _mint(_msgSender(), _localTokenId);
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        string memory contentHash = _token2hash[tokenId];
        return string(abi.encodePacked("ipfs://", contentHash));
    }

    function message() public view virtual returns (string memory) {
        return string(abi.encodePacked(_messageForV1, _messageForV2));
    }
}
