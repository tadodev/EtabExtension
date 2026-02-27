# ETABS Extension — AI Integration

Architecture, configuration, and development guide for the AI agent layer.

---

## Design Principles

**Privacy first.** Structural engineering models contain proprietary design
data, client information, and commercially sensitive material. The AI layer
is designed so that model data never leaves the user's machine unless they
explicitly configure a cloud provider.

**Local by default.** Ollama with a locally-running model is the recommended
default. No API key, no network traffic, no data sent to third parties.

**Provider-agnostic.** The agent logic is written once against a trait.
Switching from Ollama to Claude to OpenAI is a one-line config change.

**`ext-api` is the gatekeeper.** The agent calls the same functions as the
CLI. The state machine, permission matrix, and all business logic are enforced
in `ext-api` regardless of whether the caller is a human or an AI. The agent
cannot bypass a guard that the CLI cannot bypass.

---

## Crate Structure

```
ext-agent-llm    ← LlmClient trait + provider backends (no ext-api dependency)
ext-agent        ← conversation loop, tools, confirmation gate (calls ext-api)
```

### `ext-agent-llm`

Pure provider abstraction. No knowledge of ETABS, projects, or tools.

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Single-turn: full response when complete.
    async fn chat(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<LlmResponse, LlmError>;

    /// Streaming: calls on_token for each token as it arrives.
    /// Used for long-running tools (analyze, report) in Phase 2.
    async fn chat_streaming(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        on_token: impl Fn(String) + Send + 'static,
    ) -> Result<LlmResponse, LlmError>;
}

pub enum LlmResponse {
    Text(String),
    ToolCall { name: String, input: serde_json::Value },
    ToolCallWithText { text: String, name: String, input: serde_json::Value },
}
```

### `ext-agent`

Owns the conversation loop and all ETABS tool definitions.

```
ext-agent/src/
  lib.rs              AgentSession: new(), chat(), run_interactive()
  tools/
    mod.rs            ToolRegistry: definition + dispatch table
    read.rs           status, log, branch, show, diff, etabs_status, remote_status
    write.rs          commit, checkout, switch, branch_create, push, pull,
                      etabs_open, etabs_close, etabs_recover, etabs_unlock,
                      analyze, report (Phase 2)
  context.rs          system prompt: SKILL.md + live project status JSON
  confirmation.rs     [y/n] gate for write tools; Tauri event gate
  history.rs          Vec<Message> in-memory per session
  suggestion.rs       post-tool next-action hints (Phase 2)
```

---

## Provider Backends

### Phase 1: Claude (cloud, direct HTTP)

Calls the Anthropic Messages API via `reqwest`. No Anthropic SDK needed —
the API is simple enough for a direct implementation.

```
POST https://api.anthropic.com/v1/messages
Headers: x-api-key, anthropic-version: 2023-06-01, content-type
Body: { model, max_tokens, system, messages, tools, stream }
```

Use when: internet is available, privacy is acceptable, best reasoning needed.

### Phase 2: Ollama (local, private — recommended default)

Runs entirely on the user's machine. No data leaves. No API key.
Uses the OpenAI-compatible endpoint that Ollama exposes.

```toml
# config.local.toml
[ai]
provider = "ollama"
model    = "qwen2.5-coder:14b"   # good balance of capability and speed
baseUrl  = "http://localhost:11434/v1"
apiKey   = ""
```

Recommended models for structural engineering assistant use:
- `qwen2.5-coder:14b` — strong reasoning, fits 16GB VRAM
- `llama3.2:latest` — lighter, good for read-only query tasks
- `mistral:latest` — fast responses, lower resource use

### Phase 2: OpenAI-compatible (any endpoint)

The same `async-openai` backend covers OpenAI, Azure OpenAI, LM Studio,
vLLM, or any server that speaks the OpenAI chat completions protocol.

```toml
# OpenAI
[ai]
provider = "openai"
model    = "gpt-4o"
apiKey   = "sk-..."
baseUrl  = ""   # uses OpenAI default

# Azure OpenAI
[ai]
provider = "openai"
model    = "gpt-4o"
apiKey   = "..."
baseUrl  = "https://<resource>.openai.azure.com/openai/deployments/<deployment>"

# LM Studio (local)
[ai]
provider = "openai"
model    = "local-model"
apiKey   = "lm-studio"
baseUrl  = "http://localhost:1234/v1"
```

---

## Configuration

All AI config goes in `config.local.toml` — never in `config.toml`.
`config.toml` is git-tracked and pushed to OneDrive; API keys must
never appear there.

```toml
# config.local.toml — full AI section
[ai]
provider    = "ollama"              # "ollama" | "claude" | "openai"
model       = "qwen2.5-coder:14b"  # model name for the chosen provider
apiKey      = ""                    # required for claude and openai; empty for ollama
baseUrl     = "http://localhost:11434/v1"  # ollama default; change for other endpoints
autoConfirm = false                 # true skips [y/n] prompts (use carefully)
maxTokens   = 4096                  # response token limit
```

Set via the standard config command (automatically routes to `config.local.toml`):

```bash
ext config set ai.provider ollama
ext config set ai.model "qwen2.5-coder:14b"
ext config set ai.baseUrl "http://localhost:11434/v1"
ext config set ai.apiKey "sk-ant-..."   # only for cloud providers
```

---

## `ext chat` — CLI Interface

### Phase 1

Interactive REPL. Starts a session, injects current project status as
context, and loops until the user exits.

```bash
ext chat                           # start session (uses config ai.provider)
ext chat --provider claude         # override provider for this session
ext chat --provider ollama --model qwen2.5-coder:14b
ext chat --no-confirm              # skip [y/n] prompts (caution: executes immediately)
```

Session header shows current state so the user knows the agent's context:

```
ETABS Agent — HighRise Tower
Provider: ollama / qwen2.5-coder:14b (local)
Branch: main · v3 · Modified · ETABS not running
Type your question or instruction. Ctrl+C to exit.

