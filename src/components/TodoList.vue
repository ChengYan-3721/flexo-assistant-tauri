<script setup lang="ts">
import {ref} from "vue";

const addTodoDialog = ref();
</script>

<template>
    <div role="tablist" class="tabs tabs-border tabs-sm">
        <input type="radio" name="my_tabs_2" class="tab" aria-label="全部" checked />
        <input type="radio" name="my_tabs_2" class="tab" aria-label="待办" />
        <input type="radio" name="my_tabs_2" class="tab" aria-label="已完成" />
    </div>
    <button class="btn btn-soft btn-primary btn-xs fixed top-10 right-3" @click="addTodoDialog.showModal()">添加待办</button>
    <dialog ref="addTodoDialog" class="modal">
        <div class="modal-box flex flex-col gap-4">
            <h6 class="text-lg font-bold">请填写待办!</h6>
            <textarea class="textarea" placeholder="Bio"></textarea>
            <button class="btn btn-active btn-primary btn-sm w-20">添加</button>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button>close</button>
        </form>
    </dialog>
</template>