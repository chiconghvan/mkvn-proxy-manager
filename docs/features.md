# Proxy Manager — Tính năng, Luồng xử lý & Workflow

## 1. Tổng quan kiến trúc

```
main.py
 └─ proxymanager/
     ├─ qt_ui/app.py          # UI PySide6 (main)
     ├─ ui/app.py             # UI Tkinter (fallback)
     ├─ ui/widgets.py         # Widget hỗ trợ Tkinter
     ├─ clients/
     │  ├─ proxy_provider.py  # Abstract base class
     │  ├─ ipv4_provider.py   # MkVnProxyProvider (MKVN API)
     │  ├─ gpm_standard_client.py  # GPM Standard API
     │  └─ gpm_global_client.py    # GPM Global API
     ├─ services/
     │  ├─ proxy_service.py   # ProxyManagerService (orchestrator)
     │  ├─ matcher.py         # ProxyProfileMatcher
     │  └─ database_service.py # SQLite cache
     ├─ constants.py
     └─ utils.py              # RateLimiter, proxy parsing, date helpers
```

**Luồng khởi động:**
1. `main.py` ưu tiên gọi `proxymanager.qt_ui.run_app()` (PySide6).
2. Nếu PySide6 không có, fallback sang `proxymanager.ui.ProxyManagerApp` (Tkinter).
3. UI khởi tạo `MkVnProxyProvider`, `GpmStandardClient`/`GpmGlobalClient`, `DatabaseService`, và `ProxyManagerService`.
4. Gọi `load_cached_snapshot()` để hiển thị dữ liệu từ SQLite ngay lập tức.
5. Chạy sync nền (`sync_all` / `_kickoff_initial_sync`) để lấy dữ liệu mới từ API.

---

## 2. Các chức năng chính

### 2.1. Quản lý API Key & Cấu hình GPM

**Mô tả:** Người dùng nhập API Key của MKVN và chọn phiên bản GPM (Standard hoặc Global).

**Logic:**
- API Key được lưu vào file `apikey.cache`.
- Phiên bản GPM lưu vào `settings.json` với key `gpm_version`.
- Khi nhấn OK, gọi `_configure_gpm_client()` để swap client:
  - `GPM` → `GpmStandardClient(base_url=http://127.0.0.1:19995/api/v3)`
  - `GPM Global` → `GpmGlobalClient(base_url=http://127.0.0.1:9495/api/v1)`
- Sau đó chạy `sync_all(refresh_existing=True)`.

### 2.2. Sync & Load dữ liệu (Reload)

**Mô tả:** Tải danh sách proxy từ MKVN API, đồng bộ với GPM profiles, hiển thị lên bảng.

**Workflow chi tiết (`ProxyManagerService.load_proxy_rows`):**

```
1. Gọi gpm_client.list_profiles() → danh sách profiles từ GPM
2. Gọi gpm_client.list_groups() → map group_id → group_name
3. Xây dựng ProxyProfileMatcher từ profiles + groups
4. Lưu cache groups & profiles vào SQLite

5. Lấy danh sách orders từ MKVN API (get_raw_orders)
6. Lọc orders còn hạn (time_con_lai > 0 && renewal != "EXPIRE")
7. So sánh với DB:
   - Xóa orders hết hạn khỏi DB (nếu không filter order_codes)
   - Xác định orders cần fetch proxies (mới hoặc refresh)
8. Fetch proxies song song cho các orders cần (ThreadPoolExecutor, max 8 luồng)
   - Rate limit: 15 requests / 10 giây (RateLimiter)
   - Retry tối đa 5 vòng cho orders fail
9. Lưu orders & proxies vào SQLite
10. Đọc lại proxies từ DB (join với orders)
11. Match từng proxy với GPM profile → xác định profile + group name
12. Trả về rows (hiển thị) + proxies (raw)
```

