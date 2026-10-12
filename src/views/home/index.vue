<script setup lang="ts">
import ProxiesStatus from "../../components/proxiesStatus/index.vue";
import Teminal from "../../components/terminal/index.vue";
import BaseInfo from "../../components/baseInfo/index.vue";
</script>

<template>
    <div class="home">
        <div class="top">
            <BaseInfo />
        </div>
        <div class="main">
            <ProxiesStatus />
        </div>
        <div class="bottom">
            <Teminal />
        </div>
    </div>
</template>

<style lang="scss" scoped>
.home {
    display: flex;
    flex-direction: column;
    height: 100%;

    .top {
        flex: 1;
        margin: 0.6rem 0.6rem 0 0.6rem;
    }
    .main {
        flex: 2;
        overflow: hidden;
        margin: 0.6rem 0.6rem 0 0.6rem;
    }
    .bottom {
        flex: 2;
        overflow: hidden;
        margin: 0.6rem 0.6rem 0 0.6rem;
    }
}
</style>
