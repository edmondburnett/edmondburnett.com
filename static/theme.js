function toggleTheme() {
    document.documentElement.classList.toggle('dark')
    localStorage.theme = document.documentElement.classList.contains('dark')
        ? 'dark'
        : 'light'
}

// Listen for system theme changes
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.theme) {
        document.documentElement.classList.toggle('dark', e.matches)
    }
})
