# What is this?
Examples of different services written in Rust with minimal dependencies where possible.

# Why does this repo exist?
Working in IT and software development, there are repeated common components that I've bumped into time and time again.

These different components are commonly used services which occur in almost all companies. Many existing solutions and implementations of them exist, often using different licenses paid and free, written in a variety of languages (Golang, C/C++, Python, PHP, etc.), and frequently insecure and vulnerable to memory-related or outdated cipher issues.

Relying on external services is often cheaper initially than self-hosting, but once a system is set into place and running in production, is becomes nearly impossible to leave (i.e. using AWS/GCP/Azure cloud native services). Furthermore, the source code and implementation details of those services can't always be inspected or reviewed.

Finally, this repository is a personal exercise with which to challenge myself to build these components from scratch, using the Rust standard library, and very sparingly, a minimal set of dependencies for hash, encryption (curves), and random number generation, though I'm not against eventually implementing hashing and PRNG here as well.

The ambitious end goal is to have most of, and eventually all of the components needed to run a corporation's IT systems, in a way that can be most agnostic to the underlying OSes and hosting environment. Even better (and maybe dumb... we'll see) would be for there to be only a single executable binary for all these components, where given a configuration file and a "launchfromconfig" flag, a single executable could spin up an entire stack for itself, all in Rust.

Even if I can't meet that "One Binary to Rule Them All" goal, at the very least I trust that I'm capable of creating these individual components, as I've already created them in one form or another for various employers and clients in the last decade, and having the benefit of that experience and lacking any of that previous source code, I'm able to incorporate the sum total of my spread out experience from the past decade to, hopefully, avoid the sins of the past.

To you, dear reader, whomever you may be, I simply wish you a good day. Stick around, have a read, criticize my code, etc... Just thank you for coming.

-- Max

## Project Components
 - DNS --> controll your DNS, control your fate 
 - Proxy --> Catch and route requests to the appropriate handling service
 - Firewall --> Catch and route to black hole or return infinite garbage
 - CertificateAuthority --> Internally centralized certificate generation and authentication
 - Email --> SMTP (sending, receiving), IMAP/POP (client interactions), and basic web GUI for sending, composing, reading, etc.
 - NTP --> Maybe it will be the wrong (out of date) time, but at least all your services/systems are using the same timestamps, which makes investigating incidents much easier.
 - Logging --> Need to know what's going on with your stuff, the good, the bad, the ugly, as well as searching/parsing it out.
 - HTTP --> Always need HTTP(s) servers
 - ADAP (User account services: creation, management, role management, authentication, etc.)
 - Comms --> Chat, community posts, video/audio calling
 - DBs (KV store, KV in-memory cache, SQL-like, immutable record store --> blockchain)
 - Runner --> Sometimes you just need to run some jobs without caring who does it. Submit some work, get back results.
 - Scheduling --> Deceivingly complex, every organization needs scheduling, whether it be for resource management, parties, or just to manage personal time.
 - File server (bucket-style storage) --> Sometimes applications and services just need to store data blobs somewhere and access it in a variety of ways (SFTP, SSH, HTTPS, TLS-on-TCP)

## Principles
* Use minimum depencies. I want to avoid NPM/node style dependency trees with 10's of thousands of dependencies and giant build times. This not only helps make the code more secure from supply-side attacks (back-doored crates), but also makes it easier to certify/review if needed.
* Avoid the use of "unsafe" keyword and crates that contain it. This isn't absolute, but I should take pause and really consider it if I need to include the unsafe keyword or unsafe crates.
* When possible, keep it stupid simple (KISS). I don't need to support backwards compatibility from 20 year old deprecated standards. Don't need to support ancient cypher suites.
* Choose/generate sane defaults for services. Make it easy for the average IT guy to start it up, and require minimum amount of additional learning to be needed.
* Be easy to build, install, deploy --> Don't make setup a game of 20 questions.
* Overly comment code. Some folks have the opinion that the code should speak for itself, and sometimes they're right. But more often than not, I want someone who maybe isn't senior or fully experienced in Rust programming to be able to click through the source code and feel as if they understand, or could potentially understand what is going on. Don't assume that others are experts. Whenever I myself have to stop and deliberate between multiple different paths, to me, that marks a point where I should be documenting my decision making process, the alternatives I didn't choose, and why. After all, I'm human, and it's possible the road not taken ***actually*** was the correct path, but that we might not know it until several months years, or even decades after the fact. While working in codebases as Goldman Sachs, I particularly appreciated those tomes which did just this. Some programmer from 20 years ago didn't have to do it, but just when I thought I was being clever and could improve something, reading a good comment block from someone (or multiple someones sometimes) talking about how they also tried and failed to do something differently, probably saved weeks of wasted effort.
* Avoid linking to libraries. Sure this might make my binary fatter, but like Golang, I'd rather aim at easy portability, which I think more closely adheres to my flavor of KISS.
