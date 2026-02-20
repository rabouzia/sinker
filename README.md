const BLOCKLIST_RAW: &str = include_str!("../blocklist.txt");
```

Then parse it once at startup into a `HashSet` in RAM — but be careful, 320KB total RAM means a large blocklist (StevenBlack's full list is ~200k domains) **will not fit**. You'll need a trimmed list or store it in flash with a different access pattern.

---

## What Your Architecture Looks Like on ESP32
```
main()
  └── init wifi (esp-idf-svc)
  └── bind UdpSocket to 0.0.0.0:53
  └── loop:
        recv_from() → raw DNS bytes
        parse QNAME manually (or dns-parser crate)
        check blocklist
        if blocked → craft NXDOMAIN response bytes, send_to()
        if allowed → forward to 1.1.1.1:53, relay response back
