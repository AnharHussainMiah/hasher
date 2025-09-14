## Hasher

If you're using `bcrypt` to has your passwords, and sometimes you really need to manually update the password,
and like me you've not yet completed the _"send email to reset password"_ functionality, then this CLI is a tiny life saver.

### Usage

Just compile and run the binary, it will prompt for a password, and then spit out the hash when you hit enter.

```shell
🔐 Enter the string to hash:
> foo

✅ Bcrypt hash: $2b$12$Jc9EG1j92wRd86VGgEsFJuhTzooxiHnjWFjUAE3c1mpEis1I.9gJa
```
