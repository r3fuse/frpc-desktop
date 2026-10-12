<script setup lang="ts">
import { watch, ref } from "vue";
import { useStatusStore } from "../../store/statusStore";

const statusStore = useStatusStore();

const status = ref("noWork");

watch(
    () => statusStore.frpcStatus,
    () => {
        switch (statusStore.frpcStatus) {
            case 0:
                status.value = "noWork";
                break;
            case 1:
                status.value = "success";
                break;
            case 2:
                status.value = "warn";
                break;
            case 3:
                status.value = "error";
                break;
        }
    },
);
</script>

<template>
    <div class="frpcStatus">
        <div class="status">
            <div class="sign" :class="status"></div>
            <p>frpc</p>
            <p>{{ statusStore.frpcStatus == 0 ? "未运行" : "运行中" }}</p>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use "../../assets/css/main.scss" as *;

.frpcStatus {
    display: flex;
    margin: 1rem;
    color: #333333d1;
    border-top: 1px solid #9999992c;
    height: 4rem;

    .status {
        display: flex;
        align-items: center;
        height: fit-content;
        margin-top: 1.4rem;
        .sign {
            width: 10px;
            height: 10px;
            background-color: rgb(27, 196, 27);
            border-radius: 10px;
            margin-right: 0.6rem;
        }
        .noWork {
            background-color: #ccc;
        }
        .success {
            background-color: $success;
        }
        .warn {
            background-color: $warn;
        }
        .error {
            background-color: $error;
        }
        p {
            margin: 0;
            margin-right: 0.4rem;
        }
    }
}
</style>
