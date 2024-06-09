module.exports = function () {

    let base = "https://theriver.github.io/rusty-systems";
    const environment = process.env.ENVIRONMENT || "development";

    return {
        environment,
        base:   environment === "production" ? base : '',
        css:    environment === "production" ? `${base}/css/main.css` : "/css/main.css"
    };
};