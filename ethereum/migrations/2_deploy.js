const RustToken721 = artifacts.require("RustToken721");
const RustToken1155 = artifacts.require("RustToken1155");

module.exports = function (deployer) {
  deployer.deploy(RustToken721);
  deployer.deploy(RustToken1155);
};
