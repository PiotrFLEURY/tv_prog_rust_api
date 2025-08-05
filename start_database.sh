docker run --name tvprogdb \
  --rm \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres \
  -d \
  postgres:latest