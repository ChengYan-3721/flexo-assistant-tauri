<script setup lang="ts">
import {ref} from "vue";
import Decimal from "decimal.js";

const {k_map, precision} = defineProps({
    k_map: { type: Map<string, number>, required: true },
    precision: { type: Number, required: true }
})
const bareRollerDiameter1 = ref(""); // 光滚筒直径（Bare cylinder diameter）
const bareRollerCircumference1 = ref(""); // 光辊筒周长
const mountingTapeThickness1 = ref(""); // 柔版印刷中常用PET双面胶（0.38mm/0.5mm）
const plateThickness1 = ref("1.7"); // 版材厚度
const printingCircumference1 = ref(""); // 印刷周长
const distortionRate1 = ref("0"); // 变形率
const bareRollerDiameter2 = ref("");
const bareRollerCircumference2 = ref("");
const mountingTapeThickness2 = ref("");
const plateThickness2 = ref("2.28");
const printingCircumference2 = ref("");
const distortionRate2 = ref("0");
const plateThicknessList = ["1.14", "1.7", "2.28", "2.54", "2.84", "3.94", "0.95"];
const pi = 3.14159265;

// 四舍五入
const round = (num: Decimal) => {
    return num.toFixed(precision).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
// 计算祼辊周长和直径
const computeBareRoller = () => {
    if (!mountingTapeThickness1.value) {
        bareRollerDiameter1.value = "";
        bareRollerCircumference1.value = "";
        return;
    }
    let diameter = Decimal.div(printingCircumference1.value, pi);
    bareRollerDiameter1.value = round(Decimal.sub(Decimal.sub(Decimal.sub(Decimal.sub(diameter, plateThickness1.value),
        plateThickness1.value), mountingTapeThickness1.value), mountingTapeThickness1.value));
    bareRollerCircumference1.value = round(Decimal.mul(bareRollerDiameter1.value, pi));
    bareRollerDiameter2.value = bareRollerDiameter1.value;
    bareRollerCircumference2.value = bareRollerCircumference1.value;
    computePrintingCircumference();
}
// 计算印刷周长
const computePrintingCircumference = () => {
    if (!mountingTapeThickness2.value) return;
    let diameter = Decimal.add(Decimal.add(Decimal.add(Decimal.add(bareRollerDiameter2.value, mountingTapeThickness2.value),
        mountingTapeThickness2.value), plateThickness2.value), plateThickness2.value);
    printingCircumference2.value = round(Decimal.mul(diameter, pi));
    computeDistortionRate2();
}
// 计算变形率1
const computeDistortionRate1 = () => {
    if (!printingCircumference1.value) {
        bareRollerDiameter1.value = "";
        bareRollerCircumference1.value = "";
        distortionRate1.value = "0";
        return;
    }
    let num = Number(printingCircumference1.value);
    if (!num) return;
    let k = k_map.get(plateThickness1.value) || 0;
    distortionRate1.value = round(Decimal.sub(100, Decimal.div(k, num)));
    computeBareRoller();
}
// 计算变形率2
const computeDistortionRate2 = () => {
    if (!printingCircumference2.value) return;
    let num = Number(printingCircumference2.value);
    if (!num) return;
    let k = k_map.get(plateThickness2.value) || 0;
    distortionRate2.value = round(Decimal.sub(100, Decimal.div(k, num)));
}
// 计算光辊周长2
const computeBareRollerCircumference2 = () => {
    if (bareRollerDiameter2.value) {
        bareRollerCircumference2.value = round(Decimal.mul(bareRollerDiameter2.value, pi));
        computePrintingCircumference();
    } else {
        bareRollerCircumference2.value = "";
        printingCircumference2.value = "";
        distortionRate2.value = "0";
    }
}
// 计算光辊直径2
const computeBareRollerDiameter2 = () => {
    if (bareRollerCircumference2.value) {
        bareRollerDiameter2.value = round(Decimal.div(bareRollerCircumference2.value, pi));
        computePrintingCircumference();
    } else {
        bareRollerDiameter2.value = "";
        printingCircumference2.value = "";
        distortionRate2.value = "0";
    }
}
</script>

<template>
    <div class="grid grid-cols-24 gap-x-1 gap-y-3 items-center">
        <label class="col-span-6 text-sm"></label>
        <label class="col-span-9 text-sm justify-self-center">计算光辊直径 ↑</label>
        <label class="col-span-9 text-sm justify-self-center">计算印刷周长 ↓</label>
        <label class="col-span-6 text-sm">光辊筒直径</label>
        <kbd class="col-span-9 kbd kbd-md">{{ bareRollerDiameter1 }}</kbd>
        <input class="col-span-9 input input-xs" type="number"
               v-model="bareRollerDiameter2" @input="computeBareRollerCircumference2()" />
        <label class="col-span-6 text-sm">光辊筒周长</label>
        <kbd class="col-span-9 kbd kbd-md">{{ bareRollerCircumference1 }}</kbd>
        <input class="col-span-9 input input-xs" type="number"
               v-model="bareRollerCircumference2" @input="computeBareRollerDiameter2()"/>
        <label class="col-span-6 text-sm">双面胶厚度</label>
        <input class="col-span-9 input input-xs" type="number"
               v-model="mountingTapeThickness1" @input="computeBareRoller()"/>
        <input class="col-span-9 input input-xs" type="number"
               v-model="mountingTapeThickness2" @input="computePrintingCircumference()"/>
        <label class="col-span-6 text-sm">版材厚度</label>
        <select class="col-span-9 select select-xs" v-model="plateThickness1" @change="computeBareRoller()">
            <option v-for="item in plateThicknessList" :label="item" :value="item"/>
        </select>
        <select class="col-span-9 select select-xs" v-model="plateThickness2" @change="computePrintingCircumference()">
            <option v-for="item in plateThicknessList" :label="item" :value="item"/>
        </select>
        <label class="col-span-6 text-sm">印刷周长</label>
        <input class="col-span-9 input input-xs" type="number"
               v-model="printingCircumference1" @input="computeDistortionRate1()"/>
        <kbd class="col-span-9 kbd kbd-md">{{ printingCircumference2 }}</kbd>
        <label class="col-span-6 text-red-500 text-sm">变形率</label>
        <kbd class="col-span-9 text-red-500 kbd kbd-md">{{ distortionRate1 }} %</kbd>
        <kbd class="col-span-9 text-red-500 kbd kbd-md">{{ distortionRate2 }} %</kbd>
    </div>
</template>