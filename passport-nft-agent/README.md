# Passport NFT Agent

Command-line service that monitors Linera passport contracts and submits achievement updates.

## Modules
- config: layered configuration loader (env + optional file).
- chain_client: thin GraphQL client for passport metadata.
- scoring: deterministic achievement primitives and base score helpers.
- llm: optional OpenAI wrapper returning structured JSON responses.
- updater: signs payloads and prepares contract update operations.

## Usage


Environment variables prefixed with PASSPORT_AGENT__ override config values. To load from file set PASSPORT_AGENT_CONFIG=/path/to/settings.toml.
