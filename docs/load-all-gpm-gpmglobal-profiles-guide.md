# Hướng dẫn API GPM và GPM Global - Load All Profiles

Tài liệu này hướng dẫn cách sử dụng API để lấy toàn bộ danh sách profile từ GPM Standard và GPM Global.

---

## 1. GPM Standard (API v3)

GPM Standard thường chạy tại cổng `19995`. API sử dụng cơ chế phân trang (pagination) để quản lý danh sách profile lớn.

### Thông tin API
- **Base URL**: `http://127.0.0.1:19995/api/v3`
- **Endpoint**: `/profiles`
- **Method**: `GET`
123456

### Tham số (Query Parameters)
| Tham số | Mô tả | Mặc định gợi ý |
| :--- | :--- | :--- |
| `page` | Số thứ tự trang cần lấy (bắt đầu từ 1) | 1 |
| `per_page` | Số lượng profile mỗi trang | 100 |

### Cấu trúc phản hồi (Response)
Dữ liệu trả về có cấu trúc như sau:
```json
{
    "success": true,
    "data": [
        {
            "id": "uuid-của-profile",
            "name": "Tên Profile",
            "raw_proxy": "ip:port:user:pass",
            "group_id": 1
        }
    ],
    "pagination": {
        "total_page": 5,
        "current_page": 1
    }
}
```

### Lấy danh sách Group
Để hiển thị tên Group thay vì chỉ ID, bạn cần lấy danh sách Group trước:
- **Endpoint**: `/groups`
- **Method**: `GET`
- **Cấu trúc phản hồi**:
```json
{
    "success": true,
    "data": [
        {
            "id": 1,
            "name": "Group Mặc Định"
        },
        {
            "id": 2,
            "name": "Group Công Việc"
        }
    ]
}
```
**Logic**: Sau khi lấy danh sách profile, hãy so khớp `group_id` của profile với `id` trong danh sách group để lấy `name` (Tên Group).

### Cách load tất cả profile
1. Bắt đầu với `page = 1`.
2. Gọi API `GET /profiles?page=1&per_page=100`.
3. Lưu dữ liệu từ trường `data` vào danh sách tổng.
4. Kiểm tra `total_page` trong trường `pagination`.
5. Nếu `page < total_page`, tăng `page` lên 1 và lặp lại bước 2.
6. Dừng lại khi đã lấy hết số trang.

---

## 2. GPM Global (API v1)

GPM Global thường chạy tại cổng `9495`. Cấu trúc API và phản hồi có sự khác biệt so với bản Standard.

### Thông tin API
- **Base URL**: `http://127.0.0.1:9495/api/v1`
- **Endpoint**: `/profiles`
- **Method**: `GET`

### Tham số (Query Parameters)
| Tham số | Mô tả | Mặc định gợi ý |
| :--- | :--- | :--- |
| `page` | Số thứ tự trang cần lấy (bắt đầu từ 1) | 1 |
| `per_page` | Số lượng profile mỗi trang | 100 |
| `page_size`| Tương tự per_page (nên gửi cả hai) | 100 |

### Cấu trúc phản hồi (Response)
Lưu ý trường `data` bao bọc một object chứa thông tin phân trang:
```json
{
    "success": true,
    "data": {
        "current_page": 1,
        "per_page": 100,
        "last_page": 10,
        "data": [
            {
                "id": "uuid-của-profile",
                "name": "Tên Profile Global",
                "raw_proxy": "proxy-string",
                "group_id": "group-uuid"
            }
        ]
    },
    "message": "OK",
    "sender": "GPMLoginGlobal"
}
```

### Lấy danh sách Group
GPM Global cũng sử dụng phân trang cho danh sách Group:
- **Endpoint**: `/groups`
- **Method**: `GET`
- **Tham số**: `page`, `per_page`, `page_size` (tương tự như profiles)
- **Cấu trúc phản hồi**:
```json
{
    "success": true,
    "data": {
        "current_page": 1,
        "last_page": 1,
        "data": [
            {
                "id": "group-uuid",
                "name": "Tên Group Global"
            }
        ]
    }
}
```
**Logic**: Tương tự GPM Standard, hãy lưu danh sách Group vào một Dictionary (Map) với Key là `id` và Value là `name` để tra cứu nhanh khi hiển thị profile.

### Cách load tất cả profile
1. Bắt đầu với `page = 1`.
2. Gọi API `GET /profiles?page=1&per_page=100&page_size=100`.
3. Truy cập vào `data.data` để lấy danh sách profile.
4. Kiểm tra `data.last_page` để biết tổng số trang.
5. Nếu `page < last_page`, tăng `page` lên 1 và tiếp tục gọi API.
6. Dừng lại khi `page` đã đạt tới `last_page`.

---

**Lưu ý chung:** 
- Đảm bảo ứng dụng GPM hoặc GPM Global đang mở và tính năng API đã được bật.
- Cổng (Port) có thể thay đổi trong cài đặt của ứng dụng, hãy kiểm tra `settings.json` hoặc phần cài đặt nếu không kết nối được.
