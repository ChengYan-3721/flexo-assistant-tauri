<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {emit, once} from '@tauri-apps/api/event';
import {ref} from "vue";
import DistortionRate from "./components/DistortionRate.vue";
import Todo from "./components/Todo.vue";
import Typography from "./components/Typography.vue";

const darkMode = ref(false);
const precision = ref(4);
const userKMap = ref("");
const appTab = ref("distortionRate");
const distortionRateTab = ref("通用");
const todoTab = ref("全部");

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
invoke("get_config").then((config: any) => {
    darkMode.value = config.theme;
    precision.value = config.precision;
    userKMap.value = config.k_map;
    appTab.value = config.app_tab;
    distortionRateTab.value = config.distortion_rate_tab;
    todoTab.value = config.todo_tab;
})
const handleContextMenu = (e: Event) => {
    e.preventDefault();  // 阻止默认右键菜单
    // 可在此处添加自定义右键逻辑（如弹出自定义菜单）
}

once("save-config", () => {
    invoke("update_config", {
        config: {
            id: 1,
            theme: darkMode.value,
            precision: precision.value,
            k_map: userKMap.value,
            app_tab: appTab.value,
            distortion_rate_tab: distortionRateTab.value,
            todo_tab: todoTab.value,
        }
    }).finally(() => {
        emit("config-saved");
    })
})
</script>

<template>
    <div role="tablist" class="tabs tabs-border tabs-sm" @contextmenu.prevent="handleContextMenu">
        <input type="radio" name="app" class="tab" aria-label="变形率计算" value="distortionRate" v-model="appTab"/>
        <div class="tab-content">
            <DistortionRate v-model:precision="precision" v-model:userKMap="userKMap"
                            v-model:distortionRateTab="distortionRateTab"/>
        </div>
        <input type="radio" name="app" class="tab" aria-label="拼版计算" value="typography" v-model="appTab"/>
        <div class="tab-content p-3">
            <Typography v-model:precision="precision"/>
        </div>
        <input type="radio" name="app" class="tab" aria-label="待办事项" value="todo" v-model="appTab"/>
        <div class="tab-content">
            <Todo v-model:todoTab="todoTab"/>
        </div>
    </div>
    <div class="w-31 fixed top-2 right-11 flex flex-row items-center">
        <label class="text-xs w-11">小数位</label>
        <select class="select select-xs w-15" v-model.number="precision">
            <option label="3" :value="3"/>
            <option label="4" :value="4" selected/>
            <option label="5" :value="5"/>
            <option label="6" :value="6"/>
        </select>
    </div>
    <label class="toggle text-base-content fixed top-2 right-3">
        <input type="checkbox" value="dark" class="theme-controller" v-model="darkMode">
        <svg aria-label="sun" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                <circle cx="12" cy="12" r="4"></circle>
                <path d="M12 2v2"></path>
                <path d="M12 20v2"></path>
                <path d="m4.93 4.93 1.41 1.41"></path>
                <path d="m17.66 17.66 1.41 1.41"></path>
                <path d="M2 12h2"></path>
                <path d="M20 12h2"></path>
                <path d="m6.34 17.66-1.41 1.41"></path>
                <path d="m19.07 4.93-1.41 1.41"></path>
            </g>
        </svg>
        <svg aria-label="moon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
            </g>
        </svg>
    </label>
</template>