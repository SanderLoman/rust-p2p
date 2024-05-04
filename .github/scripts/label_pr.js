module.exports = async ({ github, context }) => {
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

    try {
        const prNumber = context.payload.pull_request.number;
        const repo = context.repo;
        const prBody = context.payload.pull_request.body;

        const issueNumber = extractIssueNumber(prBody);
        if (!issueNumber) {
            console.error("No issue reference found in PR description.");
            return;
        }

        const { data: issue } = await github.rest.issues.get({
            owner: repo.owner,
            repo: repo.repo,
            issue_number: parseInt(issueNumber),
        });

        const labelsToAdd = issue.labels.map((label) => label.name).filter(shouldIncludeLabel);

        if (labelsToAdd.length > 0) {
            await github.rest.issues.addLabels({
                owner: repo.owner,
                repo: repo.repo,
                issue_number: prNumber,
                labels: labelsToAdd,
            });
        }
    } catch (error) {
        console.error(`Failed to label PR: ${error.message}`);
        throw error;
    }
};
