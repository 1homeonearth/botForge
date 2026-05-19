# BotForge Runtime Architecture

## Gate
- Loads activated platform modules.
- Ingests external events and normalizes to BotForge envelopes.
- Executes approved intents through platform adapters.

## Court
- Validates capabilities and policies.
- Routes events/intents between modules.
- Owns authorization and audit decisions.

## Chamber
- Executes WASM modules.
- Only accepts normalized BotForge events.
- Emits typed intents; cannot call platforms directly.
