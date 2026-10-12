import { defineStore } from "pinia";
import { ref } from "vue";
import { WorkStatus } from "../utils/statue";

export const useStatusStore = defineStore("status", () => {
    const logs = ref<string[]>([]);
    const frpcStatus = ref<WorkStatus>(WorkStatus.noWork);
    const isWork = ref<boolean>(false);
    const keepBottom = ref<boolean>(false);
    const startTime = ref<number>(0);

    function getLogs(): string[] {
        return logs.value;
    }

    function addLogs(data: string) {
        if (!data) {
            return;
        }
        if (logs.value.length > 100) {
            logs.value.shift();
        }
        logs.value.push(data);
        if (data.includes("[W]")) {
            changeFRPStatus(WorkStatus.warn);
        }
        if (data.includes("[E]") || data.includes("server failed")) {
            changeFRPStatus(WorkStatus.error);
        }
    }

    function clearLogs() {
        logs.value = [];
    }

    function getProcessStatue(): WorkStatus {
        return frpcStatus.value;
    }

    function changeFRPStatus(newStatus: WorkStatus) {
        if (newStatus < frpcStatus.value) return;
        frpcStatus.value = newStatus;
    }

    function changeWork() {
        isWork.value = !isWork.value;
        if (isWork.value) {
            frpcStatus.value = WorkStatus.success;
        } else {
            frpcStatus.value = WorkStatus.noWork;
        }
    }

    function getIsWork(): boolean {
        return isWork.value;
    }

    return {
        logs,
        startTime,
        getLogs,
        addLogs,
        clearLogs,
        getProcessStatue,
        changeWork,
        getIsWork,
        frpcStatus,
        isWork,
        keepBottom,
    };
});
