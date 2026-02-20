# esp-sinker 🌐

A DNS sinkhole running on an ESP32 microcontroller, built with Rust and esp-idf-svc.

## What it does

The ESP32 creates its own Wi-Fi access point. Devices connect to it and all DNS queries go through the ESP32, which can block unwanted domains (ads, trackers) and forward legitimate ones upstream.

## Hardware

- ESP32 (any variant — tested on ESP32-U)
- USB cable for flashing

## Setup

### Prerequisites

```bash
# Install the ESP Rust toolchain
espup install

# Add to your shell profile
echo '. $HOME/export-esp.sh' >> ~/.zshrc
source ~/.zshrc

# Install flashing tools
cargo install espflash ldproxy
```

### Build & Flash

```bash
git clone <your-repo>
cd esp-sinker
cargo run
```

### Connect

1. On your device (phone, laptop), connect to Wi-Fi network **`esp-sinker`**
2. Password: **`12345678`**
3. Your device gets IP `192.168.71.2`, the ESP32 is at `192.168.71.1`

### Test

```bash
nslookup google.com 192.168.71.1
```

You should see the query appear in the serial monitor.

---

## What's implemented ✅

- **Wi-Fi Access Point** — ESP32 broadcasts its own network, DHCP assigns IPs to connected clients
- **UDP socket on port 53** — listens for incoming DNS queries
- **DNS packet parsing** — extracts the queried domain name from raw UDP bytes using `dns-parser`
- **Serial logging** — prints every DNS query to the monitor for debugging

## What's next 🔜

- **Blocklist check** — compare queried domain against `blocklist.txt` (embedded at compile time)
- **NXDOMAIN response** — craft and send a "domain not found" response for blocked domains
- **Upstream forwarding** — forward allowed queries to `1.1.1.1` and relay the response back
- **Subdomain matching** — block `ads.example.com` if `example.com` is in the blocklist

## Project structure

```
esp-sinker/
├── src/
│   └── main.rs        # Wi-Fi init, UDP loop, DNS parsing
├── blocklist.txt      # One domain per line, embedded at compile time
├── build.rs           # embuild linker setup
├── Cargo.toml
├── sdkconfig.defaults
└── .cargo/
    └── config.toml    # Target triple, linker config
```

## Dependencies

| Crate | Purpose |
|---|---|
| `esp-idf-svc` | Wi-Fi, NVS, peripherals |
| `dns-parser` | Parse raw DNS UDP packets |
| `anyhow` | Error handling |
| `log` | Logging |