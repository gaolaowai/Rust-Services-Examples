# Logging service
Once your organization and application grows beyond a certain size, you have to start logging. Learning your services are down from a customer email or being unable to trace what happened when it did go down are sources of life-shortening stress, especially as the stakes of failure/downtime go up.

Ideally apps never die and we're perfect programmers. But we don't live in the ideal world. Instead, we're all human, and even if we were perfect programmers, we don't always have full control over the systems where our code is running. We can't make many assumptions about environment stability (servers, networks, storage, etc.), so the best we can do is to program in a defensive manner, assuming that things will go wrong, and working to mitigate the impacts of that when it does happen. Some languages and their VMs specialize in this, such as Erlang/Elixir's BEAM VM, where failure is just accepted to happen.

To help with mitigation and post-hoc improvement, we gotta log stuff.

## components
* client module --> this may write to local file and/or stream logging data to a server
* server module --> stream in and store logs; provide some facility to parse/search and potentially carry out some automated actions.
