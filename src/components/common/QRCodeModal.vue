<template>
  <!--
    QRCodeModal.vue - 二维码弹窗组件
    用于显示网盘下载二维码，支持夸克网盘等
  -->
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue" class="qr-modal-overlay" @click="handleOverlayClick">
        <div class="qr-modal-container" @click.stop>
          <!-- 弹窗头部 -->
          <div class="qr-modal-header">
            <h3 class="qr-modal-title">{{ title }}</h3>
            <button class="qr-close-btn" @click="handleClose">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>

          <!-- 弹窗内容 -->
          <div class="qr-modal-body">
            <!-- 二维码图片 -->
            <div class="qr-image-wrapper">
              <img
                v-if="qrImageUrl"
                :src="qrImageUrl"
                :alt="title"
                class="qr-image"
                @error="handleImageError"
              />
              <div v-else-if="imageError" class="qr-error">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <line x1="12" y1="8" x2="12" y2="12"/>
                  <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
                <p>二维码加载失败</p>
              </div>
              <div v-else class="qr-loading">
                <div class="qr-spinner"></div>
                <p>加载中...</p>
              </div>
            </div>

            <!-- 提示文字 -->
            <p class="qr-hint">{{ hint }}</p>
          </div>

          <!-- 弹窗底部 -->
          <div class="qr-modal-footer">
            <button class="qr-close-action-btn" @click="handleClose">
              关闭
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
/**
 * QRCodeModal.vue - 二维码弹窗组件
 * 显示网盘分享二维码，用户可使用对应APP扫码下载
 */

import { ref, watch } from 'vue'

// ============================================
// Props 定义
// ============================================

interface Props {
  /** 控制弹窗显示/隐藏 */
  modelValue: boolean
  /** 弹窗标题 */
  title?: string
  /** 二维码图片URL */
  qrImageUrl?: string
  /** 提示文字 */
  hint?: string
  /** 点击遮罩层是否关闭 */
  closeOnOverlay?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '扫码下载',
  hint: '请使用夸克APP扫码下载',
  closeOnOverlay: true
})

// ============================================
// Emits 定义
// ============================================

const emit = defineEmits<{
  /** 更新显示状态 */
  (e: 'update:modelValue', value: boolean): void
  /** 关闭弹窗 */
  (e: 'close'): void
}>()

// ============================================
// 状态
// ============================================

const imageError = ref(false)

// ============================================
// 方法
// ============================================

/**
 * 处理遮罩层点击
 */
function handleOverlayClick() {
  if (props.closeOnOverlay) {
    handleClose()
  }
}

/**
 * 处理关闭操作
 */
function handleClose() {
  emit('update:modelValue', false)
  emit('close')
}

/**
 * 处理图片加载失败
 */
function handleImageError() {
  imageError.value = true
}

// ============================================
// 监听
// ============================================

/**
 * 监听弹窗显示状态，重置错误状态
 */
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    imageError.value = false
  }
})
</script>

<style scoped>
/* 弹窗遮罩层 */
.qr-modal-overlay {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.75);
  z-index: 2000;
  padding: 24px;
  backdrop-filter: blur(4px);
}

/* 弹窗容器 */
.qr-modal-container {
  width: 100%;
  max-width: 400px;
  background: var(--steam-bg-secondary);
  border-radius: 16px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* 弹窗头部 */
.qr-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--steam-border);
}

.qr-modal-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--steam-text-primary);
  margin: 0;
}

.qr-close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--steam-text-secondary);
  border-radius: 8px;
  transition: all 0.15s ease-out;
  background: transparent;
  border: none;
  cursor: pointer;
}

.qr-close-btn:hover {
  color: var(--steam-text-primary);
  background: var(--steam-accent-hover);
}

.qr-close-btn svg {
  width: 20px;
  height: 20px;
}

/* 弹窗内容区 */
.qr-modal-body {
  padding: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

/* 二维码图片包装器 */
.qr-image-wrapper {
  max-width: 320px;
  max-height: 320px;
  min-width: 200px;
  min-height: 200px;
  background: #ffffff;
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* 二维码图片 */
.qr-image {
  max-width: 100%;
  max-height: 280px;
  width: auto;
  height: auto;
  object-fit: contain;
}

/* 加载状态 */
.qr-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--steam-text-secondary);
}

.qr-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--steam-border);
  border-top-color: var(--steam-accent);
  border-radius: 50%;
  animation: qr-spin 0.8s linear infinite;
}

@keyframes qr-spin {
  to {
    transform: rotate(360deg);
  }
}

.qr-loading p {
  font-size: 14px;
  margin: 0;
}

/* 错误状态 */
.qr-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--steam-text-error, #ff6b6b);
}

.qr-error svg {
  width: 48px;
  height: 48px;
}

.qr-error p {
  font-size: 14px;
  margin: 0;
}

/* 提示文字 */
.qr-hint {
  font-size: 14px;
  color: var(--steam-text-secondary);
  text-align: center;
  margin: 0;
  line-height: 1.5;
}

/* 弹窗底部 */
.qr-modal-footer {
  display: flex;
  justify-content: center;
  padding: 16px 20px;
  border-top: 1px solid var(--steam-border);
}

.qr-close-action-btn {
  padding: 10px 32px;
  border-radius: 8px;
  border: none;
  background: var(--steam-accent);
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.qr-close-action-btn:hover {
  background: var(--steam-accent-hover);
  transform: translateY(-1px);
}

/* 弹窗动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease-out;
}

.modal-enter-active .qr-modal-container,
.modal-leave-active .qr-modal-container {
  transition: transform 0.2s ease-out, opacity 0.2s ease-out;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .qr-modal-container,
.modal-leave-to .qr-modal-container {
  opacity: 0;
  transform: scale(0.95);
}
</style>
