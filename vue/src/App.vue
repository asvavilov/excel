<template>
  <p>see console</p>
  пользовательское значение: <input type="number" v-model="num">
  &nbsp;
  строк: <input type="number" v-model="end">
  <hr>
  hyperformula:
  <button :disabled="wait" @click="wait = true; prepare(); wait = false;">prepare</button>
  <button :disabled="wait" @click="wait = true; calc(); wait = false;">calc</button>
  <br>
  my:
  <button :disabled="wait" @click="wait = true; prepare2(); wait = false;">prepare2</button>
  <button :disabled="wait" @click="wait = true; calc2(); wait = false;">calc2</button>
</template>

<script lang="ts">
import { defineComponent, Ref, ref, unref } from 'vue';
import { HyperFormula } from 'hyperformula';
import { gen, gen2 } from './lib/gen';

const wait = ref(false);

export default defineComponent({
  name: 'App',
  setup()
  {
    const num = ref(2);
    const end = ref(10000);

    // test hyperformula
    const data: Ref<(number|string)[][]|null> = ref(null);
    const prepare = () => {
      data.value = gen(num.value, end.value);
    };
    const calc = () => {
      const options = {
        licenseKey: 'gpl-v3'
      };

      if (!data.value)
      {
        return;
      }

      console.log('rows', end.value);

      if (data.value[0] !== undefined && data.value[0][2] !== undefined)
      {
        data.value[0][2] = num.value;
      }

      performance.clearMarks();
      performance.clearMeasures();
      performance.mark('calcBegin');

      // build an instance with defined options and data
      const hfInstance = HyperFormula.buildFromArray(data.value, options);

      // call getCellValue to get the calculation results
      const mySum = hfInstance.getCellValue({ col: 4, row: 0, sheet: 0 });

      // this outputs the result in the browser's console
      console.log('sum', mySum);

      performance.mark('calcEnd');
      performance.measure('calc', 'calcBegin', 'calcEnd');
      console.log(performance.getEntriesByName('calc')[0].duration, 'ms');
    };

    // test my
    let matrix: Ref<number>[][]|null = null;
    const prepare2 = () => {
      matrix = gen2(num.value, end.value);
    };
    const calc2 = () => {
      if (!matrix)
      {
        return;
      }

      console.log('rows', end.value);

      matrix[0][1].value = num.value;

      performance.clearMarks();
      performance.clearMeasures();
      performance.mark('calc2Begin');
      console.log('sum', matrix[0][3].value);
      //console.table(matrix.map(r => r.map(c => c.value)))
      performance.mark('calc2End');
      performance.measure('calc2', 'calc2Begin', 'calc2End');
      console.log(performance.getEntriesByName('calc2')[0].duration, 'ms');
    };

    return {
      wait,
      num,
      end,
      prepare,
      calc,
      prepare2,
      calc2,
    }
  }
});
</script>
