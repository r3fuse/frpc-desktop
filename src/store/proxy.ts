import { defineStore } from "pinia";
import { ref } from "vue";


export const useProxyStore = defineStore("proxy",()=>{
    const proxy = ref<Proxies|undefined>();

    function getProxy(){
        return proxy.value;
    }

    function setProxy(data:Proxies){
        proxy.value = data;
    }

    function clear(){
        proxy.value = undefined;
    }

    return {setProxy,getProxy,clear}

})