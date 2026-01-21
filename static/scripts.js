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

document.querySelectorAll('pre').forEach((pre) => {
    const code = pre.querySelector('code')
    if (!code) return

    // Create copy button
    const button = document.createElement('button')
    button.className = 'code-copy-btn'
    button.innerHTML = '<img src="/static/copy.svg" alt="Copy code"/>'
    button.title = 'Copy to clipboard'

    // Get code content
    button.addEventListener('click', async () => {
        const text = code.textContent
        const img = button.querySelector('img')

        try {
            await navigator.clipboard.writeText(text)
            img.src = '/static/checkmark.svg'
            img.alt = 'Copied!'
            button.classList.add('copied')
            setTimeout(() => {
                img.src = '/static/copy.svg'
                img.alt = 'Copy code'
                button.classList.remove('copied')
            }, 1000)
        } catch (err) {
            console.error('Failed to copy:', err)
        }
    })

    code.style.position = 'relative'
    code.appendChild(button)
})
