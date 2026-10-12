import { createVNode, render } from "vue";
import Notification from "../components/notification/index.vue";

type MessageType = "Success" | "Warning" | "Error";

interface MsgType {
    msg_type: MessageType;
    title: string;
    message: string;
    lazyClose?: number;
}

interface notificationInstance {
    result: Promise<boolean>;
    destroy: () => void;
}

export function notification(options: MsgType): notificationInstance {
    const container = document.createElement("div");
    const myDialog = document.querySelector(".myDialog");
    if (myDialog) {
        myDialog.appendChild(container);
    } else {
        document.body.appendChild(container);
    }
    let resolveResult!: (value: boolean) => void;
    let destroyed = false;

    const result = new Promise<boolean>((resolve) => {
        resolveResult = resolve;
    });
    const vnode = createVNode(Notification, {
        ...options,
        onConfirm() {
            resolveResult(true);
            destroy();
        },
        onCancel() {
            resolveResult(false);
            destroy();
        },
    });

    render(vnode, container);

    function destroy() {
        if (destroyed) return;
        destroyed = true;
        render(null, container);
        container.remove();
    }

    return { result, destroy };
}
