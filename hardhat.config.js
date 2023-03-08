require("@nomiclabs/hardhat-waffle")

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    solidity: {
        compilers: [
            {
                version: "0.8.0",
                settings: {},
            },
            {
                version: "0.8.7",
                settings: {},
            },
            {
                version: "0.7.0",
            },
        ],
    },
    plugins: ["@nomiclabs/hardhat-waffle"],
}
