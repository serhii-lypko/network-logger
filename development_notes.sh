
# ideas
# dns resolver -> geodata of requested dns? by ip and stuff
# how to track which request is incoming and which is outcoming?
# how to track the process id ? of the process which has been send or recieve packet

# TODO: index for MongoDB collection for the faster aggregation?


# -- MongoDB config info --

# create a container
docker run --name network-logger-mongodb -d -p 27017:27017 \
-e MONGO_INITDB_ROOT_USERNAME=admin \
-e MONGO_INITDB_ROOT_PASSWORD=ElaAbCQkZQbv9gpv \
mongo

# login to container
docker exec -it cc9ebff86ca3 bash
mongosh -u admin -p ElaAbCQkZQbv9gpv

# read operations
db.getCollectionNames()
db.movies.find()
