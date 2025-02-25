import i18next from 'i18next';
import { derived, writable } from 'svelte/store';

export const locale = writable('zh');

const translations = {
  zh: {
    sshKeys: 'SSH 密钥管理',
    gitConfig: 'Git 全局配置',
    settingsTab: '设置',
    noKeys: '暂无 SSH 密钥，点击上方按钮生成新密钥',
    keyName: '密钥名称',
    keyEmail: '电子邮箱',
    keyComment: '备注信息',
    optional: '可选',
    fingerprint: '密钥指纹',
    lastUsed: '最后使用时间',
    userName: '用户名',
    userEmail: '邮箱',
    copy: '复制',
    delete: '删除',
    cancel: '取消',
    generate: '生成',
    save: '保存',
    generateKey: '生成新密钥',
    generateED25519: '生成 ED25519 密钥',
    generateRSA: '生成 RSA 密钥',
    confirmDelete: '确认删除',
    status: {
      generating: '正在生成密钥...',
      generated: '已生成 {{type}} 密钥：{{name}}',
      copying: '正在复制公钥...',
      copied: '已复制到剪贴板',
      deleted: '已删除',
      saved: '已保存',
      loadFailed: '加载失败',
      generateFailed: '生成失败',
      copyFailed: '复制失败',
      deleteFailed: '删除失败',
      saveFailed: '保存失败'
    },
    confirm: '确认',
    keyType: '密钥类型',
    publicKey: '公钥',
    viewDetails: '查看详情',
    keyDetails: '密钥详情',
    confirmDeleteMessage: '确定要删除密钥 {{name}} 吗？此操作无法撤销。',
    configKey: '配置项',
    configValue: '值',
    newConfigKey: '新配置项',
    newConfigValue: '新值',
    add: '添加',
    searchConfig: '搜索配置项...',
    showing: '显示',
    of: '共',
    details: '详情',
    searchKey: '搜索密钥...',
    items: '项',
    confirmDeleteConfig: '确定要删除配置项 {{key}} 吗？此操作无法撤销。',
    settings: {
      appearance: '外观',
      theme: '主题',
      lightTheme: '亮色主题',
      darkTheme: '暗色主题',
      systemTheme: '跟随系统',
      lightThemeDescription: '使用亮色主题',
      darkThemeDescription: '使用暗色主题',
      systemThemeDescription: '根据系统设置自动切换主题',
      language: '语言',
      about: '关于',
      version: '版本',
      platform: '平台'
    }
  },
  en: {
    sshKeys: 'SSH Keys',
    gitConfig: 'Git Config',
    settingsTab: 'Settings',
    noKeys: 'No SSH keys found',
    keyName: 'Key Name',
    keyEmail: 'Email Address',
    keyComment: 'Comment',
    optional: 'optional',
    fingerprint: 'Fingerprint',
    lastUsed: 'Last Used',
    copy: 'Copy',
    delete: 'Delete',
    cancel: 'Cancel',
    generate: 'Generate',
    generateKey: 'Generate Key',
    generateED25519: 'Generate ED25519 Key',
    generateRSA: 'Generate RSA Key',
    confirmDelete: 'Confirm Delete',
    status: {
      generating: 'Generating key...',
      generated: 'Generated {{type}} key: {{name}}',
      copying: 'Copying public key...',
      copied: 'Public key copied to clipboard',
      deleted: 'Key deleted successfully',
      saved: 'Configuration saved',
      loadFailed: 'Failed to load',
      generateFailed: 'Failed to generate',
      copyFailed: 'Failed to copy',
      deleteFailed: 'Failed to delete',
      saveFailed: 'Failed to save'
    },
    confirm: 'Confirm',
    keyType: 'Key Type',
    publicKey: 'Public Key',
    viewDetails: 'View Details',
    keyDetails: 'Key Details',
    confirmDeleteMessage: 'Are you sure you want to delete key {{name}}? This action cannot be undone.',
    configKey: 'Config Key',
    configValue: 'Value',
    newConfigKey: 'New Config Key',
    newConfigValue: 'New Value',
    add: 'Add',
    searchConfig: 'Search configs...',
    showing: 'Showing',
    of: 'of',
    details: 'Details',
    searchKey: 'Search keys...',
    items: 'items',
    confirmDeleteConfig: 'Are you sure you want to delete config {{key}}? This action cannot be undone.',
    settings: {
      appearance: 'Appearance',
      theme: 'Theme',
      lightTheme: 'Light Theme',
      darkTheme: 'Dark Theme',
      systemTheme: 'System Theme',
      lightThemeDescription: 'Use light theme',
      darkThemeDescription: 'Use dark theme',
      systemThemeDescription: 'Automatically switch theme based on system settings',
      language: 'Language',
      about: 'About',
      version: 'Version',
      platform: 'Platform'
    }
  }
};

i18next.init({
  lng: 'zh',
  resources: {
    zh: { translation: translations.zh },
    en: { translation: translations.en }
  }
});

locale.subscribe(lang => {
  i18next.changeLanguage(lang);
});

export const t = derived(locale, () => (key: string, params?: Record<string, string>) => {
  return i18next.t(key, params);
}); 