# Architecture process

## Purpose

This project is built in small steps. The goal is to understand the system before choosing code structures.

## Working agreement

- The human owns product goals and architecture decisions.
- The agent explains the current model and proposes designs.
- No implementation starts before the design is signed off.
- Each decision records its reason and its open questions.
- Hardware facts come from the board and official documentation, not guesses.

## Session flow

### 1. Learn

We answer one focused question. Examples:

- What can the board do?
- What does the display need from the application?
- Where does usage data come from?
- What happens when Wi-Fi is unavailable?

### 2. Model with C4

We update the C4 model at the level needed for the question:

- System context: people and external systems.
- Containers: major deployable or separately running parts.
- Components: important parts inside one container.
- Code: only when implementation detail is needed.

The model describes the intended system. It is not a code dump.

### 3. Propose program design

After the model is clear, the agent proposes a program design. It must explain:

- responsibilities
- data flow
- state and failure handling
- hardware boundaries
- test boundaries
- important alternatives and why they were rejected

### 4. Sign off

The human reviews the proposal. The design is either changed or marked **signed off**. Only signed-off designs may lead to implementation work.

### 5. Implement

Implementation follows the signed-off design in small slices. A slice should leave the repository understandable and testable.

## Decision record template

For each important decision, record:

- **Decision:** what we chose.
- **Reason:** what problem it solves.
- **Evidence:** hardware or product facts supporting it.
- **Trade-off:** what becomes harder.
- **Status:** proposed, signed off, or replaced.
