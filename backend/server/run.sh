cargo install --list | grep sqlx || cargo install sqlx
docker port postgres-db || echo "Creating container" && docker run --rm --name postgres-db -p 5432:5432 -e POSTGRES_PASSWORD=mypassword -d postgres && echo "Container created"
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd ${script_dir}
cd ..
services=($(ls | grep svc-))
echo ${services[*]}
sleep 1
sqlx migrate run
for s in ${services[*]}
do
  cd ${s}
  cargo build
  cargo run &
  cd ..
done
echo "Services started"
cd server
sleep 15
rover supergraph compose --config ./supergraph-config.yaml > supergraph.graphql
node index.js
echo "Done"