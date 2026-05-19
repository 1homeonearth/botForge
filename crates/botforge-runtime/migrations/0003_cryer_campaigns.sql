CREATE TABLE IF NOT EXISTS cryer_servers (
  server_key TEXT PRIMARY KEY,
  server_name TEXT NOT NULL,
  normalized_name TEXT NOT NULL,
  active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cryer_server_defaults (
  server_key TEXT PRIMARY KEY REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  default_cooldown_days INTEGER NOT NULL,
  require_permanent_invite INTEGER NOT NULL DEFAULT 1,
  default_post_type TEXT NOT NULL,
  default_flair_id TEXT,
  default_flair_text TEXT,
  last_ad_at TEXT,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cryer_subreddit_profiles (
  profile_id TEXT PRIMARY KEY,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  subreddit_key TEXT NOT NULL,
  subreddit_name TEXT NOT NULL,
  post_type TEXT NOT NULL,
  title_template TEXT,
  body_template TEXT,
  url_template TEXT,
  flair_id TEXT,
  flair_text TEXT,
  cooldown_days INTEGER NOT NULL,
  require_permanent_invite INTEGER NOT NULL DEFAULT 1,
  is_active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(server_key, subreddit_key)
);

CREATE TABLE IF NOT EXISTS cryer_campaign_templates (
  template_id TEXT PRIMARY KEY,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  subreddit_key TEXT NOT NULL,
  staging_state TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_by TEXT NOT NULL,
  created_at TEXT NOT NULL,
  promoted_at TEXT
);

CREATE TABLE IF NOT EXISTS cryer_campaign_queue (
  queue_id TEXT PRIMARY KEY,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  subreddit_key TEXT NOT NULL,
  template_id TEXT REFERENCES cryer_campaign_templates(template_id) ON DELETE SET NULL,
  status TEXT NOT NULL,
  scheduled_for TEXT,
  idempotency_key TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cryer_cooldowns (
  cooldown_id TEXT PRIMARY KEY,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  subreddit_key TEXT NOT NULL,
  cooldown_until TEXT NOT NULL,
  reason TEXT,
  created_at TEXT NOT NULL,
  UNIQUE(server_key, subreddit_key)
);

CREATE TABLE IF NOT EXISTS cryer_posted_records (
  record_id TEXT PRIMARY KEY,
  reddit_post_id TEXT NOT NULL,
  subreddit_name TEXT NOT NULL,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  created_time TEXT NOT NULL,
  lifecycle_status TEXT NOT NULL CHECK(lifecycle_status IN ('live','removed','expired')),
  removal_category TEXT,
  removal_checked_at TEXT,
  correlation_id TEXT
);

CREATE TABLE IF NOT EXISTS cryer_schedules (
  schedule_id TEXT PRIMARY KEY,
  server_key TEXT NOT NULL REFERENCES cryer_servers(server_key) ON DELETE CASCADE,
  schedule_type TEXT NOT NULL,
  cron_expr TEXT,
  run_at TEXT,
  timezone TEXT,
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cryer_removal_checks (
  check_id TEXT PRIMARY KEY,
  posted_record_id TEXT NOT NULL REFERENCES cryer_posted_records(record_id) ON DELETE CASCADE,
  checked_at TEXT NOT NULL,
  reddit_status_json TEXT NOT NULL,
  classified_status TEXT NOT NULL,
  reason TEXT
);

CREATE TABLE IF NOT EXISTS cryer_campaign_runs (
  run_id TEXT PRIMARY KEY,
  server_key TEXT,
  run_type TEXT NOT NULL,
  dry_run INTEGER NOT NULL,
  requested_by TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at TEXT NOT NULL,
  completed_at TEXT,
  correlation_id TEXT,
  summary_json TEXT
);

CREATE TABLE IF NOT EXISTS cryer_campaign_results (
  result_id TEXT PRIMARY KEY,
  run_id TEXT NOT NULL REFERENCES cryer_campaign_runs(run_id) ON DELETE CASCADE,
  server_key TEXT,
  subreddit_key TEXT,
  outcome TEXT NOT NULL,
  reddit_post_id TEXT,
  schedule_id TEXT,
  details_json TEXT,
  created_at TEXT NOT NULL
);
