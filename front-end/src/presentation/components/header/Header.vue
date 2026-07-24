<script setup lang="ts">
    import Button from '../button/Button.vue';
    import { RouterLink } from 'vue-router';
</script>

<template>
    <div class="header_div">
        <div class="header_content">
            <div class="header_logo">
                 <nav>
                    <RouterLink class="router_link_styled" to="/">
                        <div class="router_link_div">
                            <img src="/favicon.ico" width="40" height="40" />
                            <span>Prateleira</span>
                        </div>
                    </RouterLink>
                </nav>

                <input type="text" placeholder="Pesquisar" />
            </div>

            <div class="header_search">

                <select id="explorar">
                    <option value="" disabled selected hidden v-on:click="">Explorar</option>
                    <option value="livros" v-on:click="">Livros</option>
                    <option value="autores" v-on:click="">Autores</option>
                    <option value="editoras" v-on:click="">Editoras</option>
                    <option value="generos" v-on:click="">Gêneros</option>
                </select>
                <Button msg="Login" />
            </div>
        </div>
    </div>
</template>

<style scoped>
    .header_div {
        position: fixed;
        display: flex;
        top: 0;
        left: 0;
        width: 100%;
        height: 50px;
        background-color: #4F83D1;
        box-shadow: 5px 5px 5px #c4c4c4;
    }

    .header_content {
        display: flex;
        padding-left: 1rem;
        padding-right: 1rem;
        justify-content: space-between;
        align-items: center;
        margin: 0;
        width: 100%;
    }

    .header_logo {
        width: 80%;
        gap: 5rem;
        justify-content: flex-start;
        align-items: center;
        display: flex;
    }

    .header_search {
        padding-left: 0;
        gap: 1rem;
        justify-content: flex-end;
        align-items: center;
        display: flex;
    }

    input {
        border-radius: 5px;
        width: 500px;
        height: 40px;
        background-color: #ebebeb;
    }

    .router_link_styled:hover {
        background: none !important;
        background-image: none !important;
        text-decoration: none !important;
        box-shadow: none !important;
    }

    .router_link_div {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .router_link_div span {
        font-size: 24px;
    }

    select {
        border-radius: 5px;
        height: 30px;
        width: 100px;
        background-color: #ebebeb;
        color: black;
        font-family: Georgia, 'Times New Roman', Times, serif;
    }
</style>