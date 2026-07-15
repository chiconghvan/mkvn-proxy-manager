# GPMLogin API Document

> Crawl date: 2026-06-26  
> Requested source: `https://api-docs.gpmloginapp.com/`  
> Public API content found/crawled from: `https://docs.gpmloginapp.com/api-document`  
> Local API base URL used in examples: `http://127.0.0.1:19995`

## Tổng quan

API giúp bên thứ ba quản lý, thêm, sửa, xóa profiles trên GPM-Login, đồng thời có thể mở hoặc đóng profile một cách an toàn với đầy đủ thông số thiết bị đã tạo.

### Dành cho người mới

Các bước kết nối với trình duyệt của GPM-Login thông qua API:

1. Gọi API mở profile.
2. Sử dụng các thông số mà API trả về như `browser_path` / `browser_location`, `driver_path`, `remote_debugging_port` / `remote_debugging_address` để khởi tạo kết nối với Selenium hoặc Puppeteer.
3. Sử dụng code automation bình thường như các loại trình duyệt khác.

## Danh mục endpoint

| Chức năng | Method | Endpoint |
|---|---:|---|
| Danh sách profiles | GET | `/api/v3/profiles` |
| Lấy thông tin profile | GET | `/api/v3/profile/{id}` |
| Tạo profile | POST | `/api/v3/profiles/create` |
| Mở profile | GET | `/api/v3/profiles/start/{id}` |
| Đóng profile | GET | `/api/v3/profiles/close/{id}` |
| Cập nhật profile | POST | `/api/v3/profiles/update/{profile_id}` |
| Xóa profile | GET | `/api/v3/profiles/delete/{profile_id}` |
| Danh sách nhóm | GET | `/api/v3/groups` |

---

## 1. Danh sách profiles

### API URL

```http
GET /api/v3/profiles
```

### Params

| Tên param | Bắt buộc | Mô tả |
|---|---|---|
| `group_id` | Không | ID group cần lọc. Lấy tại API Danh sách nhóm. |
| `page` | Không | Số trang. Mặc định `1`. |
| `per_page` | Không | Số profile mỗi trang. Mặc định `50`. |
| `sort` | Không | `0` - Mới nhất, `1` - Cũ tới mới, `2` - Tên A-Z, `3` - Tên Z-A. |
| `search` | Không | Từ khóa profile name. |

### Ví dụ

```http
GET http://127.0.0.1:19995/api/v3/profiles?group=Ebay&page=1&per_page=100
```

### Response

```json
{
    "success": true,
    "data": [
        {
            "id": "ID_OF_PROFILE",
            "name": "NAME_OF_PROFILE",
            "raw_proxy": "RAW_PROXY",
            "browser_type": "chromium / firefox",
            "browser_version": "BROWSER_VERSISON",
            "group_id": "ID_OF_GROUP",
            "profile_path": "Local path or S3",
            "note": "",
            "created_at": "DATE"
        }
    ],
    "pagination": {
        "total": 7,
        "page": 1,
        "page_size": 100,
        "total_page": 1
    },
    "message": "OK"
}
```

---

## 2. Lấy thông tin profile

### API URL

```http
GET /api/v3/profile/{id}
```

> Ghi chú: Tài liệu gốc ghi API URL là `/api/v3/profile/{id}`, nhưng ví dụ lại dùng `/api/v3/profiles/{id}`. Khi implement nên test cả hai nếu endpoint singular không chạy.

### Ví dụ

```http
GET http://127.0.0.1:19995/api/v3/profiles/929e187c-2da7-4ecb-b3dd-9600e211fa4f
```

### Response

```json
{
    "success": true,
    "data": {
        "id": "ID_OF_PROFILE",
        "name": "NAME_OF_PROFILE",
        "raw_proxy": "RAW_PROXY",
        "browser_type": "chromium / firefox",
        "browser_version": "BROWSER_VERSISON",
        "group_id": "ID_OF_GROUP",
        "profile_path": "Local path or S3",
        "note": "",
        "created_at": "DATE"
    },
    "message": "OK"
}
```

---

## 3. Tạo profile

### API URL

```http
POST /api/v3/profiles/create
```

### Post data

```json
{
    "profile_name": "Test profile",
    "group_name": "All",
    "browser_core": "chromium",
    "browser_name": "Chrome",
    "browser_version": "119.0.6045.124",
    "is_random_browser_version": false,
    "raw_proxy": "",
    "startup_urls": "",
    "is_masked_font": true,
    "is_noise_canvas": false,
    "is_noise_webgl": false,
    "is_noise_client_rect": false,
    "is_noise_audio_context": true,
    "is_random_screen": false,
    "is_masked_webgl_data": true,
    "is_masked_media_device": true,
    "is_random_os": false,
    "os": "Windows 11",
    "webrtc_mode": 2,
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"
}
```

