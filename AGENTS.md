# Persistent Handoff Protocol

`HANDOFF.md` is the persistent recovery document for this repository.

## At the beginning of every session

1. Read `AGENTS.md` completely.
2. Read `HANDOFF.md` completely.
3. Inspect `git status`, `git diff`, and recent commits.
4. Compare the actual repository state with `HANDOFF.md`.
5. Treat the repository and Git state as the source of truth when they differ.
6. Continue from the `Exact Next Action` section instead of restarting completed work.

## During implementation

Update `HANDOFF.md` after every meaningful milestone, including:

- completing a feature, module, or subphase;
- changing the implementation plan;
- making an architectural decision;
- discovering a blocker;
- modifying several related files;
- running important tests;
- reaching a safe checkpoint before a risky or long-running operation.

## Before stopping

Before ending or pausing work:

1. Save all edited files.
2. Inspect the current Git state.
3. Run relevant safe tests when possible.
4. Update completed, unverified, and in-progress sections.
5. Record actual test results.
6. Record modified and untracked files.
7. Update the prioritized remaining-work checklist.
8. Write one precise `Exact Next Action`.

## Safety rules

- Preserve existing work; never reset, clean, discard, or overwrite changes without explicit approval.
- Do not modify unrelated application code while maintaining handoff documentation.
- Never expose secrets, credentials, tokens, or private certificates in repository documents.
- Never claim an unexecuted test passed or convert an assumption into a fact.
- Do not create commits unless the user explicitly authorizes them.
- When a recurring mistake or incorrect assumption is discovered, record a concise lesson in `D:\All projects\Mistakes\mistakes.md` according to the project instructions.
