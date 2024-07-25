check:
    cargo check

run:
    dx serve --platform fullstack

css:
    npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch