
main()
  └── init wifi (esp-idf-svc)
  └── bind UdpSocket to 0.0.0.0:53
  └── loop:
        recv_from() → raw DNS bytes
        parse QNAME manually (or dns-parser crate)
        check blocklist
        if blocked → craft NXDOMAIN response bytes, send_to()
        if allowed → forward to 1.1.1.1:53, relay response back
