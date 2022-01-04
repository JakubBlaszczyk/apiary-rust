base_address="D:\\Programming\\apiary-rust\\"
cd ${base_address}
sqlx migrate run
services=($(ls | grep svc-))
echo ${services[*]}
for s in ${services[*]}
do
  cd ${s}
  cargo run &
  cd ..
done
cd ../server
sleep 30
rover supergraph compose --config ./supergraph-config.yaml > supergraph.graphql