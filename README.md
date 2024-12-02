# How this stuff works

```bash
cargo build --release
sudo mkdir -p /usr/share/guile/site/3.0/
sudo rm -rf /usr/share/guile/site/3.0/rust*
sudo cp target/release/deps/libguile_rust_example.so /usr/share/guile/site
cat << 'EOF' | sudo tee /usr/share/guile/site/3.0/rust-example.scm
(define-module (rust-example)
  #:use-module (system foreign)
  #:use-module (rnrs bytevectors)
  #:export (rust-hello rust-add rust-free-string rust-string->scheme-string))

(define librust (dynamic-link "libguile_rust_example"))

(define rust-hello
  (pointer->procedure '*
                     (dynamic-func "rust_hello" librust)
                     '()))

(define rust-add
  (pointer->procedure int
                     (dynamic-func "rust_add" librust)
                     (list int int)))

(define rust-free-string
  (pointer->procedure void
                     (dynamic-func "rust_free_string" librust)
                     '(*)))

(define (rust-string->scheme-string ptr)
  (let ((str (pointer->string ptr)))
    (rust-free-string ptr)
    str))
EOF
guile   # run code below
```

Here is the Scheme code
```scheme
(use-modules (rust-example))
(define msg (rust-hello))
(display (rust-string->scheme-string msg))
(newline)
(display (rust-add 5 3))
(newline)
```
