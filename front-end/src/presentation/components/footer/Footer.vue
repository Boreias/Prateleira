<script setup lang="ts">
    import { RouterLink } from 'vue-router';
</script>

<template>
    <div class="footer_div">
        <nav>
            <RouterLink class="router_link_styled" to="/about">Sobre</RouterLink>
        </nav>
    </div>
</template>

<style scoped>
    .footer_div {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 100px;
        background-color: #4FD1A1;
    }
</style>