

# Demo Playbook v0.3

<!--
This playbook demonstrates example ADL agent workflows and interactions for v0.3.
It is intended for use in demos, testing, and design reference.
-->

## Goals
- Show realistic agent conversations, planning, and execution
- Include new v0.3 features (concurrency, improved error handling, etc.)
- Support copy-paste into CLI or test harness

## Example Scenarios

### 1. Simple Plan/Execute
```adl
agent:
  name: DemoBot
  model: openai/gpt-4
plan:
  - id: step1
    prompt: |
      Say hello and summarize the user's request.
execute:
  - step: step1
```

### 2. Concurrent Steps (v0.3)
```adl
plan:
  - id: a
    prompt: Calculate the sum of 2 and 2.
  - id: b
    prompt: Calculate the product of 3 and 5.
  - id: c
    prompt: What is the result of step a plus step b?
    depends_on: [a, b]
execute:
  - step: a
    concurrent: true
  - step: b
    concurrent: true
  - step: c
```

### 3. Error Handling Example
```adl
plan:
  - id: x
    prompt: Generate invalid JSON.
    output_format: json
execute:
  - step: x
    on_error: retry
```

### 4. Tool Use (function calling)
```adl
agent:
  name: ToolsBot
  tools:
    - name: calculator
      description: Basic arithmetic operations
plan:
  - id: calc
    prompt: Use the calculator tool to add 7 and 8.
    tool: calculator
    tool_input:
      operation: add
      a: 7
      b: 8
execute:
  - step: calc
```

### 5. Multi-Agent Coordination
```adl
agents:
  - name: Planner
    model: openai/gpt-4
  - name: Executor
    model: openai/gpt-4
plan:
  - id: plan
    agent: Planner
    prompt: Create a three-step plan for making tea.
  - id: execute
    agent: Executor
    prompt: Carry out the plan from step 'plan'.
    depends_on: [plan]
execute:
  - step: plan
  - step: execute
```
