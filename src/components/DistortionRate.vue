<script setup lang="ts">
import {Reactive, reactive, Ref, ref, watch} from "vue";
import Decimal from "decimal.js";
import BareRoll from "./BareRoll.vue";

const precision = defineModel("precision", {
    required: true,
    type: Number
});
const userKMap = defineModel("userKMap", {
    required: true,
    type: String
});
const distortionRateTab = defineModel("distortionRateTab", {
    required: true,
    type: String
});
const setKVDialog: Ref<HTMLDialogElement | undefined, HTMLDialogElement | undefined> = ref();
const bareRollDialog: Ref<HTMLDialogElement | undefined, HTMLDialogElement | undefined> = ref();
const coped = ref(false);
const gears = ref("");  // 齿数
const pitch = ref("3.175");  // 齿距
const girth = ref("");  // 周长
const thickness = ref("1.7");   // 版材厚度
const distortionRate = ref("0");  // 变形率
const count = ref("");  // 模数
const before = ref(""); // 变形前
const after = ref("");  // 变形后
const normalAfter = ref("");  // 正常变形后
const deviation = ref("");  // 误差微调
let toGirth = true;
let toAfter = false;
const defaultK_map = new Map([
    ["1.14", 606],
    ["1.7", 989],
    ["2.28", 1352],
    ["2.54", 1605],
    ["2.84", 1704],
    ["3.94", 2394],
    ["0.95", 540],
])
let k_map: Map<string, number> = new Map(JSON.parse(JSON.stringify(Array.from(defaultK_map.entries()))));
const userK_map: Reactive<any> = reactive({})
// 读取到 userKMap 后将数据写入 userK_map
watch(userKMap, () => {
    Object.assign(userK_map, JSON.parse(userKMap.value));
}, {once: true})
const tableData: Reactive<[number, string, string][]> = reactive([]);
for (let i = 0; i < 10; i++) {
    tableData.push([i + 1, "", ""]);
}
// 四舍五入
const round = (num: Decimal) => {
    return num.toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
// 计算周长和变形率
const gearsChange = () => {
    toGirth = true;
    if (!gears.value) {
        girth.value = "";
        distortionRate.value = "0";
        before.value = "";
        after.value = "";
        computeTableData(true);
        return;
    }
    girth.value = round(Decimal.mul(gears.value, pitch.value));
    computeDistortionRate();
    countChange();
    computeTableData();
}
const pitchChange = () => {
    if (toGirth) {
        gearsChange();
    } else {
        girthChange();
    }
}
// 计算齿数和变形率
const girthChange = () => {
    toGirth = false;
    if (!girth.value) {
        gears.value = "";
        distortionRate.value = "0";
        before.value = "";
        after.value = "";
        computeTableData(true);
        return;
    }
    gears.value = round(Decimal.div(girth.value, pitch.value));
    computeDistortionRate();
    countChange();
    computeTableData();
}
// 计算变形率
const computeDistortionRate = () => {
    if (!girth.value) return;
    let num = Number(girth.value);
    if (!num) return;
    let k = k_map.get(thickness.value) || 0;
    distortionRate.value = round(Decimal.sub(100, Decimal.div(k, num)));
    if (distortionRateTab.value === "例外" && deviation.value) {
        normalAfter.value = round(Decimal.mul(girth.value, Decimal.div(distortionRate.value, 100)));
        let newAfter = Decimal.add(normalAfter.value, deviation.value);
        distortionRate.value = round(Decimal.mul(100, Decimal.div(newAfter, girth.value)));
    }
    if (toAfter) {
        computeAfter();
    } else {
        computeBefore();
    }
    tableData.forEach((row) => {
        if (!row[1]) return;
        row[2] = round(Decimal.mul(row[1], Decimal.div(distortionRate.value, 100)));
    })
}
const countChange = () => {
    count.value = count.value.replace(/(\D)/, '');
    if (!count.value || !girth.value) return;
    let num1 = Number(girth.value);
    let num2 = Number(count.value);
    if (!num1 || !num2) return;
    before.value = round(Decimal.div(num1, num2));
    computeAfter();
}
// 计算变形后
const computeAfter = () => {
    toAfter = true;
    if (!before.value) {
        after.value = "";
        return;
    }
    after.value = round(Decimal.mul(before.value, Decimal.div(distortionRate.value, 100)));
}
// 计算变形前
const computeBefore = () => {
    toAfter = false;
    if (!after.value) {
        before.value = "";
        return;
    }
    before.value = round(Decimal.div(after.value, Decimal.div(distortionRate.value, 100)));
}
const computeTableData = (reset: boolean = false) => {
    let num = Number(girth.value);
    tableData.forEach((row) => {
        if (num && !reset) {
            row[1] = round(Decimal.div(num, row[0]));
            row[2] = round(Decimal.mul(row[1], Decimal.div(distortionRate.value, 100)));
        } else {
            row[1] = "";
            row[2] = "";
        }
    })
}
// 复制到系统剪贴板
const copy = (value: string) => {
    navigator.clipboard.writeText(value);
    coped.value = true;
    setTimeout(() => {
        coped.value = false;
    }, 1500);
}
// 设置新 K 值
const setK_map = (k: string, new_value: string, default_value: number) => {
    let num = Number(new_value);
    if (num) {
        k_map.set(k, num);
    } else {
        k_map.set(k, default_value);
    }
    recalculate();
    userKMap.value = JSON.stringify(userK_map);
}
// 初始化 K 值
k_map.forEach((_value, key) => {
    let num = Number(userK_map[key]);
    if (num) {
        k_map.set(key, num);
    }
})
// 恢复默认 K 值
const resetK_map = () => {
    Object.assign(userK_map, {
        "1.14": "",
        "1.7": "",
        "2.28": "",
        "2.54": "",
        "2.84": "",
        "3.94": "",
        "0.95": ""
    });
    k_map = new Map(JSON.parse(JSON.stringify(Array.from(defaultK_map.entries()))));
    recalculate();
    userKMap.value = "";
}
// 重新计算
const recalculate = () => {
    if (toGirth) {
        gearsChange();
    } else {
        girthChange();
    }
}
// 改变小数位数时重新计算
watch([precision, distortionRateTab], () => {
    if (distortionRateTab.value === "通用") {
        recalculate();
    }
})

const client: Ref<string[], string[]> = ref([]);
const clients = reactive({
    a: ["70", "80", "95", "100", "101", "120", "132"],
    b: ["71", "81", "95", "100", "101", "120", "132"],
    c: ["72", "82", "95", "100", "101", "120", "132"],
    d: ["73", "83", "95", "100", "101", "120", "132"],
    f: ["74", "84", "95", "100", "101", "120", "132"],
});
</script>

<template>
    <div role="tablist" class="tabs tabs-border tabs-sm">
        <input type="radio" name="distortionRate" class="tab" aria-label="通用" value="通用" v-model="distortionRateTab"/>
        <div class="tab-content p-3">
            <div class="grid grid-cols-24 gap-x-2 gap-y-4 items-center">
                <label class="col-span-5 text-sm">齿数</label>
                <label class="col-span-8 input input-xs w-30">
                    <input type="number" v-model="gears" @input="gearsChange"/>
                    T
                </label>
                <label class="col-span-4 text-sm">齿距</label>
                <div class="col-span-7 join">
                    <input class="join-item btn input-xs" type="radio" name="pitch" aria-label="3.175" value="3.175"
                           v-model="pitch" @change="pitchChange" checked/>
                    <input class="join-item btn input-xs" type="radio" name="pitch" aria-label="5" value="5"
                           v-model="pitch"
                           @change="pitchChange"/>
                </div>
                <label class="col-span-5 text-sm" @click="copy(girth)">版辊周长</label>
                <label class="col-span-8 input input-xs w-30">
                    <input type="number" v-model="girth" @input="girthChange"/>
                    mm
                </label>
                <label class="col-span-4 text-sm text-red-500" @click="copy(distortionRate)">变形率</label>
                <kbd class="col-span-7 text-red-500 kbd kbd-md w-27">{{ distortionRate }} %</kbd>
                <label class="col-span-5 text-sm">版材厚度</label>
                <div class="col-span-19 join join-vertical">
                    <div class="join">
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="1.14"
                               value="1.14"
                               v-model="thickness" @change="computeDistortionRate"/>
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="1.7"
                               value="1.7"
                               v-model="thickness" @change="computeDistortionRate" checked/>
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.28"
                               value="2.28"
                               v-model="thickness" @change="computeDistortionRate"/>
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.54"
                               value="2.54"
                               v-model="thickness" @change="computeDistortionRate"/>
                    </div>
                    <div class="join">
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.84"
                               value="2.84"
                               v-model="thickness" @change="computeDistortionRate"/>
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="3.94"
                               value="3.94"
                               v-model="thickness" @change="computeDistortionRate"/>
                        <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="0.95"
                               value="0.95"
                               v-model="thickness" @change="computeDistortionRate"/>
                    </div>
                </div>
            </div>
        </div>
        <input type="radio" name="distortionRate" class="tab" aria-label="例外" value="例外" v-model="distortionRateTab"/>
        <div class="tab-content p-3">
            <div class="grid grid-cols-24 gap-x-2 gap-y-4 items-center">
                <label class="col-span-5 text-sm">客户</label>
                <select class="col-span-8 select select-xs w-30" v-model="client">
                    <option v-for="(v, k) in clients" :label="k" :value="v"/>
                </select>
                <label class="col-span-4 text-sm">齿数</label>
                <label class="col-span-7 input input-xs">
                    <input type="number" v-model="gears" @input="gearsChange"/>
                    T
                </label>
                <label class="col-span-5 text-sm" @click="copy(girth)">版辊周长</label>
                <label class="col-span-8 input input-xs w-30">
                    <input type="number" v-model="girth" @input="girthChange"/>
                    mm
                </label>
                <label class="col-span-4 text-sm">正常变形</label>
                <kbd class="col-span-7 kbd kbd-md w-27">{{ normalAfter }} mm</kbd>
                <label class="col-span-5 text-sm">误差微调</label>
                <label class="col-span-8 input input-xs w-30">
                    <input type="number" v-model="deviation"/>
                    mm
                </label>
                <label class="col-span-4 text-sm text-red-500" @click="copy(distortionRate)">变形率</label>
                <kbd class="col-span-7 text-red-500 kbd kbd-md w-27">{{ distortionRate }} %</kbd>
            </div>
        </div>
        <input type="radio" name="distortionRate" class="tab" aria-label="固定" value="固定" v-model="distortionRateTab"/>
        <div class="tab-content p-3">
            <div class="grid grid-cols-24 gap-x-2 gap-y-4 items-center">
                <label class="col-span-5 text-sm">客户</label>
                <select class="col-span-8 select select-xs w-30" v-model="client">
                    <option v-for="(v, k) in clients" :label="k" :value="v"/>
                </select>
                <label class="col-span-4 text-sm">齿数</label>
                <select class="col-span-7 select select-xs" v-model="gears" @change="gearsChange">
                    <option v-for="item in client" :label="item + 'T'" :value="item"/>
                </select>
                <label class="col-span-5 text-sm" @click="copy(girth)">版辊周长</label>
                <label class="col-span-8 input input-xs w-30">
                    <input type="number" v-model="girth" @input="girthChange"/>
                    mm
                </label>
                <label class="col-span-4 text-sm text-red-500" @click="copy(distortionRate)">变形率</label>
                <kbd class="col-span-7 text-red-500 kbd kbd-md w-27">{{ distortionRate }} %</kbd>
            </div>
        </div>
    </div>
    <div class="border border-base-content/5 bg-base-100 table-sm">
        <table class="table text-center">
            <thead>
            <tr>
                <th>模数</th>
                <th>变形前</th>
                <th>变形后</th>
            </tr>
            </thead>
            <tbody>
            <tr>
                <th>
                    <input class="input input-xs w-18 justify-items-center" v-model="count"
                           @input="countChange"/>
                </th>
                <td>
                    <input class="input input-xs w-32" type="number" v-model="before"
                           @input="computeAfter"/>
                </td>
                <td>
                    <input class="input input-xs w-32" type="number" v-model="after"
                           @input="computeBefore"/>
                </td>
            </tr>
            <tr v-for="item in tableData">
                <th>{{ item[0] }}</th>
                <td>{{ item[1] }}</td>
                <td>{{ item[2] }}</td>
            </tr>
            </tbody>
        </table>
    </div>
    <button v-if="distortionRateTab==='例外'" class="btn btn-outline btn-primary btn-xs fixed top-10 right-3">管理例外规则
    </button>
    <button v-if="distortionRateTab==='通用'" class="btn btn-outline btn-primary btn-xs fixed top-10 right-23"
            @click="setKVDialog?.showModal()">K值设置
    </button>
    <button v-if="distortionRateTab==='通用'" class="btn btn-outline btn-secondary btn-xs fixed top-10 right-3"
            @click="bareRollDialog?.showModal()">
        光辊变形率
    </button>
    <div v-if="coped" role="alert"
         class="fixed top-1/3 left-1/2 -translate-y-1/2 -translate-x-1/2 alert alert-success alert-soft">
        <svg class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <span>已复制到剪贴板！</span>
    </div>
    <dialog ref="setKVDialog" class="modal">
        <div class="modal-box flex flex-col gap-3 items-center w-77">
            <h6 class="text-lg font-bold">常量K值设置</h6>
            <p class="text-sm">输入数字后自动保存，留空保持默认</p>
            <label v-for="item in defaultK_map" class="input input-sm">
                <span class="label w-18">{{ item[0] }}</span>
                <input type="number" :placeholder="'默认值：'+ item[1]" v-model="userK_map[item[0]]"
                       @change="setK_map(item[0], userK_map[item[0]], item[1])"/>
            </label>
            <div class="flex justify-around items-end w-50 h-10">
                <button class="btn btn-outline btn-primary btn-sm w-20" @click="resetK_map">恢复默认</button>
                <button class="btn btn-outline btn-sm w-20" @click="setKVDialog?.close()">关闭</button>
            </div>
        </div>
    </dialog>
    <dialog ref="bareRollDialog" class="modal">
        <div class="modal-box flex flex-col gap-4 items-center">
            <h6 class="text-lg font-bold">光辊变形率计算</h6>
            <BareRoll :k_map="k_map" :precision="precision"/>
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost fixed right-2 top-2">✕</button>
            </form>
        </div>
    </dialog>
    <div v-if="distortionRateTab==='固定'" class="drawer z-10">
        <input id="fixed-rules" type="checkbox" class="drawer-toggle"/>
        <div class="drawer-content">
            <label for="fixed-rules"
                   class="btn btn-outline btn-primary btn-xs fixed top-10 right-3">管理固定规则</label>
        </div>
        <div class="drawer-side">
            <div class="min-h-full w-full bg-base-200 flex flex-col p-3 gap-3">
                <div class="flex justify-between items-center">
                    <label for="fixed-rules" class="btn btn-ghost btn-sm">＜返回</label>
                    <button class="btn btn-outline btn-primary btn-xs">添加固定规则</button>
                </div>
            </div>
        </div>
    </div>
</template>