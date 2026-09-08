<script setup lang="ts">

</script>

<template>
    <div class="bar">
        <div class="mian">
            <ul>
                <li>
                    <RouterLink to="/">
                                首页
                    </RouterLink>
                </li>
                <li>
                    <RouterLink to="server">
                        配置
                    </RouterLink>
                </li>
                <li>
                    <RouterLink to="about">
                        关于
                    </RouterLink>
                </li>
            </ul>
        </div>
    </div>
</template>


<style scoped>
a{
    text-decoration: none;
    color: #ccc;
}
.bar{
    width: fit-content;
    background-color: var(--lightBlue);
    margin: 1rem 1rem 0 2rem;
    /* padding-top: 2rem; */
    font-weight: bold;
    font-size: medium;
    box-sizing: border-box;
    position: sticky;
    width: 100%;
}
.bar ul{
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
}
.bar ul li{
    margin: 0;
    margin-right: 1.6rem;
    display: flex;
    align-content: center;
    justify-content: center;
}
</style>