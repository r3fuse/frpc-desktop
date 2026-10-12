import { defineStore } from "pinia";
import { ref } from "vue";
import { Config, Proxies } from "../utils/type";

export const useConfigStore = defineStore("config", () => {
    const config = ref<Config>({});

    function get_config(): Readonly<Config> {
        return config.value;
    }

    function set_config(data: Config) {
        config.value = data;
    }

    function get_proxy(): Proxies[] | undefined {
        return config.value.proxies ?? [];
    }

    return { config, set_config, get_config, get_proxy };
});
