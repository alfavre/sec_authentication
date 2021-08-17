# sec_authentication

## Author
```
Alban Favre
```

### Intro

This is a practical rust work for school.
This was done for the 2021 sixth semester of HEIG-VD's bachelor SEC course.

### Objectives

Implement a 2 factor authentication mechanism:

- password
- `Google Authenticator` token

### Report

A report can be found in `doc/report.md` and `doc/report.pdf`

### Checkmarks

- [X] registration process
  - [X] Based on same token system as forgot password
  - [X] password
  - [X] email
    - [X] is used as unique identity on app
- [X] login process
  - [X] email
  - [X] password
  - [X] `Google Authenticator` token
    - [X] only when 2fa is active
- [X] password reset
  - [X] sends a password reset token
    - [X] pass token valid **15 minutes**
  - [X] uses `Google Authenticator` token
    - [X] only when 2fa is active
    - [X] if 2fa not active, only reset token is mandatory
- [X] a way to disable 2fa from a user perspective
- [X] Client data is stored securely
  - [X] Passwords hash are stored (they contain the salt)

### Notes

Everything can be simulated (email address, database, server)
