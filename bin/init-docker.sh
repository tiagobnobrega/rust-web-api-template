#!/usr/bin/env sh
echo "Spin up docker image on background..."
docker-compose up --build -d

#WAIT_SEC=2
#echo "Waiting $WAIT_SEC seconds until db is ready..."
#sleep $WAIT_SEC
#
#DB_CONTAINER_NAME="local_postgres_db"
#POSTGRES_USER="$(docker exec $DB_CONTAINER_NAME printenv POSTGRES_USER)"
#APP_DB="$(docker exec $DB_CONTAINER_NAME printenv APP_DB)"
#
#echo "Creating project database on ${DB_CONTAINER_NAME} with user: ${POSTGRES_USER}..."
#docker exec -i $DB_CONTAINER_NAME psql -U "${POSTGRES_USER}" << SQL
#CREATE USER $APP_DB;
#CREATE DATABASE $APP_DB;
#GRANT ALL PRIVILEGES ON DATABASE $APP_DB TO $APP_DB;
#GRANT ALL PRIVILEGES ON DATABASE rustweb TO $POSTGRES_USER;
#SQL