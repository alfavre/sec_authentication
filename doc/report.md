# Report Lab 2 SEC

## Author

```
Alban Favre
```

### Design

#### Database

the database will be simulated with multiple .txt files, notably cooldatabase, which stores passwords hash and usernames, token database whiche stores tokens.

#### Email

The email box will be simulated in a .txt file: coolmailbox.txt. It should be verified for the registratioon process and the password forgotten process.

#### Registration

To register the user needs to ask a registration token, which will be sent via mail.
On the email they can find the next steps.
If the email address is already in use, the mail will say so.
A token is only valid 15 minutes from its creation date.

If the registration process failed, the token will have been consumed, and the user will need to ask a new one.
But this will only happen if there are unexpected things happenning in the database, which should never happen.


#### Passwords

the password strength is verified with `zxcvbn` as `passablepasswords` is deprecated.

#### Token

Token are generated with 32 random bytes from sodium oxyde, it might not be enough.