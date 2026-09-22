import { defineStore } from "pinia";
import { ref } from "vue";
import { WorkStatus } from "../utils/statue";

export const useStatusStore = defineStore("status", () => {
    const logs = ref<string[]>([]);
    const frpcStatus = ref<WorkStatus>(WorkStatus.noWork);
    const isWork = ref<boolean>(false);

    function getLogs(): string[] {
        return logs.value;
    }

    function addLogs(data: string) {
        if (logs.value.length > 1000) {
            logs.value.shift();
        }
        logs.value.push(data);
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
        getLogs,
        addLogs,
        clearLogs,
        getProcessStatue,
        changeFRPStatus,
        changeWork,
        getIsWork,
        frpcStatus,
        isWork,
    };
});
