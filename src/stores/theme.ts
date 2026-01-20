import { defineStore } from 'pinia';
import { ref, watch, computed } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';

export const useThemeStore = defineStore('theme', () => {
  // Theme state
  const themeMode = ref<ThemeMode>(
    (localStorage.getItem('theme-mode') as ThemeMode) || 'system'
  );

  // Sidebar state
  const sidebarCollapsed = ref(
    localStorage.getItem('sidebar-collapsed') === 'true'
  );

  // System preference detection
  const systemPrefersDark = ref(
    window.matchMedia('(prefers-color-scheme: dark)').matches
  );

  // Computed: actual theme (resolved from system if needed)
  const resolvedTheme = computed(() => {
    if (themeMode.value === 'system') {
      return systemPrefersDark.value ? 'dark' : 'light';
    }
    return themeMode.value;
  });

  // Computed: is dark mode active
  const isDark = computed(() => resolvedTheme.value === 'dark');

  // Apply theme to document
  function applyTheme() {
    const html = document.documentElement;
    if (isDark.value) {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
  }

  // Set theme mode
  function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode;
    localStorage.setItem('theme-mode', mode);
    applyTheme();
  }

  // Toggle sidebar
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
    localStorage.setItem('sidebar-collapsed', String(sidebarCollapsed.value));
  }

  // Set sidebar state directly
  function setSidebarCollapsed(collapsed: boolean) {
    sidebarCollapsed.value = collapsed;
    localStorage.setItem('sidebar-collapsed', String(collapsed));
  }

  // Listen for system theme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', (e) => {
    systemPrefersDark.value = e.matches;
    if (themeMode.value === 'system') {
      applyTheme();
    }
  });

  // Watch theme mode changes
  watch(themeMode, () => {
    applyTheme();
  }, { immediate: true });

  return {
    themeMode,
    sidebarCollapsed,
    systemPrefersDark,
    resolvedTheme,
    isDark,
    setThemeMode,
    toggleSidebar,
    setSidebarCollapsed,
    applyTheme
  };
});
