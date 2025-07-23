docker compose up -d zookeeper kafka
# Wait for Kafka to be ready
until docker compose exec kafka \
  /opt/bitnami/kafka/bin/kafka-topics.sh --list --bootstrap-server kafka:9092; do
  echo "Waiting for Kafka broker to be ready..."
  sleep 2
done
# Create the test topic
docker compose exec kafka /opt/bitnami/kafka/bin/kafka-topics.sh \
       --create --if-not-exists --topic test \
       --bootstrap-server kafka:9092 \
       --partitions 1 --replication-factor 1
# Now start kafql-store
docker compose up -d kafql-store

# Wait for kafql-store API to be ready
for i in {1..30}; do
    echo "waiting for kafql-store"
    if curl -s http://localhost:3000/records | grep -q '\['; then
        echo "kafql-store ready"
        break
    fi
    sleep 2
done
