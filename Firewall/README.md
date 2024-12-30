# Firewall
This will aim at two different levels:
1. Targetting Linux and MacOS to begin with this, we'll aim to create firewall.
2. Targetting the TCP/UDP level for packet inspection, create an application firewall module that we can use in our other network services (HTTP, NTP, Email, logging, Proxy, etc.)

* For goal 1 above, we'll need to rely on our first primary external library, pnet https://docs.rs/pnet/latest/pnet/
* For goal 2, we'll just be relying on std::lib's tcp/udp interfaces.
* The goal number (1 vs. 2) doesn't imply any order or importance. As a matter of fact, we'll be aiming at the second one first.

## Pnet-based firewall
The intent behind this is anomaly detection that would then lead us to update the system firewall rules. What some software has done in the past is just listen to the system logs. We *could* do that, but I'd still prefer tapping directly into the network device to sample packets coming through.
