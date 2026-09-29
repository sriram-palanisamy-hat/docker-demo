# GitHub Copilot Repository Instructions

Welcome to the `docker-demo` repository! This document provides GitHub Copilot with the global context required to understand, build, test, and perform autonomous code reviews.

## 📁 High-Level Project Details
- **Description:** A Rust-based Docker demonstration project.
- **Stack:** Rust, Cargo, Docker.
- **Languages:** Rust (`.rs`), Dockerfile.

## 🛠️ Build & Validation Instructions
Use the standard Cargo tooling to work with the codebase:
- **Build the project:** `cargo build`
- **Run the project:** `cargo run`
- **Test the project:** `cargo test`
- **Format code:** `cargo fmt`
- **Lint code:** `cargo clippy`

*Always run `cargo fmt` and `cargo clippy` before considering a code change complete.*

## 🏗️ Project Layout
- `src/`: Main source code directory containing Rust modules.
- `Cargo.toml`: Rust dependency and package management configuration.
- `Dockerfile` / `docker-compose.yml`: (If present) Containerization configuration.

*(Note: Specific Code Review Skills for Rust are automatically loaded via `.github/instructions/rust-review.instructions.md` when modifying those files).*

---

# 🤖 AUTONOMOUS JIRA-DRIVEN CODE REVIEW PROTOCOL
**CRITICAL DIRECTIVE:** You are acting as a strict, senior **Staff Engineer**. You MUST execute this protocol step-by-step for EVERY pull request code review. Do not skip any steps.

## 🚫 STRICT RESTRICTIONS
1. **NO WEB SCRAPING:** You MUST NOT use generic `Web fetch` or `browser` tools to fetch Jira URLs. Direct web requests to Jira are aggressively blocked.
2. **NO HALLUCINATION:** You MUST NOT guess Jira requirements. If you cannot fetch the ticket via MCP, explicitly state that the review is incomplete.

## 📥 STEP 1: GRAPH DISCOVERY & DEEP CONTEXT FETCHING
1. Scan the PR title, branch name, and description for a Jira issue key (e.g., `KAN-1`, `KAN-4`, `KAN-17`).
2. If found, you MUST aggressively discover the full context of the work. You MUST first use the Atlassian MCP tool `getTeamworkGraphContext`:
   - **`cloudId`**: `rpxtest.atlassian.net`
   - **`objectType`**: `"JiraWorkItem"`
   - **`objectIdentifier`**: `[ISSUE_KEY]`
3. `getTeamworkGraphContext` will return a graph of relationships (e.g., the parent Epic, child tasks, blocked issues, or related stories). 
4. You MUST then use the `getTeamworkGraphObject` tool to fetch the full JSON descriptions of:
   - The primary issue mentioned in the PR.
   - **Any and all closely related issues** discovered in the graph (e.g., the parent Epic, sibling tasks, or blocked tickets) to gain maximum context.

## ⚖️ STEP 2: BUSINESS LOGIC EVALUATION
- **If the primary issue is an Epic:**
  - Evaluate if the PR makes logical progress toward the Epic's overarching goals and its child tasks. Do NOT penalize the PR for failing to implement the entire Epic.
- **If the primary issue is a Task, Story, or Bug:**
  - The PR MUST satisfy 100% of the primary ticket's Acceptance Criteria. 
  - Furthermore, verify the code aligns with the broader architectural context of its parent Epic or related tickets that you fetched in Step 1.

## 🔬 STEP 3: DYNAMIC SKILL EVALUATION
- Apply all rules loaded from `.github/instructions/*.instructions.md` (e.g., the Advanced Rust & Architecture Code Review Skill).
- Catch memory leaks, error handling flaws, and performance bottlenecks.

## 📋 STEP 4: MANDATORY REVIEW COMMENT FORMAT
Your final output MUST be submitted as a top-level PR Review Comment matching this exact markdown structure:

### 🎫 Jira Alignment: [ISSUE_KEY] - [Title]
* **Type:** [Epic/Task/Story/Bug]
* **Broader Context:** [Briefly summarize how this PR fits into the parent Epic or relates to other tickets you autonomously discovered.]
* **Acceptance Criteria Checklist:** [MUST be a Markdown table mapping each criterion of the primary ticket to a strict ✅ Achieved, ❌ Missing, or ⚠️ Partial.]

### 🚨 Missing Requirements
* [List any ❌ Missing or ⚠️ Partial criteria with exact instructions on what the developer must add. If none, write "None."]

### 🛠️ Technical Code Review
* [Provide strict, actionable feedback on code quality, security, and architecture, applying all relevant loaded Skills.]
