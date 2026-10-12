<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
type MessageType = "Success" | "Warning" | "Error";

interface MsgType {
    msg_type: MessageType;
    title: string;
    message: string;
    lazyClose?: number;
}
const props = defineProps<MsgType>();
const emit = defineEmits<{ onCancel: []; confirm: [] }>();

let timer: ReturnType<typeof setTimeout>;

onMounted(() => {
    if (!props.lazyClose) {
        timer = setTimeout(() => {
            emit("confirm");
        }, 3000);
    } else {
        timer = setTimeout(() => {
            emit("confirm");
        }, props.lazyClose);
    }
});

onUnmounted(() => {
    clearTimeout(timer);
});
</script>

<template>
    <div class="notification" :class="props.msg_type.toLocaleLowerCase()">
        {{ props.message }}
    </div>
</template>

<style>
.notification {
    z-index: 99;
    position: absolute;
    top: 50%;
    left: calc(50%);
    transform: translateX(-50%);
    color: #ffffff;
    font-weight: bold;
    font-size: small;
    padding: 0.4rem 1rem;
    border-radius: 16px;
    background-color: rgba(196, 196, 196, 0.4);
    opacity: 0;
    animation: lineShow 3s;
}
.notification.success {
    background-color: rgba(79, 212, 102, 0.8);
}
.notification.warning {
    background-color: rgba(205, 212, 79, 0.8);
}
.notification.error {
    background-color: rgba(219, 34, 52, 0.8);
}

@keyframes lineShow {
    0% {
        top: 60%;
        opacity: 0;
    }
    80% {
        top: 50%;
        opacity: 1;
    }
    100% {
        top: 50%;
        opacity: 0;
    }
}
</style>
