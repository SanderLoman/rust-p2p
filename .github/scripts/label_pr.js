const core = require("@actions/core");
const github = require("@actions/github");

function shouldIncludeLabel(label) {
    const excludePatterns = ["LS-", "P-", "U-"];
    return !excludePatterns.some((pattern) => label.startsWith(pattern));
}

function extractIssueNumber(body) {
    const keywords = "\\b(close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved)\\b";
    const patterns = [
        new RegExp(`${keywords} https://github.com/.*/.*/issues/(\\d+)`, "gi"),
        new RegExp(`${keywords} #\\d+`, "gi"),
    ];
}

module.exports = async ({ github, context }) => {
    try {
    } catch {}
};
