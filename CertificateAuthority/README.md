# CertificateAuthority Service
Why?

While it's very common to use NGINX or similar as a TLS terminator sitting on the edge of the network just past the firewall,
and not using TLS within a "trusted" network, the truth and reality of the modern world is that we can't really trust all the systems even on our own network.
Guest devices, unauthorized machines, compromised hosts, etc., all might potentially be listening and sniffing traffic for less-than-good purposes.

Giving your internal apps and services the ability to have their own organization CA-signed certificates can encrypt traffic and add at least an additional layer of trust that you're not
talking to some rogue server process on your network.


### For CA setup details
https://deliciousbrains.com/ssl-certificate-authority-for-local-https-development

### This service
With this service we aim to replicate the behavior of "Let's Encrypt" and their service so that within an organization for local servers, we can request HTTPS certificates.

We'll also be building out endpoints to get per-OS root CA installation scripts.


