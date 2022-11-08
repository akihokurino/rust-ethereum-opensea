// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract RustToken721 is ERC721Enumerable, Ownable {
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;
    uint256 _localTokenId = 1;
    string _tokenBaseURI = "";

    constructor() ERC721("RustToken721", "RT") {}

    function setTokenBaseURI(string memory baseURI) public onlyOwner {
        require(bytes(baseURI).length > 0, "should set not empty base url");
        _tokenBaseURI = baseURI;
    }

    function mint(address to, string memory tokenName)
        public
        virtual
        onlyOwner
    {
        require(_name2token[tokenName] == 0, "already mint");

        uint256 tokenId = _localTokenId;
        _name2token[tokenName] = tokenId;
        _token2name[tokenId] = tokenName;
        _mint(to, tokenId);

        _localTokenId += 1;
    }

    function tokenBaseURI() public view virtual returns (string memory) {
        return _tokenBaseURI;
    }

    function tokenURI(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        string memory tokenName = _token2name[tokenId];
        return string(abi.encodePacked(_tokenBaseURI, tokenName));
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
