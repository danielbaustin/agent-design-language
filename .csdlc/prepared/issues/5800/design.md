# Issue 5800 Design: Browser-Trusted Observatory HTTPS

Establish one supported browser-trusted HTTPS path for the local Observatory.
The certificate source, trust installation, renewal/reissue, configuration,
startup, URL, health checks, and documentation must agree. Chrome and command
line clients must prove the same endpoint without warning bypasses. The issue
does not use AWS and does not weaken TLS verification.
