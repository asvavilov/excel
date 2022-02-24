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
  reactive:
  <button :disabled="wait" @click="wait = true; prepare2(); wait = false;">prepare2</button>
  <button :disabled="wait" @click="wait = true; calc2(); wait = false;">calc2</button>
  <br>
  js:
  <button :disabled="wait" @click="wait = true; prepare3(); wait = false;">prepare3</button>
  <button :disabled="wait" @click="wait = true; calc3(); wait = false;">calc3</button>
</template>

<script lang="ts">
import { defineComponent, Ref, ref } from 'vue';
import { HyperFormula } from 'hyperformula';
import { Cell, cellValue, gen, gen2, gen3 } from './lib/gen';

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

    // test reactive
    let matrixRef: Ref<number>[][]|null = null;
    const prepare2 = () => {
      matrixRef = gen2(num.value, end.value);
    };
    const calc2 = () => {
      if (!matrixRef)
      {
        return;
      }

      console.log('rows', end.value);

      matrixRef[0][1].value = num.value;

      performance.clearMarks();
      performance.clearMeasures();
      performance.mark('calc2Begin');
      console.log('sum', matrixRef[0][3].value);
      //console.table(matrix.map(r => r.map(c => c.value)))
      performance.mark('calc2End');
      performance.measure('calc2', 'calc2Begin', 'calc2End');
      console.log(performance.getEntriesByName('calc2')[0].duration, 'ms');
    };

    // test js
    let matrix: Cell[][]|null = null;
    const prepare3 = () => {
      matrix = gen3(num.value, end.value);
    };
    const calc3 = () => {
      if (!matrix)
      {
        return;
      }

      console.log('rows', end.value);

      matrix[0][1] = num.value;

      performance.clearMarks();
      performance.clearMeasures();
      performance.mark('calc3Begin');
      console.log('sum', cellValue(matrix[0][3]));
      //console.table(matrix)
      performance.mark('calc3End');
      performance.measure('calc3', 'calc3Begin', 'calc3End');
      console.log(performance.getEntriesByName('calc3')[0].duration, 'ms');
    };

    return {
      wait,
      num,
      end,
      prepare,
      calc,
      prepare2,
      calc2,
      prepare3,
      calc3,
    }
  }
});
</script>
