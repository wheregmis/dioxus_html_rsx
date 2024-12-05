check:
    cargo check

run:
    dx serve --platform fullstack --port 3000

css:
    npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch