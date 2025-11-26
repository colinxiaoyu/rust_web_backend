-- schema.sql

-- 创建数据库（可选，通常 migration 不包含这一项）
-- CREATE DATABASE rust_backend;

-- 切换数据库（psql 内部命令，不能放入 SQL 文件）
-- \c rust_backend;

-- 创建 users 表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);

-- 创建 tasks 表
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE
);
