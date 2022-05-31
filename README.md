## Getting started
### Pre-requisites
MAC OS: 
```shell
#postgresql
brew install postgresql
#diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```


### Set up docker local environment
Run start dev script. It should build the docker environment and spin up cargo watcher.
```shell
#run start dev script
brew install postgresql
#diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

If you need to recreate the local environment (not this will remove your database data), run the following:
```shell
#destroy instances with volumes
docker-compose down -v; rm -rf ./.data
#remove local .data folder
```

## Database Management
###Running migrations
```shell
diesel migration run
```
###revert a migration
```shell
diesel migration generate <MIGRATION_NAME>
```

###Create a new migration
```shell
diesel migration generate <MIGRATION_NAME>
```
This should create a new folder on the "migrations" directory.

It’s a good idea to make sure that down.sql is correct. You can quickly confirm that your down.sql rolls back your migration correctly by redoing the migration:
```shell
diesel migration redo
```

