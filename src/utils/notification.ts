import { createVNode,render } from "vue";
import Notification from "../components/notification/index.vue"

type MessageType = "Success" | "Warning" | "Error"

interface MsgType {
    msg_type:MessageType,
    title:string,
    message:string
}

export function notification(options:MsgType){
    return new Promise((resolve)=>{
        const container = document.createElement("div");
        document.body.appendChild(container);
        const vnode = createVNode(Notification,{
            ...options,
            onConfirm(){
                resolve(true);
                destroy()
            },
            onCancel(){
                resolve(false);
                destroy();
            }
        })

        render(vnode,container);

        function destroy(){
            render(null,container);
            container.remove();
        }
        
    })

    
}
