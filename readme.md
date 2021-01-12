# WARNING: this is a first draft and does not handle errors gracefully

# email missing students

The wife is now teaching online and has to email parents of kids to fail to
attend class.

Given 
- a csv of the class with name, email, and parent_email
- a csv of the attendees (name, email)
- an email template text file

compiles a list of missing students and prints out email to send to the parents.

## install

Requires [rust](https://www.rust-lang.org)

`cargo install --path .`

Then run `email-missing-students -h` for more info.
