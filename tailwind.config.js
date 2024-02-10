/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        colors: {
            surface: "#ffcd75",
            text: "#f4f4f4",
            accent: "#1a1c2c"
        }
    },
    plugins: [],
}

