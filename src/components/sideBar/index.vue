<script setup lang="ts">
import { onMounted } from "vue";
import FrpcStatus from "../frpcStatus/index.vue";

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
        <FrpcStatus />
    </div>
</template>

<style lang="scss" scoped>
@use "../../assets/css/main.scss" as *;
a {
    text-decoration: none;
    color: black;
}
.siderBar {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 1rem 0.6rem;
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    background-color: rgb(224, 229, 241);
    border-right: 1px solid #ccc;
    box-shadow: 1px #ccc;
    -webkit-user-select: none;
    user-select: none;

    ul {
        width: 100%;
        color: #1f2937;
        list-style-type: none;
        padding: 0;
        margin: 0;
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        font-size: 18px;
        font-weight: bold;

        li {
            $show-time: 300ms;

            height: 3rem;
            width: 100%;
            margin: auto;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            position: relative;
            transition: all $show-time ease-in-out;

            &::before {
                content: "";
                width: 0;
                height: 100%;
                position: absolute;
                left: 1px;
                background-color: $blue;
                transition: all $show-time linear;
            }

            &.active {
                background-color: $skyBlue;
                overflow: hidden;

                &::before {
                    width: 6px;
                }
            }
        }

        a {
            .active {
                color: $blue;
            }
        }
    }
}
</style>
