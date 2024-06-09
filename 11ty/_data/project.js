module.exports = function () {
    return {
        environment: process.env.ENVIRONMENT || "development",
        base: "https://theriver.github.io/rusty-systems"
    };
};