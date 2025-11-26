### 使用方式
1. 启动 docker
docker compose up -d
2. 查看日志：
docker compose logs -f postgres
3. 进入数据库：
docker exec -it postgres psql -U colin -d rust_backend


