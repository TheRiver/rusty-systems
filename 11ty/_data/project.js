module.exports = function () {

    let base = "https://theriver.github.io/rusty-systems";

    return {
        base,
        environment: process.env.ENVIRONMENT || "development",
        css: process.env.ENVIRONMENT === "production" ? `${base}/css/main.css` : "/css/main.css"
    };
};