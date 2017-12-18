extern crate multiarray;

use self::multiarray::*;
use std::cmp;

fn read_matrix(input: &str) -> Array2D<u16> {
    let rows: Vec<&str> = input.split('\n').collect();
    let row_count = rows.len();
    let first_cols: Vec<&str> = rows[0].split_whitespace().collect();
    let col_count = first_cols.len();
    let mut result = Array2D::new([row_count, col_count], 0);

    for row_idx in 0..row_count {
        let row = rows[row_idx];
        let cols: Vec<u16> = row.split_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        for col_idx in 0..col_count {
            result[[row_idx, col_idx]] = cols[col_idx];
        }
    }

    result
}

fn min_max_in(lane: Array1DRef<u16>) -> (u16, u16) {
    assert!(lane.extents()[0] > 0);

    let mut min: Option<u16> = None;
    let mut max: Option<u16> = None;
    for item in lane {
        min = Some(cmp::min(*item, min.unwrap_or(*item)));
        max = Some(cmp::max(*item, max.unwrap_or(*item)));
    }

    (min.unwrap(), max.unwrap())
}

fn sum_of_divided(lane: Array1DRef<u16>) -> u32 {
    let mut sum: u32 = 0;
    let len = lane.extents()[0];

    for i in 0..len {
        for j in 0..len {
            if i != j {
                let a = lane[i];
                let b = lane[j];

                if (a % b) == 0 {
                    sum += (a / b) as u32;
                }
            }
        }
    }

    sum
}

pub fn run() {
    let input = r#"493	458	321	120	49	432	433	92	54	452	41	461	388	409	263	58
961	98	518	188	958	114	1044	881	948	590	972	398	115	116	451	492
76	783	709	489	617	72	824	452	748	737	691	90	94	77	84	756
204	217	90	335	220	127	302	205	242	202	259	110	118	111	200	112
249	679	4015	106	3358	1642	228	4559	307	193	4407	3984	3546	2635	3858	924
1151	1060	2002	168	3635	3515	3158	141	4009	3725	996	142	3672	153	134	1438
95	600	1171	1896	174	1852	1616	928	79	1308	2016	88	80	1559	1183	107
187	567	432	553	69	38	131	166	93	132	498	153	441	451	172	575
216	599	480	208	224	240	349	593	516	450	385	188	482	461	635	220
788	1263	1119	1391	1464	179	1200	621	1304	55	700	1275	226	57	43	51
1571	58	1331	1253	60	1496	1261	1298	1500	1303	201	73	1023	582	69	339
80	438	467	512	381	74	259	73	88	448	386	509	346	61	447	435
215	679	117	645	137	426	195	619	268	223	792	200	720	260	303	603
631	481	185	135	665	641	492	408	164	132	478	188	444	378	633	516
1165	1119	194	280	223	1181	267	898	1108	124	618	1135	817	997	129	227
404	1757	358	2293	2626	87	613	95	1658	147	75	930	2394	2349	86	385"#;

    let input_matrix = read_matrix(input);
    let n = input_matrix.extents()[0];

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    for row_idx in 0..n {
        let row = input_matrix.eliminated_dim(0, row_idx);
        let (min, max) = min_max_in(row);
        sum1 += (max - min) as u32;
        sum2 += sum_of_divided(row);
    }
    println!("Day2 result 1: {}", sum1);
    println!("Day2 result 2: {}", sum2);
}