### Giải thích thông số

| Tên trường | Bắt buộc | Mô tả |
|---|---|---|
| `profile_name` | Có | Tên của profile. |
| `group_name` | Không | Tên của group. |
| `browser_name` | Không | `Chrome`, `Firefox`. Mặc định `Chrome`. |
| `browser_core` | Không | `chromium`, `firefox`. Mặc định `chromium`. |
| `browser_version` | Không | Phiên bản browser. |
| `raw_proxy` | Không | HTTP proxy: `IP:Port:User:Pass`; Socks5: `socks5://IP:Port:User:Pass`; TMProxy: `tm://API_KEY|True,False`; TinProxy: `tin://API_KEY|True,False`; TinsoftProxy: `tinsoft://API_KEY|True,False`. |
| `startup_urls` | Không | URL khởi động, ví dụ: `Url 1, Url 2, Url 3`. |
| `is_noise_canvas` | Không | Mặc định `false`. |
| `is_noise_webgl` | Không | Mặc định `false`. |
| `is_noise_client_rect` | Không | Mặc định `false`. |
| `is_noise_audio_context` | Không | Mặc định `true`. |
| `is_random_screen` | Không | Mặc định `false`. |
| `is_masked_webgl_data` | Không | Mặc định `true`. |
| `is_masked_media_device` | Không | Mặc định `true`. |
| `is_random_browser_version` | Không | Mặc định `false`. |
| `is_random_os` | Không | Mặc định `false`. |
| `os` | Không | Tên hệ điều hành chỉ định, nhập đúng như trên app. |
| `webrtc_mode` | Không | `1` - Off, `2` - Base on IP. Mặc định `2`. |
| `is_masked_font` | Không | Mặc định `true`. |
| `user_agent` | Không | User-Agent tùy chỉnh. |

### Response

```json
{
    "success": true,
    "data": {
        "id": "781d8439-b4c4-4203-a434-c853228110b1",
        "name": "Test profile",
        "raw_proxy": "",
        "profile_path": "wBPmeDpCbL-04122023",
        "browser_type": "Chrome",
        "browser_version": "119.0.6045.124",
        "note": null,
        "group_id": 1,
        "created_at": "2023-12-04T21:33:37.1200267+07:00"
    },
    "message": "OK"
}
```

---

## 4. Mở profile

### API URL

```http
GET /api/v3/profiles/start/{id}
```

### Params

| Tên param | Bắt buộc | Mô tả |
|---|---|---|
| `addination_args` | Không | Các param khởi động cùng trình duyệt. Cần hiểu rõ về trình duyệt mới nên dùng thông số này. |
| `win_scale` | Không | Giá trị từ `0` tới `1.0`. |
| `win_pos` | Không | Tọa độ trình duyệt theo dạng `x,y`. |
| `win_size` | Không | Kích thước cửa sổ theo dạng `width,height`. |

### Ví dụ

```http
GET http://127.0.0.1:19995/api/v3/profiles/start/xgyasg1995?win_scale=0.8&win_pos=300,300
```

Ví dụ mở với kích thước cửa sổ:

```http
GET http://127.0.0.1:19995/api/v3/profiles/start/xgyasg1995?win_size=800,1000
```

### Response

```json
{
    "success": true,
    "data": {
        "success": false,
        "profile_id": "17169ef5-761a-4fc4-9fba-2b634424c8c9",
        "browser_location": "C:\\Users\\buidu\\AppData\\Local\\Programs\\GPMLogin\\gpm_browser\\gpm_browser_chromium_core_119\\chrome.exe",
        "remote_debugging_address": "127.0.0.1:53378",
        "driver_path": "C:\\Users\\buidu\\AppData\\Local\\Programs\\GPMLogin\\gpm_browser\\gpm_browser_chromium_core_119\\gpmdriver.exe"
    },
    "message": "OK"
}
```

### Gợi ý dùng với automation

Sau khi gọi API mở profile, dùng `remote_debugging_address` để attach bằng CDP/Playwright/Puppeteer, hoặc dùng `driver_path` với Selenium.

---

## 5. Đóng profile

### API URL

```http
GET /api/v3/profiles/close/{id}
```

### Ví dụ

```http
GET http://127.0.0.1:19995/api/v3/profiles/close/xgyasg1995
```

### Response

```json
{
    "success": true,
    "message": "Đóng thành công"
}
```

---

## 6. Cập nhật profile

### API URL

```http
POST /api/v3/profiles/update/{profile_id}
```

### Post data

