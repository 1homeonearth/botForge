# Platform Modules Contract and Registry

Runtime IDs are short lowercase names: discord, reddit, spotify, youtube, rss, webhook, telegram, matrix, mattermost, session, email.

Platform modules are Rust-owned boundaries that:
- expose manifests, setup schema, capabilities, events, intents
- normalize raw platform events before Court
- execute intents after Court authorization
- use BotForge secret references only
- may use raw network only when manifest allows

Registry lifecycle: discover -> validate manifest -> build/test -> capability review -> setup completion -> stage -> activate/suspend/deactivate.
Dependency report is required before capability removal.