You>
```

### Phase 2 additions

```bash
ext chat --resume                  # load last saved session for this project/branch
ext chat --clear-history           # wipe saved session history
ext chat --non-interactive         # read from stdin, write to stdout (for scripting)
```

Non-interactive mode allows piping:
```bash
echo "what is the current status?" | ext chat --non-interactive --json
```

---

## Tool Execution Flow

Every agent turn follows this sequence:

```
1. User sends message
2. Inject system prompt: SKILL.md + ext_api::status() JSON
3. Send to LLM with full tool list
4. LLM responds with ToolCall or Text
5. If ToolCall:
     a. Is it a write tool? → show action + prompt [y/n]
     b. User confirms → call ext-api function
     c. Wrap result as tool_result message
     d. Send back to LLM for final text response
6. Print final text to user
7. (Phase 2) Run post-tool hook → suggest next action if applicable
```

### Confirmation gate

Write tools always show what will happen before executing:

```
Agent> I'll run: ext commit "Refined beam B45 to W21x93"
       This saves the current working file as v4 on branch main.
       Confirm? [y/n]
```

For `--no-confirm` sessions, the confirmation line is printed but
execution proceeds immediately (useful for trusted automation).

Destructive operations (`checkout` with discard, `branch -d --force`,
`stash drop`) always confirm even with `--no-confirm`.

---

## System Prompt Strategy

The system prompt is built fresh at the start of every turn (not cached),
so the agent always has the latest project state.

```rust
fn build_system_prompt(skill: &str, status: &StatusResult) -> String {
    format!(
        "{skill}\n\n\
         ---\n\
         ## Current Project State\n\
         {status_json}",
        skill = skill,           // SKILL.md embedded via include_str!()
        status_json = serde_json::to_string_pretty(status).unwrap(),
    )
}
```

The injected status JSON gives the LLM:
- Current branch and latest version
- Working file state (CLEAN, MODIFIED, OPEN_CLEAN, etc.)
- Whether ETABS is running
- Whether any stash exists
- OneDrive sync status

This means the user never has to explain the current state — the agent
already knows it before the first message.

---

## Privacy Guarantees

| Data | Sent to cloud provider? |
|---|---|
| Your message text | Yes — when using cloud provider |
| Current project state (status JSON) | Yes — injected as context |
| `.edb` binary model data | **Never** — tools return text summaries only |
| `.e2k` text export (diff output) | Only the diff text, when you ask for diff |
| Parquet analysis results | Only numeric summaries, not raw data |
| `config.local.toml` contents | **Never** — not included in context |
| API key | **Never** — used only for auth header, never in prompt |

With Ollama or LM Studio: nothing leaves the machine. Ever.

**Recommendation for sensitive projects:** Use `ai.provider = "ollama"`.
The capability difference for project management tasks (status, commit,
branch, log) is minimal compared to a cloud model.

---

## Adding a New Tool — Step Order

1. Implement the `ext-api` function if it doesn't exist (follow the
   standard adding-a-command steps in `agents.md`)
2. Add the tool definition in `ext-agent/src/tools/read.rs` or `write.rs`:
   ```rust
   ToolDefinition {
       name: "my_tool",
       description: "Clear description for the LLM — be specific",
       input_schema: json!({
           "type": "object",
           "properties": {
               "version": { "type": "string", "description": "e.g. v3 or main/v3" }
           },
           "required": ["version"]
       }),
   }
   ```
3. Add dispatch arm in `ToolRegistry::dispatch()`:
   ```rust
   "my_tool" => self.my_tool(input).await,
   ```
4. Implement the handler — call `ext-api`, return `serde_json::Value`
5. If write tool: add to `WRITE_TOOLS` constant for confirmation gate
6. Update this document with the new tool in the tool surface table

---

## Phase Rollout

### Phase 1 (Week 9–10, alongside Reports + Remote)

- `ext-agent-llm`: `LlmClient` trait + Claude backend only
- `ext-agent`: all read tools + write tools (with confirmation gate)
- `ext chat` CLI subcommand (interactive REPL, rustyline)
- Ollama support stubbed in config but backend not yet implemented

Tools available in Phase 1:
```
READ:   project_status, list_versions, show_version, list_branches,
        diff_versions, etabs_status, remote_status, config_list
WRITE:  commit_version, create_branch, switch_branch, checkout_version,
        stash_save, stash_pop, etabs_open, etabs_close, etabs_recover,
        push, pull
DEFER:  analyze_version, generate_report, etabs_unlock
        (deferred: need streaming UI before exposing 2–5 min operations)
```

### Phase 2

- `ext-agent-llm`: Ollama + OpenAI-compat backend via `async-openai`
- Streaming responses (`chat_streaming`) for long-running tools
- `analyze_version` and `generate_report` tools unlocked
- Tauri streaming chat panel
- Post-tool suggestions (`suggestion.rs`)
- `ext chat --resume` / `--clear-history` (session persistence in ext-db)
- `ext chat --non-interactive` (stdin/stdout for scripting)

### Phase 3 (if needed)

- ACP (Agent Communication Protocol) adapter over `AgentSession`
  — only if external systems need to call our agent programmatically
- Multi-agent: review agent, CI agent, etc.