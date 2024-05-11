const core = require("@actions/core");
const github = require("@actions/github");

function shouldIncludeLabel(label) {
    const excludePatterns = ["LS-", "P-Prevent-Stale", "D-"];
    return !excludePatterns.some((pattern) => label.startsWith(pattern));
}

function extractIssueNumber(body) {
    const keywords = "\\b(close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved)\\b";
    const urlPattern = new RegExp(`${keywords} https://github.com/.*/.*/issues/(\\d+)`, "gi");
    const refPattern = new RegExp(`${keywords} #(\\d+)`, "gi");

    const urlMatch = urlPattern.exec(body);
    if (urlMatch) {
        return urlMatch[1];
    }

    const refMatch = refPattern.exec(body);
    if (refMatch) {
        return refMatch[0].match(/#(\d+)/)[1];
    }

    return null;
}

module.exports = async ({ github, context }) => {
    try {
        const prNumber = context.payload.pull_request.number;
        const prBody = context.payload.pull_request.body;
        const repo = context.repo;

        const issueNumber = extractIssueNumber(prBody);
        if (!issueNumber) {
            console.log("No linked issue found in PR description.");
            return;
        }

        const issue = await github.rest.issues.get({
            owner: repo.owner,
            repo: repo.repo,
            issue_number: issueNumber,
        });

        const labels = issue.data.labels.map((label) => label.name).filter(shouldIncludeLabel);

        if (labels.length > 0) {
            await github.rest.issues.addLabels({
                owner: repo.owner,
                repo: repo.repo,
                issue_number: prNumber,
                labels: labels,
            });
        }
    } catch (error) {
        console.error("Failed to label PR:", error);
        core.setFailed(error.message);
    }
};
