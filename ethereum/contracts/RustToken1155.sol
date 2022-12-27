// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";

contract RustToken1155 is ERC1155Upgradeable, OwnableUpgradeable {
    mapping(uint256 => string) private _token2hash;
    uint256 private _localTokenId;
    string private _baseUrl;
    uint256 private _totalSupply;

    string public name;
    string public symbol;

    function initialize(string memory _name, string memory _symbol)
        public
        initializer
    {
        __ERC1155_init("");
        __Ownable_init();
        _localTokenId = 0;
        _totalSupply = 0;
        _baseUrl = "https://akiho-playground.infura-ipfs.io/ipfs/";

        name = _name;
        symbol = _symbol;
    }

    function mint(string memory contentHash, uint256 amount)
        public
        virtual
        onlyOwner
    {
        _localTokenId += 1;
        _token2hash[_localTokenId] = contentHash;
        _mint(_msgSender(), _localTokenId, amount, "");
        _totalSupply += amount;
    }

    function setBaseURI(string memory base) public virtual onlyOwner {
        _baseUrl = base;
    }

    function uri(uint256 tokenId)
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
        return _totalSupply;
    }

    function totalOwned() public view virtual returns (uint256) {
        uint256 owned = 0;
        for (uint256 i = 1; i <= _localTokenId; i++) {
            owned += balanceOf(owner(), i);
        }
        return owned;
    }

    function isOwner(uint256 tokenId, address target)
        public
        view
        virtual
        returns (bool)
    {
        return balanceOf(target, tokenId) > 0;
    }
}
