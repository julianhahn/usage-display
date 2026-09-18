# ChatGPT TLS trust anchor

`gts-root-r4.der` is the self-signed Google Trust Services GTS Root R4 certificate, in DER format. It is public data, not a credential.

Source: the host system trust store, `/etc/ssl/certs/GTS_Root_R4.pem`. Publisher: https://pki.goog/repository/

SHA-256 fingerprint:

```text
34:9D:FA:40:58:C5:E2:63:12:3B:39:8A:E7:95:57:3C:4E:13:13:C8:3F:E6:8F:93:55:6C:D5:E8:03:1B:3C:7D
```

The board fetched NTP time and received HTTP 200 with this trust root and required hostname and certificate-date checks. A temporary build with the unrelated ISRG Root X1 instead was rejected with X.509 error `0x2700`. The working GTS Root R4 firmware was then restored and succeeded again. Evidence: `.logs/ntp-final-serial.log` and `.logs/tls-wrong-root-serial.log`.

Ordinary NTP is unauthenticated. This prototype trusts the network's time reply; it does not provide protection against an attacker who controls that time source.

Do not replace this root with a certificate taken from an unverified server connection. A future server chain change may require an updated trust anchor.
