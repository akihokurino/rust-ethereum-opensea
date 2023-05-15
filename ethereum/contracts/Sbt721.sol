// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract Sbt721 is ERC721Enumerable, Ownable {
    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;

    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
        _localTokenId = 0;
    }

    function mint(string memory contentHash) public virtual onlyOwner {
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

    function isOwner(
        uint256 tokenId,
        address target
    ) public view virtual returns (bool) {
        return ownerOf(tokenId) == target;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 firstTokenId,
        uint256 batchSize
    ) internal override {
        require(from == address(0), "Err: token is SOUL BOUND");
        super._beforeTokenTransfer(from, to, firstTokenId, batchSize);
    }
}
