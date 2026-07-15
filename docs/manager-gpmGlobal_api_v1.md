# GPMLogin Local API v1

> Tài liệu rút gọn, sắp xếp lại từ bản crawl `gpmlogin_api_v1_crawled.md`.

## Thông tin chung

| Mục | Giá trị |
|---|---|
| Base URL | `http://localhost:9495/api/v1` |
| Localhost khác | `http://127.0.0.1:9495/api/v1` |
| Port mặc định | `9495` |
| Auth | Không yêu cầu API key/token |
| Response | JSON |

GPMLogin chạy một HTTP server cục bộ trên máy để điều khiển phần mềm bằng code: tạo/chạy/dừng profile, quản lý group, proxy và extension.

Nếu port `9495` bị chiếm, GPMLogin có thể tự chọn port ngẫu nhiên trong khoảng `8000–10000` và ghi port đang dùng vào file `http.port` trong thư mục dữ liệu cục bộ của ứng dụng.

## Response format chung

```json
{
  "success": true,
  "data": {},
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## Phân trang chung

Các endpoint dạng danh sách thường nhận các query sau:

| Query | Kiểu | Mặc định | Mô tả |
|---|---:|---:|---|
| `page` | `int` | `1` | Trang hiện tại |
| `page_size` | `int` | `30` | Số item mỗi trang |
| `search` | `string` | — | Từ khóa tìm kiếm |
| `sort` | `int` | `0` | Kiểu sắp xếp |

Response phân trang:

```json
{
  "current_page": 1,
  "per_page": 30,
  "total": 120,
  "last_page": 4,
  "data": []
}
```

Bảng `sort`:

| Giá trị | Ý nghĩa |
|---:|---|
| `0` | Mới tạo → cũ nhất |
| `1` | Cũ nhất → mới tạo |
| `2` | Tên A → Z |
| `3` | Tên Z → A |

## Enum thường dùng

Các trường enum có thể nhận **số nguyên** hoặc **chuỗi tên**, không phân biệt hoa thường.

| Trường | Giá trị |
|---|---|
| `browser_type` | `1 = chrome`, `2 = firefox` |
| `os_type` | `1 = Windows`, `2 = macOS Intel`, `3 = macOS ARM`, `4 = Linux`, `5 = Android` |
| `webrtc_mode` | `1 = Theo IP`, `2 = Cố định`, `3 = Thật`, `4 = Tắt` |
| `geolocation_mode` | `1 = Cho phép`, `2 = Hỏi`, `3 = Chặn` |
| `canvas_mode` | `1 = Nhiễu`, `2 = Thật`, `3 = Chặn` |
| `client_rect_mode` | `1 = Nhiễu`, `2 = Thật` |
| `webgl_image_mode` | `1 = Nhiễu`, `2 = Thật` |
| `webgl_metadata_mode` | `1 = Giả lập`, `2 = Thật` |
| `audio_mode` | `1 = Nhiễu`, `2 = Thật` |
| `font_mode` | `1 = Giả lập`, `2 = Thật` |

## Mục lục endpoint

### Profiles

| Method | Endpoint | Mô tả |
|---|---|---|
| `GET` | `/profiles` | Lấy danh sách profile |
| `GET` | `/profiles/{id}` | Lấy chi tiết 1 profile |
| `POST` | `/profiles/create` | Tạo profile |
| `POST` | `/profiles/update/{id}` | Cập nhật profile |
| `GET` | `/profiles/delete/{id}` | Xóa profile |
| `GET` | `/profiles/start/{id}` | Mở trình duyệt |
| `GET` | `/profiles/stop/{id}` | Đóng trình duyệt |
| `GET` | `/browsers/versions` | Lấy danh sách phiên bản trình duyệt |

### Groups

| Method | Endpoint | Mô tả |
|---|---|---|
| `GET` | `/groups` | Lấy danh sách group |
| `GET` | `/groups/{id}` | Lấy chi tiết 1 group |
| `POST` | `/groups/create` | Tạo group |
| `POST` | `/groups/update/{id}` | Cập nhật group |
| `GET` | `/groups/delete/{id}` | Xóa group |

### Proxies

| Method | Endpoint | Mô tả |
|---|---|---|
| `GET` | `/proxies` | Lấy danh sách proxy |
| `GET` | `/proxies/{id}` | Lấy chi tiết 1 proxy |
| `POST` | `/proxies/create` | Tạo proxy |
| `POST` | `/proxies/update/{id}` | Cập nhật proxy |
| `GET` | `/proxies/delete/{id}` | Xóa proxy |

### Extensions

| Method | Endpoint | Mô tả |
|---|---|---|
| `GET` | `/extensions` | Lấy danh sách extension |
| `GET` | `/extensions/update-state/{id}` | Bật/tắt extension |

---

# 1. Profiles

## 1.1. Lấy danh sách profile

```http
GET /api/v1/profiles
```

Trả về danh sách profile có phân trang. Danh sách này **không kèm fingerprint** để response gọn hơn. Muốn xem fingerprint đầy đủ, dùng endpoint lấy 1 profile.

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `page` | `int` | Không | `1` | Trang hiện tại |
| `page_size` | `int` | Không | `30` | Số item mỗi trang |
| `search` | `string` | Không | — | Tìm theo tên profile |
| `sort` | `int` | Không | `0` | Kiểu sắp xếp `0–3` |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "current_page": 1,
    "per_page": 30,
    "total": 2,
    "last_page": 1,
    "data": [
      {
        "id": "37f783ac-2635-4d53-ab8d-a300c790ecdc",
        "name": "Profile 01",
        "group_id": "all",
        "storage_path": "C:\\GPMLogin\\profiles\\37f7...",
        "raw_proxy": "socks5://127.0.0.1:5000",
        "browser": {
          "name": "chrome",
          "version": "137.0.7151.41"
        },
        "os": "windows",
        "note": "",
        "created_at": "2026-06-08 10:00:00",
        "updated_at": "2026-06-08 10:00:00",
        "tags": []
      }
    ]
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.2. Lấy chi tiết 1 profile

```http
GET /api/v1/profiles/{id}
```

Trả về chi tiết profile, bao gồm dữ liệu `fingerprint`.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID profile |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "37f783ac-2635-4d53-ab8d-a300c790ecdc",
    "name": "Profile 01",
    "group_id": "all",
    "storage_path": "C:\\GPMLogin\\profiles\\37f7...",
    "raw_proxy": "",
    "browser": {
      "name": "chrome",
      "version": "137.0.7151.41"
    },
    "os": "windows",
    "note": "",
    "created_at": "2026-06-08 10:00:00",
    "updated_at": "2026-06-08 10:00:00",
    "tags": [],
    "fingerprint": {
      "...": "dữ liệu fingerprint đầy đủ"
    }
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.3. Tạo profile

```http
POST /api/v1/profiles/create
```

Tạo profile mới. Chỉ `name` là bắt buộc. Các trường khác nếu bỏ qua hoặc `null` sẽ dùng cấu hình mặc định, fingerprint được random tự động.

### Body params

| Field | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `name` | `string` | Có | — | Tên profile |
| `group_id` | `string` | Không | — | ID group chứa profile |
| `raw_proxy` | `string` | Không | — | Proxy, ví dụ `socks5://ip:port`; rỗng = không dùng proxy |
| `bypass_proxy_extensions` | `string` | Không | — | Danh sách domain bỏ qua proxy |
| `browser_type` | `enum` | Không | `chrome` / `1` | `1 = chrome`, `2 = firefox` |
| `browser_version` | `string` | Không | Bản mới nhất | Phiên bản trình duyệt |
| `os_type` | `enum` | Không | OS hiện tại | `1 = Windows`, `2 = macOS Intel`, `3 = macOS ARM`, `4 = Linux`, `5 = Android` |
| `custom_user_agent` | `string` | Không | — | User-Agent tùy chỉnh |
| `task_bar_title` | `string` | Không | — | Tiêu đề cửa sổ trên taskbar |
| `webrtc_mode` | `enum` | Không | — | `1 = Theo IP`, `2 = Cố định`, `3 = Thật`, `4 = Tắt` |
| `fixed_webrtc_public_ip` | `string` | Không | — | IP công khai cố định, cần khi `webrtc_mode = 2` |
| `port_protect` | `string` | Không | — | Danh sách port được bảo vệ |
| `geolocation_mode` | `enum` | Không | — | `1 = Cho phép`, `2 = Hỏi`, `3 = Chặn` |
| `canvas_mode` | `enum` | Không | — | `1 = Nhiễu`, `2 = Thật`, `3 = Chặn` |
| `client_rect_mode` | `enum` | Không | — | `1 = Nhiễu`, `2 = Thật` |
| `webgl_image_mode` | `enum` | Không | — | `1 = Nhiễu`, `2 = Thật` |
| `webgl_metadata_mode` | `enum` | Không | — | `1 = Giả lập`, `2 = Thật` |
| `audio_mode` | `enum` | Không | — | `1 = Nhiễu`, `2 = Thật` |
| `is_masked_media` | `bool` | Không | `false` | `true` = giả lập danh sách camera/mic/loa |
| `font_mode` | `enum` | Không | — | `1 = Giả lập`, `2 = Thật` |
| `timezone_base_on_ip` | `bool` | Không | `true` | `true` = múi giờ tự theo IP proxy |
| `timezone` | `string` | Không | — | Múi giờ cố định, ví dụ `Asia/Ho_Chi_Minh` |
| `is_language_base_on_ip` | `bool` | Không | `true` | `true` = ngôn ngữ tự theo IP proxy |
| `fixed_language` | `string` | Không | — | Ngôn ngữ cố định, ví dụ `vi`, `en`, `zh` |
| `startup_urls` | `string` | Không | — | URL tự mở khi khởi động; nhiều URL ngăn cách bằng xuống dòng |
| `note` | `string` | Không | — | Ghi chú profile |

