#### #Get Balance (Xem số dư)

**Method: GET**

**URL:** [https://proxy.mkvn.net/api/apiv1/getbalance?token=xxxyyyzzz](https://proxy.mkvn.net/api/apiv1/getbalance?token=xxxyyyzzz)

**Dữ liệu đầu vào:** token (bắt buộc)

**Kết quả:**

```{
    "status": "SUCCESS",    
    "statusCode": 200,    
    "username": "usera",    
    "level": "PARTNER",    
    "balance": 42720,    
    "chietkhau": 20  
    }   
```

#### #Get Products (thông tin toàn bộ sản phẩm, hàng tồn kho)

**Method: GET**

**URL:** [https://proxy.mkvn.net/api/apiv1/products?token=xxxyyyzzz](https://proxy.mkvn.net/api/apiv1/products?token=xxxyyyzzz)

**Kết quả:**

`   {    "status": "SUCCESS",    "statusCode": 200,    "data": [      {        "id_product": "271",        "name_product": "(SP0) - GÓI 10 PROXY IPV6 - FREE",        "price": "10000000",        "type": "IPV6",        "countrycode": "vn",        "description": "GÓI FREE 1 THÁNG, VUI LÒNG VÀO NHÓM ZALO NHẬN (KHÔNG MUA)",        "buy_max": "1000",        "buy_min": null,        "catalogue": "SP01",        "store_quantity": 20      }]  }   `

#### #Get List Orders

**Method: GET**

**URL:** [https://proxy.mkvn.net/api/apiv1/getlistorders?token=xxxyyyzzz](https://proxy.mkvn.net/api/apiv1/getlistorders?token=xxxyyyzzz)

**Dữ liệu đầu vào:** token (bắt buộc)

**Kết quả:**

`   {    "status": "SUCCESS",    "statusCode": 200,    "total_orders": 25,    "status_summary": {      "ON": 0,      "OFF": 20,      "EXPIRE": 5    },    "data": [      {        "username": "usera",        "code": "010924C4AE66BF2013",        "name_product": "(SP01) - HÀ NỘI - GÓI 1 PROXY IPV6 - FACEBOOK",        "quantity": 2,        "price": 2080,        "time_buy": "2024-09-01 18:10:44",        "time_dau_ky": "2025-12-01",        "time_cuoi_ky": "2025-12-31",        "time_con_lai": 485,        "renewal": "OFF",        "note": null,        "type": "IPV6"      }]  }   `

#### #Proxies (lấy proxy từ đơn hàng)

**Method: GET**

**URL:** [https://proxy.mkvn.net/api/apiv1/proxies?token=xxxyyyzzz&ordercode=020924EA8A022B82C5](https://proxy.mkvn.net/api/apiv1/proxies?token=xxxyyyzzz&ordercode=020924EA8A022B82C5)

**Dữ liệu đầu vào:** token (bắt buộc), ordercode (bắt buộc)

**Kết quả:**

`   {    "status": "SUCCESS",    "statusCode": 200,    "message": "Truy vấn thành công!",    "order_code": "020924EA8A022B82C5",    "proxies": [      "hanoi108.proxy.mkvn.net:10378:FS3pG:12345",      "hanoi108.proxy.mkvn.net:10379:l2wCW:12345"    ],    "proxiesip": [      "103.89.142.95:10378:FS3pG:12345",      "103.89.142.95:10379:l2wCW:12345"    ]  }   `

#### #Buy (mua)

**Method: POST**

**URL:** [https://proxy.mkvn.net/api/apiv1/buy?token=xxxyyyzzz&id\_product=15&quantity=2&renewal=off¬e=khách số 1](https://proxy.mkvn.net/api/apiv1/buy?token=xxxyyyzzz&id_product=15&quantity=2&renewal=off&note=khách số 1)

**Yêu cầu:** Cấp Partner

**Dữ liệu đầu vào:** token (bắt buộc), id\_product (bắt buộc), quantity (bắt buộc), renewal (tùy chọn, mặc định "ON"), note (tùy chọn)

**Kết quả:**

```   {    "status": "SUCCESS",    "statusCode": 200,    "message": "Giao dịch thành công!",    "order_code": "020924EA8A022B82C5",    "name_product": "(SP01) - HÀ NỘI - GÓI 1 PROXY IPV6 - FACEBOOK",    "id_product": 15,    "value_order": 2080,    "renewal": "OFF",    "note": "khách số 1",    "quantity": 2,    "proxies": [      "hanoi108.proxy.mkvn.net:10378:FS3pG:12345",      "hanoi108.proxy.mkvn.net:10379:l2wCW:12345"    ],    "proxiesip": [      "103.89.142.95:10378:FS3pG:12345",      "103.89.142.95:10379:l2wCW:12345"    ],    "timeday": "02-09-2024 09:44:44",    "timestamp": 1725245084  }   ```

#### #Renewal Plus (Cộng Tháng)

**Method: POST**

**URL:** [https://proxy.mkvn.net/api/apiv1/renewalplus?token=xxxyyyzzz&ordercode=020924EA8A022B82C5&month=1](https://proxy.mkvn.net/api/apiv1/renewalplus?token=xxxyyyzzz&ordercode=020924EA8A022B82C5&month=1)

**Dữ liệu đầu vào:** token (bắt buộc), ordercode (bắt buộc), month (bắt buộc)

**Kết quả:**

`   {    "status": "SUCCESS",    "statusCode": 200,    "message": "Gia hạn thành công!",    "order_code": "020924EA8A022B82C5",    "value_order": 2080,    "new_expiry_time": "02-10-2024",    "remaining_balance": 3985440  }   `

#### #Renewal On/Off (Bật/Tắt Gia Hạn Theo Đơn)

**Method: POST**

**URL:** [https://proxy.mkvn.net/api/apiv1/renewalonoff?token=xxxyyyzzz&ordercode=020924EA8A022B82C5&renewal=off](https://proxy.mkvn.net/api/apiv1/renewalonoff?token=xxxyyyzzz&ordercode=020924EA8A022B82C5&renewal=off)

**Dữ liệu đầu vào:** token (bắt buộc), ordercode (bắt buộc), renewal (bắt buộc) on hoặc off

**Kết quả:**

`   {    "status": "SUCCESS",    "statusCode": 200,    "message": "Cập nhật trạng thái gia hạn thành công.",    "order_code": "020924EA8A022B82C5",    "renewal": "OFF"  }   `