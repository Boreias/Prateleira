<script setup lang="ts">
    defineProps<{
        msg: string,
        action?: Function
    }>()
</script>

<template>
    <button type="button">{{ msg }}</button>
</template>

<style>
    button {
        border-radius: 5px;
        height: 30px;
        width: 60px;
        background-color: #4facd1;
        color: black;
        font-family: Georgia, 'Times New Roman', Times, serif;
    }
</style>