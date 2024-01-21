# 使用 Rust 官方 Docker 映像作為基礎映像
FROM rust:1.67 as builder

# 安裝 sqlite3
RUN apt-get update && \
    apt-get install -y sqlite3 pkg-config

# 創建並設定工作目錄
WORKDIR /usr/src/study_space_optimizer

# 複製 Cargo.toml 和 Cargo.lock 檔案
COPY Cargo.toml Cargo.lock ./

# 創建一個假的 main.rs 來獲取依賴項
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 構建依賴項
RUN cargo build --release

# 複製程式碼和 SQL 檔案
COPY src ./src
COPY create_tables.sql ./

# 使用 sqlite3 執行 SQL 檔案，創建資料庫
RUN sqlite3 SSO.db3 < create_tables.sql

# DB路徑，用於 cargo build 時 sqlx 可以找的到資料庫
ENV DATABASE_URL=sqlite:./SSO.db3

# 構建應用程式
RUN touch src/main.rs && cargo build --release

# 第二階段：創建運行映像
FROM debian:bullseye-slim


RUN apt-get update && apt-get install -y openssl libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# 下载并编译安装 OpenSSL 3.0.0
# RUN wget
# RUN tar -xzf openssl-3.0.0.tar.gz && \
#     cd openssl-3.0.0 && \
#     ./config && \
#     make && make install
# RUN curl -O https://www.openssl.org/source/openssl-3.2.0.tar.gz && \
#     tar -xzf openssl-3.2.0.tar.gz && \
#     cd openssl-3.2.0 && \
#     ./config && \
#     make && \
#     make install

# 安装必要的构建工具和依赖
# RUN apt-get update && \
#     apt-get install -y build-essential curl ca-certificates && \
#     rm -rf /var/lib/apt/lists/*


# 複製執行檔和資料庫檔案
COPY --from=builder /usr/src/study_space_optimizer/target/release/study_space_optimizer /usr/src/study_space_optimizer/study_space_optimizer
COPY --from=builder /usr/src/study_space_optimizer/SSO.db3 /usr/src/study_space_optimizer/SSO.db3
COPY Rocket.toml /usr/src/study_space_optimizer/Rocket.toml

WORKDIR /usr/src/study_space_optimizer

# 設置環境變數
COPY .env.release /usr/src/study_space_optimizer/.env
ENV DATABASE_URL=sqlite:./SSO.db3


# 設定容器啟動時運行的命令
CMD ["/usr/src/study_space_optimizer/study_space_optimizer"]