<script setup lang="ts"></script>

<template>
    <div class="container">
        <div>
            <p>Nada</p>
        </div>

        <div class="content_container">
            <div>
                <p>Alguma mensagem</p>
            </div>
            <div>
                <h3>Entrar no Prateleira</h3>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .container {
        display: grid;
        grid-template-columns: auto auto;
        gap: 10px;
        height: 100%;
    }

    .content_container {
        height: 100%;
        display: grid;
        grid-template-rows: 1fr 1fr;
        grid-area: 10px;
    }
</style>