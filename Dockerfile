# 构建阶段
FROM rust:1.75-slim as builder

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 安装 trunk
RUN cargo install trunk --version 0.18.0

# 安装 wasm 目标
RUN rustup target add wasm32-unknown-unknown

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件（利用 Docker 缓存）
COPY Cargo.toml Cargo.lock ./

# 复制源码
COPY src ./src
COPY index.html ./
COPY Trunk.toml ./

# 构建生产版本
RUN trunk build --release

# 运行阶段 - 使用 nginx 提供静态文件
FROM nginx:alpine

# 复制构建产物到 nginx 目录
COPY --from=builder /app/dist /usr/share/nginx/html

# 复制自定义 nginx 配置
COPY nginx.conf /etc/nginx/conf.d/default.conf

# 暴露端口
EXPOSE 80

# 启动 nginx
CMD ["nginx", "-g", "daemon off;"]
