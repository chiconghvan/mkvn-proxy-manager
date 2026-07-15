export interface ProxyRow {
  order_code: string;
  raw_proxy: string;
  raw_proxy_ip?: string | null;
  proxy_type?: string | null;
  profile_id?: string | null;
  profile_name?: string | null;
  group_name?: string | null;
  manager?: string | null;
  purchase_date?: string | null;
  remaining_days: number;
  renewal?: string | null;
  status: string;
  note?: string | null;
  price?: number | null;
}

export interface Product {
  id_product: number;
  name_product: string;
  price?: number | null;
  proxy_type?: string | null;
  countrycode?: string | null;
  description?: string | null;
  buy_max?: number | null;
  buy_min?: number | null;
  catalogue?: string | null;
  store_quantity?: number | null;
}

export interface Balance {
  username?: string | null;
  level?: string | null;
  balance: number;
  chietkhau?: number | null;
}

export interface AppSettings {
  mkvn_token: string;
  gpm_standard_enabled: boolean;
  gpm_standard_url: string;
  gpm_global_enabled: boolean;
  gpm_global_url: string;
  donut_enabled: boolean;
  donut_url: string;
  sqlite_path?: string | null;
  auto_sync_interval_secs: number;
  theme: 'light' | 'dark' | string;
  column_widths?: Record<string, number>;
  auto_check_update: boolean;
}

export interface AppUpdateInfo {
  current_version: string;
  new_version: string;
  release_notes: string;
  download_url: string;
  published_at: string;
  release_page_url?: string | null;
  update_available: boolean;
}

export interface SyncProgress {
  message: string;
  current: number;
  total: number;
}

export interface ManagerSyncResult {
  manager: string;
  profiles: number;
  groups: number;
  error?: string | null;
}

export interface SyncResult {
  managers: ManagerSyncResult[];
  orders: number;
  proxies: number;
  matched: number;
  errors: string[];
}
