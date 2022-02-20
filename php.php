<?php
function gen($end)
{
	$arr = [];
	for ($r = 0; $r < $end; $r++)
	{
		$arr[] = rand(0, 999999) / 1000000;
	}
	return $arr;
}

function sum($arr, $row, $end)
{
	$sum = 0;
	for ($r = $row; $r < $end; $r++)
	{
		$sum += $arr[$r];
	}
	return $sum;
}

$end = 10000;
$col0 = gen($end);

$begin = microtime(true);
$col1 = [];
foreach ($col0 as $r => $val)
{
	$col1[] = sum($col0, $r, $end);
}
$sum = sum($col1, 0, $end);

echo "rows $end\n";
echo "sum $sum\n";
echo round((microtime(true) - $begin) * 1000)." ms";
