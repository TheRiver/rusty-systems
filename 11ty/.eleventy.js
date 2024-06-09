const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");

const project = require("./_data/project")();

const util = require('node:util');
const exec = util.promisify(require('node:child_process').exec);

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(syntaxHighlight);

    eleventyConfig.addPassthroughCopy("images");
    eleventyConfig.addPassthroughCopy("css");
    eleventyConfig.addPassthroughCopy("lsystem/examples");

    // Formats the given name as it would the package name.
    eleventyConfig.addShortcode("name", function (name) {
        return `<em class="name">${name}</em>`;
    });

    // Provides a formatted name for the package
    eleventyConfig.addShortcode("rusty-systems", function () {
        return `<em class="name">Rusty-Systems</em>`;
    });

    eleventyConfig.addFilter("base", function (value) {
        return project.base + value;
    });

    // Provides support for running the lsystem command line tool, like so
    // <img src="{{'./lsystem/examples/fig-1.8.plant' | derive | base }}">
    eleventyConfig.addAsyncFilter("derive", async function (name) {
        console.log("[lsystem] deriving", name);
        const file = name.split("/").at(-1);
        const output = "images/examples/" + file.substring(0, file.lastIndexOf('.')) + ".svg";

        const {err} = await exec(`lsystem derive ${name} --output _site/${output}`);

        if (err) throw new Error("" + err);

        return "/" + output;
    });

};