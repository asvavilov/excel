import { computed, Ref, ref } from "vue";

export function gen(input: number, end: number)
{
	const table: any = [
		['num', Math.random()*10, input, 'prepare', '=SUM(D2:D' + end + ')+B2']
	];
	for (let r = 2; r < end; r++)
	{
		table.push([Math.random(), , , '=SUM(A' + r + ':A' + end + ')*C1*B1']);
	}
	return table;
}

type Cell = Ref<number>;

function fnSum(matrix: Cell[][], c1: number, r1: number, c2: number, r2: number, dbg = false)
{
	let sum = 0;
	for (let c = c1; c <= c2; c++)
	{
		for (let r = r1; r <= r2; r++)
		{
			sum += matrix[r]?.[c].value || 0;
		}
	}
	return sum;
}

export function gen2(input: number, end: number)
{
	const matrix: Cell[][] = [
		[ref(0), ref(input), ref(0), computed(() => fnSum(matrix, 2, 0, 2, end, true))]
	];
	for (let r = 1; r < end; r++)
	{
		matrix.push([ref(Math.random()), ref(0), computed(() => fnSum(matrix, 0, r, 0, end) * matrix[0][1].value)]);
	}

	return matrix;
}
