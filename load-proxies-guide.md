# Load & Cache MKVN Proxies

Tài liệu mô tả flow project load toàn bộ proxy từ MKVN API và lưu cache xuống SQLite.

---

## Tổng quan luồng xử lý

```
UI (Reload / Initial Sync)
  → ProxyManagerService.load_proxy_rows()
    → [1] GPM: load profiles + groups (để map proxy → profile)
    → [2] MKVN API: getlistorders → lọc đơn còn hạn
    → [3] DB sync: so sánh orders API vs DB, xóa đơn hết hạn
    → [4] DB sync: upsert orders metadata
    → [5] Parallel fetch: tải proxy cho từng order qua API
    → [6] DB save: lưu proxy vào bảng proxies
    → [7] Đọc lại từ DB → trả về rows cho UI
```

---

## Bước chi tiết

### 1. Khởi tạo & Bootstrap

**File:** `proxymanager/qt_ui/app.py` (PySide6) hoặc `proxymanager/ui/app.py` (Tkinter)

- `ProxyManagerWindow.__init__` tạo các đối tượng:
  - `MkVnProxyProvider` — client gọi MKVN API
  - `GpmStandardClient` hoặc `GpmGlobalClient` — client GPM (lấy profiles/groups)
  - `DatabaseService(DB_FILE_PATH)` — SQLite, file `proxies.db`
  - `ProxyManagerService` — orchestrator kết nối provider + GPM + DB

- **Startup sequence:**
  1. `_load_cached_ui()` → đọc cache từ DB, hiển thị ngay (không gọi API)
  2. `_kickoff_initial_sync()` → `QTimer.singleShot(0, ...)` chạy sync nền sau khi UI render

```python
# qt_ui/app.py:669-677
def _load_cached_ui(self):
    snapshot = self.service.load_cached_snapshot()
    self._apply_snapshot(snapshot, source_label="cache")

def _kickoff_initial_sync(self):
    if not self.api_key:
        self._log("API key trống, chỉ hiển thị cache.")
        return
    self.sync_all(refresh_existing=False, source_label="sync nền")
```

### 2. Bootstrap (Tải sản phẩm + Số dư)

**File:** `proxymanager/services/proxy_service.py:41-56`

```python
def bootstrap(self, api_key, progress_callback=None):
    products = self.provider_client.get_products(api_key)   # GET /products
    balance = self.provider_client.get_balance(api_key)      # GET /getbalance
    if self.db_service:
        self.db_service.cache_products(products)
        self.db_service.set_app_state("balance", balance)
```

Gọi 2 API endpoint của MKVN, lưu sản phẩm và số dư vào DB.

### 3. Load Proxy Rows (Luồng chính)

**File:** `proxymanager/services/proxy_service.py:83-196`

```
load_proxy_rows(api_key, progress_callback, order_codes, refresh_existing)
```

#### 3a. Load GPM Profiles & Groups (để matching)

```python
profiles = self.gpm_client.list_profiles()
groups_map = self.gpm_client.list_groups()
self.matcher.build(profiles, groups_map)
```

- Profiles/Groups được cache vào DB (`gpm_profiles_cache`, `gpm_groups_cache`)
- Dùng để map proxy → profile name + group name hiển thị trên UI

#### 3b. Lấy danh sách đơn hàng từ MKVN API

```python
all_api_orders = self.provider_client.get_raw_orders(api_key)
# GET /getlistorders → response.data = [order, ...]
```

**Endpoint:** `GET https://proxy.mkvn.net/api/apiv1/getlistorders?token=<key>`

Response shape:
```json
{
  "status": "SUCCESS",
  "statusCode": 200,
  "data": [
    {
      "code": "ORD001",
      "name_product": "Proxy 1...",
      "time_con_lai": "15.5",
      "renewal": "ON",
      "type": "IPv4",
      ...
    }
  ]
}
```

#### 3c. Lọc đơn còn hạn

```python
active_api_orders = [
    o for o in all_api_orders
    if self.provider_client._parse_remaining_days(o.get("time_con_lai")) > 0
    and o.get("renewal") != "EXPIRE"
]
```

**Rules:**
- Bỏ đơn có `time_con_lai <= 0` (hết hạn)
- Bỏ đơn có `renewal == "EXPIRE"`

#### 3d. DB Sync — Dọn dẹp đơn hết hạn

```python
db_orders = self.db_service.get_all_orders()
db_codes = {o['code'] for o in db_orders}
api_codes = {o['code'] for o in active_api_orders}

to_delete = db_codes - api_codes  # Đơn có trong DB nhưng không còn ở API
for code in to_delete:
    self.db_service.delete_order(code)  # CASCADE xóa luôn proxies
```

#### 3e. Xác định đơn cần tải proxy mới

```python
to_fetch_proxies = []
for o in active_api_orders:
    if refresh_existing or o['code'] not in db_codes or order_codes:
        to_fetch_proxies.append(o)
```

- `refresh_existing=True` → tải lại toàn bộ (Reload button)
- `order_codes` được chỉ định → chỉ tải các đơn đó
- Đơn đã có trong DB và không force refresh → bỏ qua, dùng cache

#### 3f. Upsert orders metadata

```python
self.db_service.upsert_orders(active_api_orders)
self.db_service.upsert_order_sync_meta(active_api_orders, status="synced")
```

Lưu/thêm mới thông tin đơn vào bảng `orders`.

### 4. Parallel Fetch — Tải proxy từng order

**File:** `proxymanager/services/proxy_service.py:155-166`

```python
workers = self.provider_client._auto_workers(len(to_fetch_proxies))
loaded_proxies, _ = self.provider_client._load_orders_parallel(
    api_key, to_fetch_proxies, workers, progress_callback=progress_callback
)
```

