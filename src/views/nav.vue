<script setup lang="ts">

</script>

<template>
    <div class="nav">
        <RouterLink to="/">
            <span>dashboard</span>
        </RouterLink>
        <RouterLink to="/terminal">
            <span>Settings</span>
        </RouterLink>
    </div>
</template>


<style scoped>
a{
    text-decoration: none;
}
.nav{
    display: flex;
    gap: 1rem;
}
.nav span{
    color: #eee;
    cursor: pointer;
    border-radius: 10px;
    background-color: rgb(107, 117, 161);
    padding: 0 0.6rem;
}
</style>