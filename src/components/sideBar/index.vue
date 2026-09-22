<script setup lang="ts">
import { onMounted } from "vue";

const lis = [
    { name: "状态", path: "/" },
    { name: "配置", path: "/server" },
    { name: "Log", path: "/terminal" },
    { name: "About", path: "/about" },
];
let titles: NodeListOf<Element>;

function addClass(e: MouseEvent) {
    if (titles == undefined) return;
    removeClassName(titles);
    const li = e.currentTarget as HTMLLIElement;
    li.classList.add("active");
}

function removeClassName(nodes: NodeListOf<Element>) {
    nodes.forEach((item) => {
        item.classList.remove("active");
    });
}

onMounted(() => {
    titles = document.querySelectorAll(".siderBar>ul a li");
});
</script>

<template>
    <div class="siderBar">
        <ul>
            <RouterLink v-for="(li, index) in lis" :to="li.path">
                <li
                    class="title"
                    :class="index == 0 ? 'active' : ''"
                    @click="addClass"
                >
                    {{ li.name }}
                </li>
            </RouterLink>
        </ul>
    </div>
</template>

<style lang="scss" scoped>
@use "../../assets/css/main.scss" as *;
a {
    text-decoration: none;
    color: white;
}
.siderBar {
    display: flex;
    padding: 1rem 0.6rem;
    height: 100%;
    box-sizing: border-box;

    ul {
        color: white;
        list-style-type: none;
        padding: 0;
        margin: 0;
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        font-size: 18px;
        font-weight: bold;

        li {
            width: 10rem;
            height: 2.6rem;
            margin: auto;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            border-radius: 0.4rem;

            &.active {
                background-color: $blue;
            }
        }

        a {
            .active {
                color: $darkblue;
            }
        }
    }
}
</style>
