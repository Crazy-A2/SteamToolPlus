<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="show" class="modal-overlay" @click.self="handleClose">
        <div class="modal-container">
          <div class="modal-header">
            <h3 class="modal-title">清单入库功能首次使用配置（以后直接点入库按钮就行，无需这些操作了）</h3>
            <button class="modal-close" @click="handleClose">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>

          <div class="modal-body">
            <div class="setup-steps">
              <div class="step">
                <span class="step-number">1</span>
                <div class="step-content">
                  <span class="step-text">SteamTools （屏幕上的steam图标悬浮球，没有就双击托盘的steamtools.exe）</span>
                  <button class="btn-open" @click="openSteamTools">
                    <svg viewBox="0 0 24 24" fill="currentColor" class="icon">
                      <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
                    </svg>
                    打开
                  </button>
                </div>
              </div>
              <div class="step">
                <span class="step-number">2</span>
                <div class="step-content">
                  <span class="step-text">示例文件夹（第一次使用清单入库请将这个文件夹中的内容拖入_steamtools_的悬浮图标）</span>
                  <button class="btn-open" @click="openExampleFolder">
                    <svg viewBox="0 0 24 24" fill="currentColor" class="icon">
                      <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"/>
                    </svg>
                    打开
                  </button>
                </div>
              </div>
              <div class="step">
                <span class="step-number">3</span>
                <span class="step-text">将全部文件拖到悬浮窗完成初始化（会显示成功编译1个lua文件......）</span>
              </div>
            </div>

            <div class="checkbox-container">
              <label class="checkbox-label">
                <input
                  type="checkbox"
                  v-model="isCompleted"
                  class="checkbox-input"
                />
                <span class="checkbox-text">我已完成以上配置</span>
              </label>
            </div>
          </div>

          <div class="modal-footer">
            <button
              class="btn btn-secondary"
              @click="handleClose"
            >
              关闭
            </button>
            <button
              class="btn btn-primary"
              :disabled="!isCompleted"
              @click="handleConfirm"
            >
              确认
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  close: [];
  confirm: [];
}>();

const isCompleted = ref(false);

const handleClose = () => {
  isCompleted.value = false;
  emit('close');
};

const handleConfirm = () => {
  if (isCompleted.value) {
    emit('confirm');
    isCompleted.value = false;
  }
};

// 打开SteamTools
const openSteamTools = async () => {
  try {
    await invoke('open_steamtools');
  } catch (error) {
    console.error('打开SteamTools失败:', error);
    alert(`打开SteamTools失败: ${error}`);
  }
};

// 打开示例文件夹
const openExampleFolder = async () => {
  try {
    await invoke('open_example_folder');
  } catch (error) {
    console.error('打开示例文件夹失败:', error);
    alert(`打开示例文件夹失败: ${error}`);
  }
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-container {
  background: var(--steam-bg-primary, #1e1e1e);
  border-radius: 12px;
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--steam-border, #333);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--steam-border, #333);
  background: var(--steam-bg-secondary, #252525);
}

.modal-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--steam-text-primary, #fff);
  margin: 0;
  flex: 1;
  padding-right: 16px;
}

.modal-close {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  color: var(--steam-text-secondary, #aaa);
  cursor: pointer;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.modal-close:hover {
  background: var(--steam-bg-hover, #333);
  color: var(--steam-text-primary, #fff);
}

.modal-close svg {
  width: 20px;
  height: 20px;
}

.modal-body {
  padding: 24px;
  background: var(--steam-bg-primary, #1e1e1e);
}

.setup-steps {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 24px;
}

.step {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.step-number {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--steam-accent-blue, #3b82f6);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
  flex-shrink: 0;
}

.step-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.step-text {
  font-size: 14px;
  color: var(--steam-text-primary, #fff);
  line-height: 1.6;
  padding-top: 4px;
}

.btn-open {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--steam-accent-blue, #3b82f6);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  align-self: flex-start;
}

.btn-open:hover {
  background: var(--steam-accent-blue-hover, #2563eb);
}

.btn-open .icon {
  width: 16px;
  height: 16px;
}

.checkbox-container {
  padding: 16px;
  background: var(--steam-bg-secondary, #252525);
  border-radius: 8px;
  border: 1px solid var(--steam-border, #333);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
}

.checkbox-input {
  width: 20px;
  height: 20px;
  accent-color: var(--steam-accent-blue, #3b82f6);
  cursor: pointer;
}

.checkbox-text {
  font-size: 14px;
  color: var(--steam-text-primary, #fff);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--steam-border, #333);
  background: var(--steam-bg-secondary, #252525);
}

.btn {
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: var(--steam-accent-blue, #3b82f6);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--steam-accent-blue-hover, #2563eb);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--steam-bg-tertiary, #333);
  color: var(--steam-text-primary, #fff);
  border: 1px solid var(--steam-border, #444);
}

.btn-secondary:hover {
  background: var(--steam-bg-hover, #444);
}

/* 过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.3s ease;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.95);
}
</style>
