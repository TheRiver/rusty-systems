const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");

const project = require("./_data/project")();

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(syntaxHighlight);

    eleventyConfig.addPassthroughCopy("images");
    eleventyConfig.addPassthroughCopy("css");

    eleventyConfig.addShortcode("name", function(name) {
        return `<em class="name">${name}</em>`;
    });

    eleventyConfig.addShortcode("rusty-systems", function() {
        return `<em class="name">Rusty-Systems</em>`;
    });

    eleventyConfig.addFilter("base", function(value) {
        return project.base + value;
    });

};