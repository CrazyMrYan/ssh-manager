import { writable } from 'svelte/store';

function createThemeStore() {
  const { subscribe, set } = writable<'light' | 'dark' | 'system'>('system');

  function setTheme(theme: 'light' | 'dark' | 'system') {
    set(theme);
    if (theme === 'system') {
      const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      document.documentElement.classList.toggle('dark', isDark);
    } else {
      document.documentElement.classList.toggle('dark', theme === 'dark');
    }
    localStorage.setItem('theme', theme);
  }

  // 初始化主题
  const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | 'system' | null;
  if (savedTheme) {
    setTheme(savedTheme);
  }

  return {
    subscribe,
    setTheme
  };
}

export const themeStore = createThemeStore(); 