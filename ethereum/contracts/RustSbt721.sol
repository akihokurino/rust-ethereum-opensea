// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

contract RustSbt721 is ERC721Upgradeable, OwnableUpgradeable {
    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;
    string private _baseUrl;

    function initialize(string memory name, string memory symbol)
        public
        initializer
    {
        __ERC721_init(name, symbol);
        __Ownable_init();
        _localTokenId = 0;
        _baseUrl = "https://akiho-playground.infura-ipfs.io/ipfs/";
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
        string memory contentHash = _token2hash[tokenId];
        return string(abi.encodePacked(_baseUrl, contentHash));
    }

    function isOwner(uint256 tokenId, address target)
        public
        view
        virtual
        returns (bool)
    {
        return ownerOf(tokenId) == target;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId,
        uint256 batchSize
    ) internal override {
        require(from == address(0), "Err: token is SOUL BOUND");
        super._beforeTokenTransfer(from, to, tokenId, batchSize);
    }
}
