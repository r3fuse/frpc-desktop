import { defineStore } from "pinia";
import { ref } from "vue";
import { Config, Proxies } from "../utils/type";

export const useConfigStore = defineStore("config", () => {
    const config = ref<Config>({});

    function get_config(): Config {
        return config.value;
    }

    function set_config(data: Config) {
        config.value = data;
    }

    function get_proxy(): Proxies[] | undefined {
        return config.value.proxies;
    }

    return { get_config, set_config, get_proxy };
});
