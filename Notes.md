## Personal comments
### Creating a new project
Let us create a new project and open it in VS Code
```
cargo new clinews
cd clinews
code .
```
In VSCode terminal we can run ```bacon```, an automatic Rust checker. 

We can add packages or "crates" using ```cargo add``` (to install this command we run ```cargo install cargo-edit```)


### Notes to code
To debug the code we can use the macro ```debug!("Hello")```

To let the compiler know that we have not finished the code but that we still want too compile we use the macro ```todo!()```