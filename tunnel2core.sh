#!/usr/bin/env bash

ssh -L 30000:172.16.1.1:3000 -L 30001:172.16.1.2:3000 -L 30002:172.16.1.3:3000 -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -p 2000 root@127.0.0.1
