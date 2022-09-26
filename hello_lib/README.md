```

$ find . -print | sed -e 's;[^/]*/;|____;g;s;____|; |;g'
.
|____hello_app
| |____Cargo.toml
| |____src
| | |____main.rs
|
|____hello_lib
| |____Cargo.toml
| |____src
| | |____lib.rs
| | |____foo.rs
| | |____foo
| | | |____bar.rs
| | |____main.rs
$

---

$ pwd
/Users/hello_rust
$ ls
hello_app  hello_lib
$

$ vi hello_app/Cargo.toml
[dependencies]
hello_lib = { path = "../hello_lib" }
$

---

// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

pub mod foo;

there should be a *.rs file with same basename: foo.rs

---

pub mod foo {
    //
}

inline, curly brackets instead of the semicolon

---

src/lib.rs: // crate root file
pub mod foo;

there should be src/foo.rs

---

src/foo.rs:
pub mod bar;

bar is a submodule of foo, there should be src/foo/bar.rs

```
