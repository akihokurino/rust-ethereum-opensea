pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract RustToken1155 is Context, ERC1155, Ownable {
    mapping(string => uint256) private _name2token;
    mapping(uint256 => string) private _token2name;
    uint256 _localTokenId = 1;
    string _tokenBaseURI = "";

    string public name = "RustToken1155";
    string public symbol = "RT";

    constructor() ERC1155("") {}

    function setTokenBaseURI(string memory baseURI) public onlyOwner {
        require(bytes(baseURI).length > 0, "should set not empty base url");
        _tokenBaseURI = baseURI;
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
                    _tokenBaseURI,
                    "/",
                    tokenName,
                    ".metadata.json"
                )
            );
    }

    function currentSupply() public view virtual returns (uint256) {
        return _localTokenId - 1;
    }
}
