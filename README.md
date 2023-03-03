# Notes app built in the Rust frontend framework Yew

In this repository i have written a simple note keeping Yew app using Tailwind as a CSS framework with the intent of trying out Rust frontend development as a JavaScript/TypeScript+React replacement.

To run the example just `trunk serve` in the repo's root, any Tailwind css in the project will be automatically compiled through this command as well.

Requires:
1. Trunk (cargo install trunk)
2. Adding wasm32 target to rustup (`rustup target add wasm32-unknown-unknown`)
3. Tailwind, PostCSS, autoprefixer (`npm install -D tailwindcss postcss autoprefixer`)

Screenshots:
![Home Screen](/screenshots/Home%20screen.png)
![Note editing](/screenshots/Note%20editor.png)