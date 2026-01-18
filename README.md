# Agentic Workflow Showcase with Claude Code
This repository contains structured exercises designed to demonstrate the shift from "Autocomplete Coding" to "Agentic Engineering" using Claude Code.

## The Philosophy: Analyze -> Plan -> Approve
In this repo, we do not treat the AI as a chatbot. We treat it as a **Agentic Coding Partner**.
Every exercise is designed to fail initially, requiring the Agent to use **Plan Mode** to do the following:
1. **Analyze** logs and cross-file dependencies.
2. **Propose** a strategic plan.
3. **Execute** only after human architectural approval.

## The Evolution of AI Coding

We have shifted from simple "assistance" to full "engineering partnership." This is how the workflow has evolved over the years:

*   **1990s – 2020 | Statistical Era (IntelliSense):** Simple pattern matching. It finishes variable names but lacks any understanding of logic.
*   **2021 – 2023 | Predictive Era (Autocomplete):** Predicting the next line of code. Fast, but context-blind, often leading to "correct-looking" bugs.
*   **2023 – 2024 | Junior Era (Simple Chat):** Helpful for snippets and explanations, but impulsive. It focuses on the immediate chat request while skipping the system-wide plan.
*   **2025 – 2026 | Senior Era (Agentic Partner):** System-wide reasoning using **Plan Mode**. It analyzes the whole project and proposes a strategy before touching a single file.

## Why "Plan Mode" is the Secret Weapon

Plan Mode decouples Thinking from Doing, shifting the AI from an impulsive "Chatbot" to a strategic Engineering Partner.

*   **Measure Twice, Cut Once:** Scans the entire repo to identify root causes before touching a single line of code.
*   **Human as Architect:** You audit a technical blueprint rather than cleaning up an AI-generated mess.
*   **Context Discovery:** Eliminates "guessing" by forcing the agent to explore file dependencies and project structure first.
*   **Efficiency:** 1 minute of planning prevents 20 minutes of reverting a broken build.

## Comparison: Autocomplete vs. Plan Mode

| Aspect | Autocomplete (Old) | Plan Mode (Agentic) |
| :--- | :--- | :--- |
| **Logic** | Impulsive (Guess & Write) | Strategic (Analyze & Propose) |
| **Human Role** | Janitor (Cleaning up) | Architect (Directing) |
| **Outcome** | Trial and Error | Right-First-Time |

## Exercises Roadmap

The curriculum is designed to move from local compiler issues to complex system-wide architectural failures.


## News

- **2026-01-18**: [GitHub Copilot now supports OpenCode](https://github.blog/changelog/2026-01-16-github-copilot-now-supports-opencode/)
- **2026-01-16**: [Ollama: Claude Code with Anthropic API compatibility](https://ollama.com/blog/claude)
- **2026-01-10**: [Code Is Cheap Now. Software Isn’t.](https://www.chrisgregori.dev/opinion/code-is-cheap-now-software-isnt)
- **2026-01-09**: [OpenCode no longer works with the Claude Max plan](https://github.com/anthropics/claude-code/issues/17118)


## References


- [Z.ai GLM Coding Plan](https://z.ai/subscribe?ic=7ZJ0PPEBSA)
- [diet103/claude-code-infrastructure-showcase](https://github.com/diet103/claude-code-infrastructure-showcase)
- [vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills)
- [Claude Code Documentation](https://claude.ai/docs/code)
