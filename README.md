# Testing Leptos as a static site generator (SSG)

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website.

`cargo leptos watch` generates a `dist` directory every time the code changes. The directory can be served as a static website.
