# Required:
# sudo sysctl kernel.perf_event_mlock_kb=2048
# echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid

cargo build --profile profiling
samply record ./target/profiling/reventureregions