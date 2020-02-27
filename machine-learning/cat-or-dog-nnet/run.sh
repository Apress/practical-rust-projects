#cargo run --bin generate_data > training.csv
#cargo run --bin generate_data > testing.csv

cargo run --bin train_and_predict -- --train training.csv --test testing.csv