**GPM Pagination:**
- **Standard:** `response.data` là array, `response.pagination.total_page`.
- **Global:** `response.data.data` là array, `response.data.last_page`.

### 2.3. Matching Proxy ↔ GPM Profile

**Mô tả:** Tự động ghép proxy từ MKVN với profile trong GPM dựa trên IP:Port.

**Class:** `ProxyProfileMatcher` (`services/matcher.py`)

**Logic:**
1. `build(profiles, groups_map)`: Duyệt tất cả profiles, extract `host:port` từ `raw_proxy`, xây dictionary lookup:
   ```
   { "host:port": [{"profile": "...", "group": "..."}, ...] }
   ```
2. `match(proxy_item)`: Extract `host:port` từ proxy string, tra cứu trong lookup.
3. Matching priority theo AGENTS.md: IP → IP:Port.

### 2.4. Mua Proxy (Buy)

**Mô tả:** Mua proxy mới từ MKVN API.

**Workflow:**
```
1. Gọi bootstrap (nếu chưa có products) → lấy danh sách sản phẩm + số dư
2. Hiển thị dialog chọn sản phẩm (lọc theo tồn kho > 0)
3. Người dùng chọn: sản phẩm, số lượng, renewal (ON/OFF), ghi chú
4. Gọi MkVnProxyProvider.buy_proxy():
   - Với mỗi đơn (quantity lần):
     - POST /buy với id_product, quantity=1, renewal, note
     - Retry tối đa 3 lần nếu fail
     - Delay ngẫu nhiên 0.5-1s giữa các đơn
   - Parse response → extract proxies
5. Lưu skeletal order + proxies vào SQLite
6. Reload orders vừa mua để đồng bộ đầy đủ thông tin
```

### 2.5. Gia hạn Proxy (Renew)

**Mô tả:** Gia hạn thời gian sử dụng proxy theo tháng.

**Workflow:**
1. Người dùng chọn proxy trong bảng → chọn số tháng gia hạn.
2. Gọi `MkVnProxyProvider.renew_proxy()` → `POST /renewalplus` với `ordercode` + `month`.
3. Reload các order vừa gia hạn để cập nhật bảng.

### 2.6. Bật/Tắt Renewal Tự Động

**Mô tả:** Bật hoặc tắt chế độ tự động gia hạn cho order.

**Workflow:**
1. Chọn proxy → xác định order_code và trạng thái renewal hiện tại.
2. Gọi `MkVnProxyProvider.toggle_renewal()` → `POST /renewalonoff`.
3. Cập nhật local state và reload order.

### 2.7. Copy Proxy

**Mô tả:** Copy proxy string của các dòng được chọn vào clipboard.

**Logic:** Lấy giá trị cột "proxy" từ các row đang chọn → join bằng newline → copy vào clipboard.

### 2.8. Kiểm tra Proxy theo IP (Chỉ Tkinter)

**Mô tả:** Nhập danh sách IP/Proxy, kiểm tra xem có thuộc tài khoản không.

**Logic:** `check_proxies_by_input(lines)` → match từng dòng theo `ip` hoặc `proxy_compare`.

### 2.9. Lọc & Tìm kiếm (Chỉ PySide6)

- **Search:** Lọc theo tên profile (text search, case-insensitive).
- **Group filter:** Lọc theo group name (dropdown).
- **Sort:** Click vào header column để sort.
- **Row coloring:**
  - Xanh lá: renewal ON
  - Đỏ nhạt: days <= 3
  - Vàng nhạt: unmatched (no profile)
  - Đỏ đậm: sync status failed/error

### 2.10. Cache & Snapshot (DatabaseService)

**SQLite tables:**
| Table | Mục đích |
|---|---|
| `orders` | Thông tin đơn hàng từ MKVN |
| `proxies` | Chi tiết từng proxy trong order |
| `gpm_groups_cache` | Groups từ GPM |
| `gpm_profiles_cache` | Profiles từ GPM (kèm host:port) |
| `products_cache` | Sản phẩm MKVN |
| `app_state` | Key-value store (balance, ...) |
| `order_sync_meta` | Trạng thái đồng bộ orders |
| `proxy_sync_meta` | Trạng thái đồng bộ proxies |

