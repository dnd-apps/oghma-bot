# oghma-bot

Bot for D&amp;D to aid in some trivial tasks that can be tedious at the start of the session

## Current Planned Features

> More can be added, i just want to get this content added initially. 

- [ ] Nickname Manager
- [ ] Roll Tables

## Architecture

- Dgraph for the backend, this both is good for singleton and at scale. 
- Kubernetes for personal deployment but guide for docker-compose
- Bot access to backend. (command base first to get it working and accessible. BE & FE for mobile/desktop management)
- Backend APi with discord auth checks.  
- Frontend for easy management.
