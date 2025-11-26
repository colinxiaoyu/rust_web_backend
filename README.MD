# Rust 小型 Web 后台服务项目规划

## 一、项目目标

创建一个小型 Web 后台服务，具备以下功能：

- 用户注册、登录（JWT 认证）
- 简单的任务管理（类似 Todo List）
- 提供 REST API 接口
- 数据存储在 PostgreSQL
- 使用 Rust 的 async 特性，提高并发处理能力

**项目目标**：学习 Rust 基础、Web 框架、异步处理、数据库访问和 API 开发。

---

## 二、技术栈

| 层次       | 技术                  | 说明                                   |
| ---------- | --------------------- | -------------------------------------- |
| Web 框架   | Axum                  | 简洁、现代 async 框架，易与 Tokio 配合 |
| 异步运行时 | Tokio                 | 支撑 async / await                     |
| 数据库     | PostgreSQL            | 关系型数据库，学习 SQL                 |
| ORM / SQL  | SQLx                  | 支持异步 SQL，编译时验证语法           |
| 序列化     | Serde                 | JSON 数据序列化/反序列化               |
| 身份认证   | jsonwebtoken (JWT)    | 用户认证和授权                         |
| 日志       | tracing               | 日志和调试                             |
| 测试       | tokio::test + reqwest | 异步测试 API                           |

---

## 三、项目模块划分

src/
├─ main.rs # 程序入口，启动服务器
├─ routes/ # 路由处理器
│ ├─ mod.rs
│ ├─ user.rs # 用户注册、登录
│ └─ task.rs # 任务管理 API
├─ services/ # 业务逻辑
│ ├─ mod.rs
│ ├─ user_service.rs
│ └─ task_service.rs
├─ models/ # 数据模型
│ ├─ mod.rs
│ ├─ user.rs
│ └─ task.rs
├─ db/ # 数据库连接、操作
│ └─ mod.rs
└─ utils/ # 工具函数
├─ jwt.rs
└─ hash.rs # 密码加密


---

## 四、主要 API

| 方法   | 路径       | 功能             | 备注           |
| ------ | ---------- | ---------------- | -------------- |
| POST   | /register  | 用户注册         | 密码加密存储   |
| POST   | /login     | 用户登录         | 返回 JWT token |
| GET    | /tasks     | 获取用户任务列表 | 需要 JWT       |
| POST   | /tasks     | 创建任务         | 需要 JWT       |
| PUT    | /tasks/:id | 更新任务         | 需要 JWT       |
| DELETE | /tasks/:id | 删除任务         | 需要 JWT       |

---

## 五、开发步骤

### 1. 环境准备
- 安装 Rust: `rustup install stable`
- 创建项目: `cargo new rust_web_backend`

### 2. 添加依赖到 `Cargo.toml`

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "8.3"
tracing = "0.1"
dotenvy = "0.15"


