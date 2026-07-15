use std::{collections::HashSet, path::Path, sync::Arc, time::Duration};

use parking_lot::Mutex;
use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    database::schema::SCHEMA,
    error::AppResult,
    matcher::MatchResult,
    models::{MkvnOrder, Product, ProxyDetail, ProxyRow, UnifiedGroup, UnifiedProfile},
};

#[derive(Debug, Clone)]
pub struct StoredProxy {
    pub order_code: String,
    pub raw_proxy: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> AppResult<Self> {
        let conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    pub fn upsert_orders(&self, orders: &[MkvnOrder]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO orders (code, username, name_product, quantity, price, time_buy, time_dau_ky, time_cuoi_ky, time_con_lai, renewal, note, proxy_type, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, datetime('now'))
                 ON CONFLICT(code) DO UPDATE SET
                   username=excluded.username,
                   name_product=excluded.name_product,
                   quantity=excluded.quantity,
                   price=excluded.price,
                   time_buy=excluded.time_buy,
                   time_dau_ky=excluded.time_dau_ky,
                   time_cuoi_ky=excluded.time_cuoi_ky,
                   time_con_lai=excluded.time_con_lai,
                   renewal=excluded.renewal,
                   note=excluded.note,
                   proxy_type=excluded.proxy_type,
                   updated_at=datetime('now')",
            )?;
            for order in orders {
                stmt.execute(params![
                    order.code,
                    order.username,
                    order.name_product,
                    order.quantity,
                    order.price,
                    order.time_buy,
                    order.time_dau_ky,
                    order.time_cuoi_ky,
                    order.time_con_lai,
                    order.renewal,
                    order.note,
                    order.proxy_type,
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn upsert_products(&self, products: &[Product]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO products_cache (id_product, name_product, price, proxy_type, countrycode, store_quantity, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'))
                 ON CONFLICT(id_product) DO UPDATE SET
                   name_product=excluded.name_product,
                   price=excluded.price,
                   proxy_type=excluded.proxy_type,
                   countrycode=excluded.countrycode,
                   store_quantity=excluded.store_quantity,
                   updated_at=datetime('now')",
            )?;
            for p in products {
                stmt.execute(params![p.id_product, p.name_product, p.price, p.proxy_type, p.countrycode, p.store_quantity])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_products(&self) -> AppResult<Vec<Product>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT id_product, name_product, price, proxy_type, countrycode, store_quantity FROM products_cache ORDER BY name_product")?;
        let rows = stmt.query_map([], |row| {
            Ok(Product {
                id_product: row.get(0)?,
                name_product: row.get(1)?,
                price: row.get(2)?,
                proxy_type: row.get(3)?,
                countrycode: row.get(4)?,
                store_quantity: row.get(5)?,
                ..Default::default()
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn upsert_proxies(&self, order_code: &str, proxies: &[ProxyDetail]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM proxies WHERE order_code = ?1", params![order_code])?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO proxies (order_code, raw_proxy, raw_proxy_ip, host, port)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(order_code, raw_proxy) DO UPDATE SET
                   raw_proxy_ip=excluded.raw_proxy_ip,
                   host=excluded.host,
                   port=excluded.port",
            )?;
            for proxy in proxies {
                stmt.execute(params![proxy.order_code, proxy.raw_proxy, proxy.raw_proxy_ip, proxy.host, proxy.port.map(|p| p as i64)])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn delete_expired_orders(&self, active_codes: &HashSet<String>) -> AppResult<()> {
        let existing = self.get_order_codes()?;
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        for code in existing.iter().filter(|code| !active_codes.contains(*code)) {
            tx.execute("DELETE FROM orders WHERE code = ?1", params![code])?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_order_codes(&self) -> AppResult<HashSet<String>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT code FROM orders")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<HashSet<_>, _>>().map_err(Into::into)
    }

    pub fn upsert_profiles(&self, profiles: &[UnifiedProfile]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO profiles_cache (id, manager, name, raw_proxy, host, port, group_id, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))
                 ON CONFLICT(manager, id) DO UPDATE SET
                   name=excluded.name,
                   raw_proxy=excluded.raw_proxy,
                   host=excluded.host,
                   port=excluded.port,
                   group_id=excluded.group_id,
                   updated_at=datetime('now')",
            )?;
            for p in profiles {
                stmt.execute(params![p.id, p.manager, p.name, p.raw_proxy, p.host, p.port.map(|v| v as i64), p.group_id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn upsert_groups(&self, groups: &[UnifiedGroup]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO groups_cache (id, manager, name)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(manager, id) DO UPDATE SET name=excluded.name",
            )?;
            for g in groups {
                stmt.execute(params![g.id, g.manager, g.name])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_profiles(&self) -> AppResult<Vec<UnifiedProfile>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT id, manager, name, raw_proxy, host, port, group_id FROM profiles_cache")?;
        let rows = stmt.query_map([], |row| {
            let port: Option<i64> = row.get(5)?;
            Ok(UnifiedProfile {
                id: row.get(0)?,
                manager: row.get(1)?,
                name: row.get(2)?,
                raw_proxy: row.get(3)?,
                host: row.get(4)?,
                port: port.map(|p| p as u16),
                group_id: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_groups(&self) -> AppResult<Vec<UnifiedGroup>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT id, manager, name FROM groups_cache")?;
        let rows = stmt.query_map([], |row| {
            Ok(UnifiedGroup { id: row.get(0)?, manager: row.get(1)?, name: row.get(2)? })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_order_codes_with_proxies(&self) -> AppResult<HashSet<String>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT DISTINCT order_code FROM proxies")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<HashSet<_>, _>>().map_err(Into::into)
    }

    pub fn get_stored_proxies(&self) -> AppResult<Vec<StoredProxy>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare("SELECT order_code, raw_proxy, host, port FROM proxies")?;
        let rows = stmt.query_map([], |row| {
            let port: Option<i64> = row.get(3)?;
            Ok(StoredProxy {
                order_code: row.get(0)?,
                raw_proxy: row.get(1)?,
                host: row.get(2)?,
                port: port.map(|p| p as u16),
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn save_match_results(&self, results: &[MatchResult]) -> AppResult<()> {
        let mut conn = self.conn.lock();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM match_results", [])?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO match_results (proxy_host, proxy_port, order_code, manager, profile_id, profile_name, group_name)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )?;
            for r in results {
                stmt.execute(params![r.proxy_host, r.proxy_port as i64, r.order_code, r.manager, r.profile_id, r.profile_name, r.group_name])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_proxy_rows(&self) -> AppResult<Vec<ProxyRow>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT
                o.code, p.raw_proxy, p.raw_proxy_ip, o.proxy_type,
                m.profile_id, m.profile_name, m.group_name, m.manager,
                o.time_buy, o.time_con_lai, o.renewal, o.note, o.price
             FROM proxies p
             JOIN orders o ON o.code = p.order_code
             LEFT JOIN match_results m ON m.order_code = p.order_code AND m.proxy_host = p.host AND m.proxy_port = p.port
             ORDER BY o.code, p.raw_proxy",
        )?;
        let rows = stmt.query_map([], |row| {
            let time_con_lai: Option<String> = row.get(9)?;
            let profile_name: Option<String> = row.get(5)?;
            let status = if profile_name.is_some() { "Matched" } else { "Unmatched" }.to_string();
            Ok(ProxyRow {
                order_code: row.get(0)?,
                raw_proxy: row.get(1)?,
                raw_proxy_ip: row.get(2)?,
                proxy_type: row.get(3)?,
                profile_id: row.get(4)?,
                profile_name,
                group_name: row.get(6)?,
                manager: row.get(7)?,
                purchase_date: row.get(8)?,
                remaining_days: parse_remaining_days(time_con_lai.as_deref()),
                renewal: row.get(10)?,
                status,
                note: row.get(11)?,
                price: row.get(12)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn get_setting(&self, key: &str) -> AppResult<Option<String>> {
        let conn = self.conn.lock();
        conn.query_row("SELECT value FROM settings WHERE key = ?1", params![key], |row| row.get(0)).optional().map_err(Into::into)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> AppResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn set_app_state(&self, key: &str, value: &str) -> AppResult<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}

pub fn parse_remaining_days(value: Option<&str>) -> i64 {
    let Some(value) = value else { return 0; };
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if !digits.is_empty() {
        return digits.parse().unwrap_or(0);
    }
    let mut current = String::new();
    for ch in value.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else if !current.is_empty() {
            return current.parse().unwrap_or(0);
        }
    }
    current.parse().unwrap_or(0)
}
