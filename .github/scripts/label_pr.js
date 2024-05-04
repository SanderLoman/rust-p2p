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

    for (const pattern of patterns) {
        const match = body.match(pattern);
        if (match) {
            return match[0].match(/\d+$/)[0];
        }
    }
    return null;
}

async function run() {
    try {
        const token = core.getInput("github-token", { required: true });
        const octokit = github.getOctokit(token);
        const context = github.context;
        const prNumber = context.payload.pull_request.number;
        const repo = context.repo;
        const prBody = context.payload.pull_request.body;

        const issueNumber = extractIssueNumber(prBody);
        if (!issueNumber) {
            core.setFailed("No issue reference found in PR description.");
            return;
        }

        const { data: issue } = await octokit.issues.get({
            owner: repo.owner,
            repo: repo.repo,
            issue_number: issueNumber,
        });

        const labelsToAdd = issue.labels.map((label) => label.name).filter(shouldIncludeLabel);

        if (labelsToAdd.length > 0) {
            await octokit.issues.addLabels({
                owner: repo.owner,
                repo: repo.repo,
                issue_number: prNumber,
                labels: labelsToAdd,
            });
        }
    } catch (error) {
        core.setFailed(`Failed to label PR: ${error.message}`);
        console.error(error);
    }
}

run();