**Cache snapshot flow:**
1. `load_cached_snapshot()` → đọc từ DB, build matcher, trả về rows.
2. Dùng để hiển thị UI ngay lập tức trước khi sync API.
3. Sync API xong → upsert vào DB → refresh UI.

### 2.11. Luồng xử lý bất đồng bộ (PySide6)

- `JobRunnable` (QRunnable) chạy task trong thread pool.
- `WorkerSignals`: progress, finished, error signals.
- UI cập nhật qua signal-slot: progress bar, status text, log box.
- Khi đang busy, disable các action buttons.

---

## 3. Chi tiết API

### 3.1. MKVN API (ipv4_provider.py)

Base URL: `https://proxy.mkvn.net/api/apiv1`

| Method | Path | Params | Mô tả |
|---|---|---|---|
| GET | `/getbalance` | token | Lấy số dư |
| GET | `/products` | token | Danh sách sản phẩm |
| GET | `/getlistorders` | token | Danh sách đơn hàng |
| GET | `/proxies` | token, ordercode | Chi tiết proxy của 1 order |
| POST | `/buy` | token, id_product, quantity, renewal, note | Mua proxy |
| POST | `/renewalplus` | token, ordercode, month | Gia hạn |
| POST | `/renewalonoff` | token, ordercode, renewal | Bật/tắt auto renewal |

**Rate limit:** 15 requests / 10 giây (dùng chung `mkvn_limiter`).

### 3.2. GPM Standard API (gpm_standard_client.py)

Base URL: `http://127.0.0.1:19995/api/v3`

| Method | Path | Params | Mô tả |
|---|---|---|---|
| GET | `/profiles` | page, per_page | Danh sách profiles |
| GET | `/groups` | - | Danh sách groups |

**Response shape:** `{ "data": [...], "pagination": { "total_page": N } }`

### 3.3. GPM Global API (gpm_global_client.py)

Base URL: `http://127.0.0.1:9495/api/v1`

| Method | Path | Params | Mô tả |
|---|---|---|---|
| GET | `/profiles` | page, per_page, page_size | Danh sách profiles |
| GET | `/groups` | page, per_page, page_size | Danh sách groups |

**Response shape:** `{ "data": { "data": [...], "last_page": N, "per_page": M } }`

---

## 4. Proxy Identity Parsing

**File:** `utils.py`

Hai hàm parsing:
- `extract_ip_port_from_proxy()`: Regex tìm `ip:port` (dùng cho MKVN).
- `extract_host_port_from_proxy()`: Parsing linh hoạt hơn:
  1. URL scheme (`http://user:pass@host:port`)
  2. `user:pass@host:port`
  3. `host:port:user:pass`
  4. `host:port`

---

## 5. UI States & Visual Indicators (PySide6)

| Trạng thái | Màu background | Ý nghĩa |
|---|---|---|
| Renewal ON | `#dcfce7` (xanh lá) | Tự động gia hạn |
| Days <= 3 | `#fecaca` (đỏ) | Sắp hết hạn |
| Unmatched | `#fef3c7` (vàng) | Chưa có profile GPM |
| Sync failed | `#fee2e2` (hồng) | Lỗi đồng bộ |
| Default | Trắng/xám xen kẽ | Bình thường |

---

## 6. File cấu hình

| File | Format | Mục đích |
|---|---|---|
| `apikey.cache` | Text | API key MKVN |
| `settings.json` | JSON | `gpm_version` (GPM / GPM Global) |
| `proxies.db` | SQLite | Cache dữ liệu |
| `provider_spec.json` | JSON | MKVN API spec (reference) |