### Request mẫu

```json
{
  "name": "Test profile from api",
  "group_id": null,
  "raw_proxy": "socks5://127.0.0.1:5000",
  "browser_type": 1,
  "browser_version": "137.0.7151.41",
  "os_type": 1,
  "canvas_mode": 1,
  "timezone_base_on_ip": true,
  "is_language_base_on_ip": true,
  "note": null
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "69911f98-...",
    "name": "Test profile from api",
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.4. Cập nhật profile

```http
POST /api/v1/profiles/update/{id}
```

Cập nhật một phần profile. Trường bỏ qua hoặc `null` sẽ **giữ nguyên giá trị cũ**, không reset.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID profile cần cập nhật |

### Body

Dùng cùng bộ field như khi tạo profile, nhưng tất cả đều không bắt buộc.

### Request mẫu

```json
{
  "name": "Tên mới",
  "raw_proxy": "http://user:pass@127.0.0.1:8080",
  "note": "Đã đổi proxy"
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "69911f98-...",
    "name": "Tên mới",
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.5. Xóa profile

```http
GET /api/v1/profiles/delete/{id}
```

Xóa profile. Có thể xóa mềm hoặc xóa vĩnh viễn.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID profile |

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `mode` | `string` | Không | `soft` | `soft` = đưa vào thùng rác; `hard` = xóa vĩnh viễn cả dữ liệu trên đĩa |

### Response mẫu

```json
{
  "success": true,
  "data": null,
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.6. Mở trình duyệt profile

```http
GET /api/v1/profiles/start/{id}
```

Khởi chạy trình duyệt của profile và trả về thông tin để điều khiển qua Selenium, Puppeteer hoặc Playwright.

- Dùng `websocket_debugging_url` để kết nối Puppeteer/Playwright.
- Dùng `remote_debugging_port` cho Selenium qua `debuggerAddress`.
- Nếu profile đã mở, API trả lại kết quả từ cache, không mở lại.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID profile cần mở |

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `remote_debugging_port` | `int` | Không | Cổng trống ngẫu nhiên | Chỉ định port remote debugging |
| `window_scale` | `double` | Không | `1` | Tỉ lệ cửa sổ, ví dụ `0.8` |
| `window_pos` | `string` | Không | — | Vị trí cửa sổ dạng `x,y`, ví dụ `100,100` |
| `window_size` | `string` | Không | — | Kích thước cửa sổ dạng `width,height`, ví dụ `800,600` |
| `skip_proxy_check` | `bool` | Không | `false` | `true` = bỏ qua kiểm tra proxy trước khi mở |
| `addition_args` | `string` | Không | — | Tham số Chromium thêm, ví dụ `--mute-audio` |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "profile_id": "7798d4ca-a002-4a52-9223-5140c68667bc",
    "driver_path": "C:\\GPMLogin\\drivers\\chromedriver.exe",
    "remote_debugging_port": 40444,
    "websocket_debugging_url": "ws://127.0.0.1:40444/devtools/browser/abc-123",
    "addition_info": {
      "process_id": 12345,
      "profile_name": "Profile 01",
      "window_handle": 1180736,
      "exec_time": 1200
    }
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.7. Đóng trình duyệt profile

```http
GET /api/v1/profiles/stop/{id}
```

Đóng trình duyệt đang chạy của profile.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID profile cần đóng |

### Response mẫu

```json
{
  "success": true,
  "data": null,
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 1.8. Danh sách phiên bản trình duyệt

```http
GET /api/v1/browsers/versions
```

Trả về danh sách phiên bản Chromium và Firefox đang hỗ trợ. Dùng để điền `browser_version` khi tạo profile.

### Response mẫu

```json
{
  "success": true,
  "data": {
    "chromium": [
      "137.0.7151.41",
      "136.0.7103.93",
      "..."
    ],
    "firefox": [
      "128.0",
      "127.0",
      "..."
    ]
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

---

# 2. Groups

## 2.1. Lấy danh sách group

```http
GET /api/v1/groups
```

Trả về danh sách group có phân trang.

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `page` | `int` | Không | `1` | Trang hiện tại |
| `page_size` | `int` | Không | `30` | Số item mỗi trang |
| `search` | `string` | Không | — | Tìm theo tên group |
| `sort` | `int` | Không | `0` | Kiểu sắp xếp `0–3` |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "current_page": 1,
    "per_page": 30,
    "total": 1,
    "last_page": 1,
    "data": [
      {
        "id": "67db46f9-df8c-41c5-a18f-631a090a17a1",
        "name": "Marketing",
        "sort_order": 1,
        "created_at": "2026-06-08T10:00:00Z",
        "updated_at": "2026-06-08T10:00:00Z",
        "creator": null
      }
    ]
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 2.2. Lấy chi tiết 1 group

```http
GET /api/v1/groups/{id}
```

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID group |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "67db46f9-df8c-41c5-a18f-631a090a17a1",
    "name": "Marketing",
    "sort_order": 1,
    "created_at": "2026-06-08T10:00:00Z",
    "updated_at": "2026-06-08T10:00:00Z",
    "creator": null
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 2.3. Tạo group

```http
POST /api/v1/groups/create
```

Tạo group mới.

> Ghi chú: tài liệu Postman cũ dùng khóa `order`; bản hiện tại dùng `sort_order`.

### Body params

| Field | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `name` | `string` | Có | — | Tên group |
| `sort_order` | `int` | Không | `0` | Thứ tự hiển thị, số càng nhỏ càng lên trên |

### Request mẫu

```json
{
  "name": "Group created from api",
  "sort_order": 999
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "...",
    "name": "Group created from api",
    "sort_order": 999,
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 2.4. Cập nhật group

```http
POST /api/v1/groups/update/{id}
```

Đổi tên hoặc thứ tự group. `sort_order` chỉ được cập nhật khi lớn hơn `0`.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID group |

### Body params

| Field | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `name` | `string` | Có | — | Tên group mới |
| `sort_order` | `int` | Không | — | Thứ tự mới; chỉ áp dụng khi `> 0` |

### Request mẫu

```json
{
  "name": "Group edited by api",
  "sort_order": 5
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 2.5. Xóa group

```http
GET /api/v1/groups/delete/{id}
```

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID group |

### Response mẫu

```json
{
  "success": true,
  "data": null,
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

---

# 3. Proxies

## 3.1. Lấy danh sách proxy

```http
GET /api/v1/proxies
```

Trả về danh sách proxy có phân trang.

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `page` | `int` | Không | `1` | Trang hiện tại |
| `page_size` | `int` | Không | `30` | Số item mỗi trang |
| `search` | `string` | Không | — | Tìm theo chuỗi proxy |
| `sort` | `int` | Không | `0` | Kiểu sắp xếp `0–3` |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "current_page": 1,
    "per_page": 30,
    "total": 1,
    "last_page": 1,
    "data": [
      {
        "id": "a36a1e31-bdae-4566-95df-f246996a141c",
        "raw_proxy": "socks5://127.0.0.1:5001",
        "meta_data": null,
        "created_at": "2026-06-08T10:00:00Z",
        "updated_at": "2026-06-08T10:00:00Z",
        "tags": []
      }
    ]
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 3.2. Lấy chi tiết 1 proxy

```http
GET /api/v1/proxies/{id}
```

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID proxy |

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "a36a1e31-bdae-4566-95df-f246996a141c",
    "raw_proxy": "socks5://127.0.0.1:5001",
    "meta_data": null,
    "created_at": "2026-06-08T10:00:00Z",
    "updated_at": "2026-06-08T10:00:00Z",
    "tags": []
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 3.3. Tạo proxy

```http
POST /api/v1/proxies/create
```

Thêm proxy mới vào kho.

### Body params

| Field | Kiểu | Bắt buộc | Mô tả |
|---|---|---|---|
| `raw_proxy` | `string` | Có | Hỗ trợ `ip:port`, `ip:port:user:pass`, `http://...`, `socks5://...` |

### Request mẫu

```json
{
  "raw_proxy": "socks5://127.0.0.1:5001"
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "id": "...",
    "raw_proxy": "socks5://127.0.0.1:5001",
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 3.4. Cập nhật proxy

```http
POST /api/v1/proxies/update/{id}
```

Đổi chuỗi proxy của một bản ghi.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID proxy |

### Body params

| Field | Kiểu | Bắt buộc | Mô tả |
|---|---|---|---|
| `raw_proxy` | `string` | Có | Chuỗi proxy mới |

### Request mẫu

```json
{
  "raw_proxy": "http://user:pass@127.0.0.1:8080"
}
```

### Response mẫu

```json
{
  "success": true,
  "data": {
    "...": ""
  },
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 3.5. Xóa proxy

```http
GET /api/v1/proxies/delete/{id}
```

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID proxy |

### Response mẫu

```json
{
  "success": true,
  "data": null,
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

---

# 4. Extensions

## 4.1. Lấy danh sách extension

```http
GET /api/v1/extensions
```

Trả về toàn bộ extension đã cài, dạng mảng, không phân trang.

### Response mẫu

```json
{
  "success": true,
  "data": [
    {
      "id": "9d30b2b55768f1051833465850725851",
      "name": "uBlock Origin",
      "version": "1.54.0",
      "is_active": true
    }
  ],
  "message": "OK",
  "sender": "GPMLogin Global v1.0.0"
}
```

## 4.2. Bật/tắt extension

```http
GET /api/v1/extensions/update-state/{id}
```

Bật/tắt một extension. Có thể giới hạn áp dụng theo group cụ thể.

### Path params

| Param | Kiểu | Mô tả |
|---|---|---|
| `id` | `string` | ID extension |

### Query

| Query | Kiểu | Bắt buộc | Mặc định | Mô tả |
|---|---|---|---|---|
| `active` | `bool` | Có | — | `true` = bật, `false` = tắt |
| `applied_group_ids` | `string` | Không | — | Danh sách ID group ngăn cách bởi dấu phẩy, ví dụ `id1,id2`; bỏ qua = áp dụng toàn cục |

### Response mẫu

```json
{
  "success": true,
  "data": null,
  "message": "Extension state updated successfully",
  "sender": "GPMLogin Global v1.0.0"
}
```

---

# 5. Ví dụ nhanh

## Mở profile và lấy WebSocket URL

```bash
curl "http://localhost:9495/api/v1/profiles/start/PROFILE_ID?window_size=800,1000&window_pos=100,100&skip_proxy_check=true"
```

Response sẽ có:

```json
{
  "data": {
    "remote_debugging_port": 40444,
    "websocket_debugging_url": "ws://127.0.0.1:40444/devtools/browser/abc-123"
  }
}
```

## Đóng profile

```bash
curl "http://localhost:9495/api/v1/profiles/stop/PROFILE_ID"
```

## Tạo profile tối thiểu

```bash
curl -X POST "http://localhost:9495/api/v1/profiles/create" \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Test profile from API\"}"
```

## Tạo profile có proxy

```bash
curl -X POST "http://localhost:9495/api/v1/profiles/create" \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Profile with proxy\",\"raw_proxy\":\"socks5://127.0.0.1:5000\"}"
```

---

# 6. Ghi chú khi dùng với automation

- Endpoint quan trọng nhất cho Playwright/Puppeteer là `GET /profiles/start/{id}`.
- Với Playwright/Puppeteer, ưu tiên dùng `websocket_debugging_url`.
- Với Selenium, dùng `remote_debugging_port` để gắn vào Chrome đang chạy.
- Nếu cần cố định layout automation, truyền `window_size` và `window_pos` khi start profile.
- Nếu proxy đã được bạn tự kiểm tra trước, có thể dùng `skip_proxy_check=true` để mở profile nhanh hơn.
- Khi cập nhật profile, field bỏ qua hoặc `null` sẽ giữ nguyên giá trị cũ.