**File:** `proxymanager/clients/ipv4_provider.py:142-167`

- Dùng `ThreadPoolExecutor` chạy parallel, mỗi thread gọi 1 order
- Mỗi order gọi: `GET /proxies?token=<key>&ordercode=<code>`
- Workers tối đa 8, tự động theo số lượng orders
- Mỗi request phải qua `mkvn_limiter.wait()` — rate limit 15 request / 10 giây

#### 4a. Extract proxy từ response

**File:** `proxymanager/clients/ipv4_provider.py:87-125`

```python
def _extract_proxies_from_response(self, order_data, proxy_details):
    proxies_host = proxy_details.get("proxies", [])   # ["http://user:pass@ip:port", ...]
    proxies_ip   = proxy_details.get("proxiesip", [])  # ["ip:port:user:pass", ...]
```

- Proxy format: `http://user:pass@ip:port` hoặc `ip:port:user:pass`
- Mỗi proxy có `idproxy = "{order_code}::{index}"` (VD: `ORD001::1`)
- Extract IP bằng regex `(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d+)`

#### 4b. Retry failed orders

**File:** `proxymanager/clients/ipv4_provider.py:211-229`

```python
for round_idx in range(1, 6):  # Tối đa 5 vòng retry
    if not remaining:
        break
    round_loaded, round_remaining = self._load_orders_parallel(...)
    proxies.extend(round_loaded)
    remaining = round_remaining
```

- Sau lần tải đầu, các order fail được retry tối đa 5 lần
- Danh sách order fail cuối cùng lưu vào `_last_failed_order_codes`

### 5. Lưu cache vào SQLite

**File:** `proxymanager/services/database_service.py:315-342`

```python
def save_proxies_for_orders(self, proxies_by_order):
    # proxies_by_order = { "ORD001": [proxy_dict, ...], ... }
    for order_code, proxies in proxies_by_order.items():
        cursor.execute("DELETE FROM proxies WHERE order_code = ?", (order_code,))
        # INSERT OR UPDATE từng proxy
```

**Strategy:** Xóa sạch proxies của order đó rồi insert lại (không incremental).

### 6. Đọc lại từ DB

**File:** `proxymanager/services/proxy_service.py:168-172`

```python
proxies = self.db_service.get_all_active_proxies()
```

```sql
SELECT p.*, o.type, o.time_con_lai, o.time_cuoi_ky, o.name_product, o.renewal, o.time_buy
FROM proxies p
JOIN orders o ON p.order_code = o.code
ORDER BY o.code, p.idproxy
```

### 7. Trả kết quả cho UI

**File:** `proxymanager/services/proxy_service.py:195-196`

```python
rows, all_proxies = self._build_rows_from_proxies(proxies)
```

Mỗi row bao gồm: `stt`, `order_code`, `proxy`, `ip`, `type`, `profile`, `group`, `days`, `renewal`, `time_buy`, `sync_status`.

---

## SQLite Schema liên quan

### Bảng `orders`

| Column | Type | Ghi chú |
|---|---|---|
| `code` | TEXT PK | Mã đơn hàng |
| `name_product` | TEXT | Tên sản phẩm |
| `time_con_lai` | REAL | Số ngày còn lại |
| `renewal` | TEXT | ON/OFF/EXPIRE |
| `type` | TEXT | IPv4, IPv6... |

### Bảng `proxies`

| Column | Type | Ghi chú |
|---|---|---|
| `idproxy` | TEXT PK | `{order_code}::{index}` |
| `order_code` | TEXT FK | → orders.code |
| `ip` | TEXT | IP address |
| `proxy` | TEXT | `http://user:pass@ip:port` |
| `proxy_compare` | TEXT | `ip:port:user:pass` (để so sánh) |

### Indexes

- `idx_proxies_order_code` trên `proxies(order_code)`
- `idx_proxies_ip` trên `proxies(ip)`
- `idx_orders_renewal` trên `orders(renewal)`
- `idx_orders_time_con_lai` trên `orders(time_con_lai)`

### PRAGMA Tuning

```python
PRAGMA foreign_keys=ON
PRAGMA journal_mode=WAL
PRAGMA synchronous=NORMAL
PRAGMA temp_store=MEMORY
PRAGMA busy_timeout=5000
```

---

## Rate Limiting

**File:** `proxymanager/utils.py:86-109`

```python
class RateLimiter:
    def __init__(self, count, window):
        # count=15, window=10.0 seconds
        self.timestamps = deque()  # sliding window

mkvn_limiter = RateLimiter(15, 10.0)  # Singleton
```

- Mọi request MKVN đều gọi `mkvn_limiter.wait()` trước khi gửi
- Sliding window: nếu 10 giây qua đã có 15 request → sleep chờ slot trống
- Shared across threads: dùng `threading.Lock` để an toàn

---

## Progress Callback Shape

Mọi hàm trong flow đều nhận `progress_callback(message, ratio)`:

- `ratio = None` → chỉ hiển thị message, không có progress bar
- `ratio = (current, total)` → cập nhật progress bar (`current/total * 100%`)
- VD: `("Đã tải 5/10 đơn: ORD001", (5, 10))`

---

## Cache Flow Summary

```
[Lần đầu mở app]
  1. Đọc proxies.db → hiển thị cache (nếu có)
  2. Gọi API: getlistorders → lọc đơn còn hạn
  3. So sánh DB vs API → xóa đơn hết hạn
  4. Tải proxy cho đơn mới/thiếu (parallel + rate limit)
  5. Save vào DB → hiển thị kết quả

[Reload button]
  - Nếu đã chọn order → chỉ reload order đó
  - Nếu không chọn → refresh_existing=True → tải lại toàn bộ

[Shutdown]
  - Không cần lưu gì thêm, mọi thứ đã ở trong proxies.db
```
