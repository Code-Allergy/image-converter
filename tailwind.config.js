/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                primary: '#9aa8d5',   // Custom primary color
                secondary: '#6b7280', // Custom secondary color
            },
        },
    },
    plugins: [],
}