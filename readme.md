# Fake Radius auth
This is a pam module designed to add a random sleep and acceptance chance to requests to help us debug service auth flows without hitting the production Radius server.
This might get split into multiple pam modules, ex: `pam_random_sleep` and `pam_radom_deny`.

## setup
1. build the code
1. put so in `/lib/security/`
1. update pam config
