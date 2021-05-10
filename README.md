# sec_authentication

## Author Alban Favre

### Intro

This is a practical rust work for school.

### Objectives

Implement a 2 factor authentication mechanism:

- password
- `Google Authenticator` token

### Checkmarks

- [ ] registration process
  - [ ] password
  - [ ] email
    - [ ] is used as a uname and a way to reset pass
- [ ] login process
  - [ ] email
  - [ ] password
  - [ ] `Google Authenticator` token
    - [ ] only when 2fa is active
- [ ] password reset
  - [ ] sends a password reset token
    - [ ] pass token valid **15 minutes**
  - [ ] uses `Google Authenticator` token
    - [ ] only when 2fa is active
- [ ] a way to disable 2fa for a user
  - [ ] **NOT** the other way around
- [ ] Client data is stored securely

### Notes

Everything can be simulated (email address, database, server)
