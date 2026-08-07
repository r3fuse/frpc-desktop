<script setup lang="ts">

</script>

<template>
    <div class="server">
        <div class="config">
            <div class="address">
                <label for="ip">服务器地址：</label><input type="text" id="ip">
            </div>
            <div class="port">
                <label for="port">绑定端口：</label><input type="number" id="port">
            </div>
        </div>
    </div>
</template>


<style scoped>
.address{
    color: var(--pureple);
}
</style>