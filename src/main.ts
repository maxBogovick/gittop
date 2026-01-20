import { createApp } from 'vue';
import { createPinia } from 'pinia';
import './style.css';
import App from './App.vue';
import router from './router';

// Apply theme immediately to prevent flash
function initTheme() {
  const savedTheme = localStorage.getItem('theme-mode');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

  const isDark = savedTheme === 'dark' || (savedTheme === 'system' && prefersDark) || (!savedTheme && prefersDark);

  if (isDark) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}

initTheme();

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount('#app');