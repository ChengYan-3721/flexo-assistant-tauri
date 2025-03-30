<script setup lang="ts">
const color = defineModel("color", {
    required: true,
    type: String
});
const content = defineModel("content", {
    required: true,
    type: String
});
const finished = defineModel("finished", {
    required: true,
    type: Boolean
});
defineEmits(["editTodo", "changeState", "removeTodo"]);
</script>

<template>
    <div role="alert" :class="color">
        <pre class="grow self-start break-all text-wrap">{{ content }}</pre>
        <div class="flex-none self-start">
            <button v-if="!finished" class="btn btn-square btn-xs" @click="$emit('editTodo')">
                <svg class="icon" viewBox="-150 -150 1300 1300" xmlns="http://www.w3.org/2000/svg">
                    <g stroke-linejoin="round" stroke-linecap="round" stroke-width="40" fill="none"
                       stroke="currentColor">
                        <path
                            d="M806 911H218c-57.9 0-105-47.1-105-105V218c0-57.9 47.1-105 105-105h349.4c11.6 0 21 9.4 21 21s-9.4 21-21 21H218c-34.7 0-63 28.3-63 63v588c0 34.7 28.3 63 63 63h588c34.7 0 63-28.3 63-63V457c0-11.6 9.4-21 21-21s21 9.4 21 21v349c0 57.9-47.1 105-105 105z"></path>
                        <path
                            d="M896.6 129c8.2 8.2 8.2 21.5 0 29.7L525.3 529.9c-8.2 8.2-21.5 8.2-29.7 0s-8.2-21.5 0-29.7L866.9 129c8.1-8.2 21.5-8.2 29.7 0z"></path>
                    </g>
                </svg>
            </button>
            <button class="btn btn-square btn-xs" @click="$emit('changeState')">
                <svg v-if="!finished" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 21 21"
                     stroke-width="2.5"
                     stroke="#00D390" class="size-[1.2em]">
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="M5 13l4 4L19 7"/>
                </svg>
                <svg v-else class="size-[1.2em]" viewBox="0 0 900 900" xmlns="http://www.w3.org/2000/svg">
                    <g stroke-linejoin="round" stroke-linecap="round" stroke-width="20" fill="none"
                       stroke="currentColor">
                        <path
                            d="M531.19158 198.31299c198.31299 19.19158 396.62598 147.135444 396.62598 403.023174 0 179.12141-166.327024 396.62598-371.037207 422.214753-38.383159 6.397193-51.177546-57.574739-12.794387-63.971932 115.149478-19.19158 198.31299-115.149478 198.31299-223.901763 0-134.341058-51.177546-191.915797-147.135444-204.710184-19.19158-6.397193-38.383159-6.397193-70.369126-6.397193v134.341058c0 70.369126-83.163512 102.355092-127.943864 51.177546L115.37402 422.214753C83.388053 390.228787 83.388053 345.448434 115.37402 313.462468L396.850522 25.588773c44.780353-51.177546 127.943865-12.794386 127.943864 51.177546v121.546671z m332.654048 403.023174c0-127.943865-57.574739-217.50457-153.532638-275.079309-70.369126-38.383159-153.532638-63.971932-211.107376-63.971933-19.19158 0-31.985966-12.794386-31.985967-31.985966V76.766319c0-12.794386-12.794386-19.19158-19.191579-6.397193L166.551566 358.242821c-6.397193 6.397193-6.397193 12.794386 0 19.19158L448.028068 665.308096c6.397193 6.397193 19.19158 0 19.191579-6.397193V492.583879c0-19.19158 12.794386-31.985966 31.985967-31.985966H512c44.780353 0 70.369126 0 102.355092 6.397193 127.943865 19.19158 198.31299 102.355092 198.31299 268.682116v38.383159c31.985966-57.574739 51.177546-115.149478 51.177546-172.724217z"
                            fill="#777777">
                        </path>
                    </g>
                </svg>
            </button>
            <button class="btn btn-square btn-xs text-red-600" @click="$emit('removeTodo')">✘</button>
        </div>
    </div>
</template>