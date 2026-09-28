# Copilot Code Review Instructions

When reviewing pull requests in this repository, you MUST adhere to the following workflow for Jira integration:

1. **Identify the Jira Ticket:** Look for a Jira issue key (e.g., `KAN-1`, `PROJ-123`) or Jira URL in the Pull Request title, branch name, or description.
2. **Fetch Ticket Context using MCP:** If a Jira issue key is found, you MUST autonomously call the Atlassian MCP server tools to fetch the issue's details.
   - **CRITICAL:** Do NOT use your built-in `Web fetch` or `browser` tools to fetch Jira URLs. Direct web requests to Jira are blocked by the environment.
   - Instead, you MUST use the Atlassian MCP tool `getTeamworkGraphObject`.
   - Pass the `cloudId` (e.g., `neovancedemoatlassian.atlassian.net`) and the `objects` array containing the full URL or ARI of the Jira ticket.
3. **Analyze Acceptance Criteria:** Read the ticket's description returned by `getTeamworkGraphObject` and extract the core requirements and Acceptance Criteria.
4. **Evaluate the PR:** Perform your code review specifically evaluating whether the code changes satisfy the Acceptance Criteria of the fetched Jira ticket. 
5. **Report:** In your review summary, explicitly mention which Jira ticket you evaluated against, and highlight any acceptance criteria that appear to be missing, incomplete, or incorrectly implemented.
