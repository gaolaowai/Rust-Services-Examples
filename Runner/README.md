# Job runner service
Submit a task to a job queue, get a ticket/request number, and collect the results at a later time.

### components
* job format --> multi-part text, JSON format, support Python, BASH, and statically-linked binary (base64 encoded) execution
* job runner --> spawn job, collecting outputs for results retreival. Side-effects (i.e. script reaching out to DB or other files created) might not be tracked (consider some field in the job format to specify files/data to be collected/tracked). Multiple runners can be reading from the queue.
* job queue --> store and queue job requests, handling status requests, and returning results when finished.
* listener service --> accept job requests, return results (in response to status update requests)
