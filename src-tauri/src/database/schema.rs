pub const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS orders (
  code TEXT PRIMARY KEY,
  username TEXT,
  name_product TEXT,
  quantity INTEGER,
  price REAL,
  time_buy TEXT,
  time_dau_ky TEXT,
  time_cuoi_ky TEXT,
  time_con_lai TEXT,
  renewal TEXT,
  note TEXT,
  proxy_type TEXT,
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS proxies (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  order_code TEXT NOT NULL REFERENCES orders(code) ON DELETE CASCADE,
  raw_proxy TEXT NOT NULL,
  raw_proxy_ip TEXT,
  host TEXT,
  port INTEGER,
  UNIQUE(order_code, raw_proxy)
);

CREATE TABLE IF NOT EXISTS profiles_cache (
  id TEXT NOT NULL,
  manager TEXT NOT NULL,
  name TEXT,
  raw_proxy TEXT,
  host TEXT,
  port INTEGER,
  group_id TEXT,
  updated_at TEXT DEFAULT (datetime('now')),
  PRIMARY KEY (manager, id)
);

CREATE TABLE IF NOT EXISTS groups_cache (
  id TEXT NOT NULL,
  manager TEXT NOT NULL,
  name TEXT,
  PRIMARY KEY (manager, id)
);

CREATE TABLE IF NOT EXISTS match_results (
  proxy_host TEXT NOT NULL,
  proxy_port INTEGER NOT NULL,
  order_code TEXT,
  manager TEXT,
  profile_id TEXT,
  profile_name TEXT,
  group_name TEXT,
  PRIMARY KEY (proxy_host, proxy_port, order_code)
);

CREATE TABLE IF NOT EXISTS products_cache (
  id_product INTEGER PRIMARY KEY,
  name_product TEXT,
  price REAL,
  proxy_type TEXT,
  countrycode TEXT,
  store_quantity INTEGER,
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT
);

CREATE TABLE IF NOT EXISTS app_state (
  key TEXT PRIMARY KEY,
  value TEXT
);
"#;
