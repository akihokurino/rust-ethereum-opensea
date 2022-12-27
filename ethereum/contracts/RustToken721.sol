// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

contract RustToken721 is ERC721Upgradeable, OwnableUpgradeable {
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

    function latestTokenId() public view virtual returns (uint256) {
        return _localTokenId;
    }

    function totalSupply() public view virtual returns (uint256) {
        return _localTokenId;
    }

    function totalOwned() public view virtual returns (uint256) {
        uint256 owned = 0;
        for (uint256 i = 1; i <= _localTokenId; i++) {
            if (ownerOf(i) == owner()) {
                owned += 1;
            }
        }
        return owned;
    }

    function isOwner(uint256 tokenId, address target)
        public
        view
        virtual
        returns (bool)
    {
        return ownerOf(tokenId) == target;
    }
}
