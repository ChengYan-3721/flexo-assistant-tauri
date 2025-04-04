<script setup lang="ts">
const content = defineModel<string>("content", {required: true});
const top = defineModel<boolean>("top", {required: true});
const finished = defineModel<boolean>("finished", {required: true});
const note = defineModel<boolean>("note", {required: true});
defineEmits(["editTodo", "changeTop", "changeState", "removeTodo"]);
</script>

<template>
    <div role="alert" :class="'alert alert-soft flex text-sm p-1 '+(note? 'alert-info' : (finished? 'alert-success' : 'alert-warning'))">
        <pre class="grow self-start break-all text-wrap">{{ content }}</pre>
        <div class="flex-none self-start">
            <button v-if="!finished" class="btn btn-square btn-xs" @click="$emit('editTodo')">
                <svg class="icon" fill="currentColor" viewBox="-150 -150 1300 1300" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M806 911H218c-57.9 0-105-47.1-105-105V218c0-57.9 47.1-105 105-105h349.4c11.6 0 21 9.4 21 21s-9.4 21-21 21H218c-34.7 0-63 28.3-63 63v588c0 34.7 28.3 63 63 63h588c34.7 0 63-28.3 63-63V457c0-11.6 9.4-21 21-21s21 9.4 21 21v349c0 57.9-47.1 105-105 105z"></path>
                    <path
                        d="M896.6 129c8.2 8.2 8.2 21.5 0 29.7L525.3 529.9c-8.2 8.2-21.5 8.2-29.7 0s-8.2-21.5 0-29.7L866.9 129c8.1-8.2 21.5-8.2 29.7 0z"></path>
                </svg>
            </button>
            <button v-if="top" class="btn btn-square btn-xs btn-active text-red-600" @click="$emit('changeTop')">
                <svg class="size-[1.2em]" fill="currentColor" viewBox="100 100 800 800"
                     xmlns="http://www.w3.org/2000/svg">
                    <path d="M832 640h-96L671.68 128h64V64H287.36v64h64.96L288 640H192v64h288v256h64v-256h288z"></path>
                </svg>
            </button>
            <button v-else class="btn btn-square btn-xs" @click="$emit('changeTop')">
                <svg class="size-[1.2em]" fill="currentColor" viewBox="100 100 800 800"
                     xmlns="http://www.w3.org/2000/svg">
                    <path d="M893.805714 455.363048l-125.805714 13.970285-128 128-15.993905 207.969524c-2.096762 27.257905-35.206095 39.497143-54.54019 20.163048l-164.717715-164.717715-213.333333 213.333334-41.496381-41.496381 213.333334-213.333333-186.051048-186.051048c-19.334095-19.334095-7.070476-52.419048 20.163048-54.540191l207.969523-15.993904 128-128 28.964572-115.833905a32.01219 32.01219 0 0 1 53.662476-14.872381l296.96 296.96c18.968381 18.968381 7.558095 51.46819-19.090286 54.442667z m-152.746666-42.057143l91.136-10.142476-224.402286-224.402286-21.479619 85.894095-154.819048 154.819048-171.812571 13.238857 310.272 310.272 13.214476-171.788191 157.891048-157.891047z"></path>
                </svg>
            </button>
            <button v-if="!note" class="btn btn-square btn-xs" @click="$emit('changeState')">
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
            <button v-if="!note" class="btn btn-square btn-xs text-red-600" @click="$emit('removeTodo')">✘</button>
        </div>
    </div>
</template>