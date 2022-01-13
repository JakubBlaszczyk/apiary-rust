cargo install --list | grep sqlx || cargo install sqlx
docker port postgres-db || docker run --rm --name postgres-db -p 5432:5432 -e POSTGRES_PASSWORD=mypassword -d postgres
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd ${script_dir}
cd ..
services=($(ls | grep svc-))
echo ${services[*]}
sqlx migrate run
for s in ${services[*]}
do
  cd ${s}
  cargo run &
  cd ..
done
cd server
sleep 15
rover supergraph compose --config ./supergraph-config.yaml > supergraph.graphql
echo "Done"