cargo run --bin generate > test_data.csv &&
cat test_data.csv | cargo run --bin plot &&
cat test_data.csv | cargo run --bin cluster > result.csv &&
cat result.csv | cargo run --bin plot_results
