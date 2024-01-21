# SSO-backend

### APIs:

#### 獲取狀態碼

- **HTTP 方法**: GET
- **路徑**: `/api/statuscode/<status_code>`
- **用途**: 根據提供的狀態碼，獲取對應的 HTTP 狀態描述。
- **輸入參數**:
  - `status_code` (int): 要獲取的 HTTP 狀態碼。
- **輸出**:
  - JSON 格式的 `StatusCodeResponse`，包含狀態碼和對應的標準描述。

#### 斷開資料庫連接

- **HTTP 方法**: GET
- **路徑**: `/api/disconnectdb`
- **用途**: 斷開與資料庫的連接。
- **輸入參數**: 無。
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。
  - 失敗時返回相應的錯誤狀態碼。

#### 處理超時

- **HTTP 方法**: GET
- **路徑**: `/api/timeout/<timeout>`
- **用途**: 實現指定時間的延遲以模擬超時行為。
- **輸入參數**:
  - `timeout` (int): 指定的延遲時間，單位為毫秒。
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。

#### 使用大量記憶體

- **HTTP 方法**: GET
- **路徑**: `/api/big_memory/<size>`
- **用途**: 分配指定大小的記憶體空間。
- **輸入參數**:
  - `size` (int): 欲分配的記憶體大小。
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。

#### 使用大量 CPU

- **HTTP 方法**: GET
- **路徑**: `/api/big_cpu`
- **用途**: 執行一個高 CPU 使用量的操作。
- **輸入參數**: 無。
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。

#### 設定不可預約時間

- **HTTP 方法**: POST
- **路徑**: `/api/timeslots/unavailable`
- **用途**: 設定特定時間段為不可預約。
- **輸入參數**:
  - `request` (JSON):
    - `timeslot` (JSON):
      - `start_time` (int)
      - `end_time` (int)
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。
  - 若用戶無權限或輸入不合法，則返回相應的錯誤狀態碼。

#### 設定不可使用座位

- **HTTP 方法**: POST
- **路徑**: `/api/seats/info`
- **用途**: 更新座位的資訊，包括其可用性及其他相關資訊。
- **輸入參數**:
  - `request` (JSON):
    - `seat_id` (int)
    - `available` (bool)
    - `other_info` (string)
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。
  - 若用戶無權限或輸入不合法，則返回相應的錯誤狀態碼。

#### 註冊

- **HTTP 方法**: POST
- **路徑**: `/api/users/register`
- **用途**: 讓新用戶註冊帳戶。
- **輸入參數**:
  - `RegisterRequest` (JSON):
    - `email`（string）
    - `password`（string）
    - `user_role`（string）。
- **輸出**:
  - 成功時返回用戶的驗證令牌。
  - 失敗時返回相應的錯誤狀態碼。

#### 重發驗證郵件

- **HTTP 方法**: GET
- **路徑**: `/api/users/resend_verification`
- **用途**: 重發帳戶驗證郵件給用戶。
- **輸入參數**:
  - 無。
- **輸出**:
  - 成功時返回新的驗證令牌。
  - 若不允許重發郵件或其他錯誤，則返回相應的錯誤狀態碼。

#### 登入

- **HTTP 方法**: POST
- **路徑**: `/api/users/login`
- **用途**: 讓用戶登入系統。
- **輸入參數**:
  - `LoginRequest` (JSON):
    - `email` (string)
    - `password` (string)
- **輸出**:
  - 成功時返回用戶的登入令牌。
  - 失敗時返回相應的錯誤狀態碼。

#### 預約座位

- **HTTP 方法**: POST
- **路徑**: `/api/reservations`
- **用途**: 讓用戶為指定座位預約特定時段。
- **輸入參數**:
  - `InsertReservationRequest` (JSON):
    - `seat_id` (int)
    - `timeslot` (JSON):
      - `start_time` (int)
      - `end_time` (int)
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。
  - 失敗時返回相應的錯誤狀態碼。

#### 刪除預約時段

- **HTTP 方法**: DELETE
- **路徑**: `/api/reservations/<reservation_id>`
- **用途**: 允許用戶刪除他們的預約時段。
- **輸入參數**:
  - `reservation_id` (int)
- **輸出**:
  - 成功時無輸出，僅返回成功狀態碼。
  - 失敗時返回相應的錯誤狀態碼。

#### 顯示使用者預約時段

- **HTTP 方法**: GET
- **路徑**: `/api/users/reservations`
- **用途**: 為用戶展示他們所有的預約時段。
- **輸入參數**: 無。
- **輸出**:
  - 成功時返回用戶所有預約時段的列表（JSON 格式）。
  - 失敗時返回相應的錯誤狀態碼。

#### 查詢當前特定位置預約狀態

- **HTTP 方法**: GET
- **路徑**: `/api/seats/<seat_id>/reservations/<start_time>/<end_time>`
- **用途**: 查詢特定座位在特定時段的預約狀態。
- **輸入參數**:
  - `seat_id` (int): 要查詢的座位編號。
  - `start_time` (int): 查詢時段的開始時間。
  - `end_time` (int): 查詢時段的結束時間。
- **輸出**:
  - 成功時返回該時段座位預約狀態的列表（JSON 格式）。
  - 失敗時返回相應的錯誤狀態碼。

#### 查詢當前所有位置狀態

- **HTTP 方法**: GET
- **路徑**: `/api/seats/status`
- **用途**: 查詢當前所有座位的狀態。
- **輸入參數**: 無。
- **輸出**:
  - 成功時返回所有座位的狀態總覽（JSON 格式）。
  - 失敗時返回相應的錯誤狀態碼。

#### 查詢當前所有位置狀態 + filter

- **HTTP 方法**: GET
- **路徑**: `/api/seats/status/<start_time>/<end_time>`
- **用途**: 根據指定的時間段，查詢座位的狀態。
- **輸入參數**:
  - `start_time` (int)
  - `end_time` (int)
- **輸出**:
  - 成功時返回該時段內所有座位的狀態總覽（JSON 格式）。
  - 失敗時返回相應的錯誤狀態碼。
