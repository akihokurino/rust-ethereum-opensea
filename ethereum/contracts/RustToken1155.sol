// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";

contract RustToken1155 is ERC1155Upgradeable, OwnableUpgradeable {
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;
    uint256 _localTokenId;

    string public name;
    string public symbol;

    function initialize() public initializer {
        __ERC1155_init("");
        __Ownable_init();
        _localTokenId = 1;
        name = "RustToken1155";
        symbol = "RT";
    }

    function mint(
        address to,
        string memory tokenName,
        uint256 amount
    ) public virtual onlyOwner {
        require(_name2token[tokenName] == 0, "already mint");

        uint256 tokenId = _localTokenId;
        _name2token[tokenName] = tokenId;
        _token2name[tokenId] = tokenName;
        _mint(to, tokenId, amount, "");

        _localTokenId += 1;
    }

    function mintBatch(
        address to,
        string[] memory tokenNames,
        uint256[] memory amounts
    ) public {
        require(tokenNames.length <= 10);

        uint256[] memory tokenIds = new uint256[](tokenNames.length);
        for (uint256 i = 0; i < tokenNames.length; i++) {
            string memory tokenName = tokenNames[i];
            require(_name2token[tokenName] == 0, "already mint");

            uint256 tokenId = _localTokenId;
            _name2token[tokenName] = tokenId;
            _token2name[tokenId] = tokenName;
            tokenIds[i] = tokenId;

            _localTokenId += 1;
        }

        _mintBatch(to, tokenIds, amounts, "");
    }

    function uri(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        string memory tokenName = _token2name[tokenId];
        return
            string(
                abi.encodePacked(
                    "https://ipfs.moralis.io:2053/ipfs/",
                    tokenName
                )
            );
    }

    function currentSupply() public view virtual returns (uint256) {
        return _localTokenId - 1;
    }

    function usedTokenNames() public view virtual returns (string[] memory) {
        if (_localTokenId == 1) {
            return new string[](0);
        }
        uint256 len = _localTokenId - 1;
        string[] memory names = new string[](len);
        for (uint256 i = 0; i < len; i++) {
            names[i] = _token2name[i + 1];
        }
        return names;
    }
}
