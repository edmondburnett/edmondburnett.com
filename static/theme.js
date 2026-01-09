function toggleTheme() {
    document.documentElement.classList.toggle('dark')
    localStorage.theme = document.documentElement.classList.contains('dark')
        ? 'dark'
        : 'light'
    updateIcon()
}

// Listen for system theme changes
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.theme) {
        document.documentElement.classList.toggle('dark', e.matches)
    }
})

function updateIcon() {
    const isDark = document.documentElement.classList.contains('dark')
    document.querySelector('.light-icon').style.display = isDark
        ? 'none'
        : 'inline'
    document.querySelector('.dark-icon').style.display = isDark
        ? 'inline'
        : 'none'
}

// Set initial icon state on page load
updateIcon()