```json
{
    "profile_name": "NAME_OF_PROFILE",
    "group_id": 1,
    "raw_proxy": "",
    "startup_urls": "",
    "note": "",
    "color": "COLOR_HEX",
    "user_agent": "auto",
    "is_noise_canvas": false,
    "is_noise_webgl": false,
    "is_noise_client_rect": false,
    "is_noise_audio_context": true
}
```

### Giải thích thông số

Các thông số không bắt buộc hoặc không muốn thay đổi thì không cần đưa vào post data.

| Tên trường | Bắt buộc | Mô tả |
|---|---|---|
| `name` / `profile_name` | Có | Tên của profile. Tài liệu bảng ghi `name`, ví dụ JSON dùng `profile_name`. |
| `group` / `group_id` | Không | Group của profile. Tài liệu bảng ghi `group`, ví dụ JSON dùng `group_id`. |
| `raw_proxy` | Không | HTTP proxy: `IP:Port:User:Pass`; Socks5: `socks5://IP:Port:User:Pass`; TMProxy: `tm://API_KEY|True,False`; TinProxy: `tin://API_KEY|True,False`; TinsoftProxy: `tinsoft://API_KEY|True,False`. |
| `startup_urls` | Không | URL khởi động, ví dụ: `Url 1, Url 2, Url 3`. |
| `note` | Không | Ghi chú. |
| `color` | Không | Mã hex màu profile. |
| `user_agent` | Không | `auto` hoặc tự điền. |
| `is_noise_canvas` | Không | Xem chi tiết tại API tạo profile. |
| `is_noise_webgl` | Không | Xem chi tiết tại API tạo profile. |
| `is_noise_client_rect` | Không | Xem chi tiết tại API tạo profile. |
| `is_noise_audio_context` | Không | Xem chi tiết tại API tạo profile. |

### Response

```json
{
    "success": true,
    "message": "OK",
    "data": {}
}
```

---

## 7. Xóa profile

### API URL

```http
GET /api/v3/profiles/delete/{profile_id}
```

### Param

| Param | Mô tả |
|---|---|
| `mode` | `1` - chỉ xóa ở database; `2` - xóa cả database và nơi lưu trữ. Ví dụ: `/api/v3/profiles/delete/123-456-789?mode=2`. |

### Response

```json
{
    "success": true,
    "data": null,
    "message": "Xóa thành công"
}
```

---

## 8. Danh sách nhóm

### API URL

```http
GET /api/v3/groups
```

### Response

```json
{
    "success": true,
    "data": [
        {
            "id": 1,
            "name": "All",
            "sort": 1,
            "created_by": -1,
            "created_at": "2023-11-13T15:50:35.6516111-08:00",
            "updated_at": "2023-11-13T15:50:35.6516111-08:00"
        }
    ],
    "message": "OK"
}
```

---

## Mẫu code nhanh

### JavaScript: gọi API mở profile và lấy debugging address

```js
const BASE_URL = 'http://127.0.0.1:19995';

async function startProfile(profileId) {
  const url = `${BASE_URL}/api/v3/profiles/start/${profileId}?win_size=800,1000`;
  const res = await fetch(url);
  const json = await res.json();

  if (!json.success) {
    throw new Error(json.message || 'Start profile failed');
  }

  return json.data;
}

startProfile('PROFILE_ID_HERE')
  .then(data => {
    console.log('Remote debugging:', data.remote_debugging_address);
    console.log('Driver path:', data.driver_path);
  })
  .catch(console.error);
```

### Python: gọi API mở profile

```python
import requests

BASE_URL = "http://127.0.0.1:19995"


def start_profile(profile_id: str):
    url = f"{BASE_URL}/api/v3/profiles/start/{profile_id}"
    params = {
        "win_size": "800,1000"
    }
    response = requests.get(url, params=params, timeout=30)
    response.raise_for_status()
    data = response.json()

    if not data.get("success"):
        raise RuntimeError(data.get("message", "Start profile failed"))

    return data["data"]


if __name__ == "__main__":
    result = start_profile("PROFILE_ID_HERE")
    print("Remote debugging:", result.get("remote_debugging_address"))
    print("Driver path:", result.get("driver_path"))
```

## Ghi chú khi dùng thực tế

- API local thường chạy ở port `19995` trên `127.0.0.1`.
- Với automation Chromium, trường quan trọng nhất sau khi mở profile là `remote_debugging_address`.
- Với Selenium, dùng `driver_path` do API trả về.
- Một số tên trường trong docs gốc có chỗ chưa thống nhất, ví dụ `profile`/`profiles`, `name`/`profile_name`, `group`/`group_id`. Nên test trực tiếp trên bản GPMLogin đang cài.
