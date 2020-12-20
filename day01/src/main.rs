// Advent of Code 2020 day 1
// Copyright (C) 2020  Victor "VoxWave" Bankowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::BufReader;
use std::{collections::HashSet, error::Error, io::prelude::*};

const INPUT: [u64; 200] = [
    1544, 1560, 1947, 1659, 1972, 1940, 1977, 1689, 1916, 1638, 1804, 1543, 1789, 545, 968, 1959,
    1783, 1869, 1581, 1976, 1859, 1660, 1793, 69, 1653, 1866, 1541, 1920, 1751, 1681, 1829, 2009,
    1752, 680, 1864, 1628, 1917, 1876, 2002, 1974, 1827, 1791, 1552, 1669, 1849, 1167, 1744, 1764,
    1913, 1782, 1926, 1795, 1738, 1877, 1811, 1746, 1682, 1943, 1761, 1850, 983, 1617, 1901, 1750,
    1842, 1588, 1679, 1759, 1994, 1847, 1657, 1981, 1648, 1996, 1572, 1953, 1555, 1665, 1680, 1872,
    1826, 1316, 1962, 1893, 1545, 1535, 1895, 1819, 1891, 1919, 1853, 1831, 704, 1978, 1780, 1722,
    1652, 1625, 478, 1030, 1985, 1720, 1817, 264, 1988, 1892, 1712, 1222, 1840, 1894, 1906, 1890,
    1846, 1939, 1991, 1835, 1799, 1865, 1663, 1908, 1575, 1970, 1956, 1556, 1688, 1558, 1698, 1771,
    1807, 1878, 1707, 1770, 1823, 1802, 1930, 1703, 1136, 1910, 1998, 1973, 1611, 1979, 1612, 1838,
    1715, 1885, 1879, 1904, 1941, 1734, 1900, 1809, 1691, 1848, 1683, 1754, 1874, 1975, 1896, 1567,
    1785, 1644, 1922, 1651, 1046, 1971, 1600, 1933, 1857, 1960, 1948, 1675, 1828, 1633, 1868, 1615,
    1884, 1674, 1860, 1775, 995, 1596, 2006, 1737, 1649, 1997, 1767, 1784, 1705, 1664, 1766, 1839,
    1533, 1935, 1796, 1781, 1589, 1594, 1987, 1769,
];

fn main() -> Result<(), Box<dyn Error>> {
    // part1()?;
    for _ in 0..1000 {
        part2();
    }
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut nmbr_set = HashSet::new();
    for num in INPUT.iter() {
        if *num < 2020 {
            if nmbr_set.contains(&(2020 - num)) {
                println!("Answer: {}", num * (2020 - num));
                break;
            } else {
                nmbr_set.insert(num);
            }
        }
    }
    Ok(())
}

#[inline(never)]
fn part2() {
    let mut vec_o_vecs: Vec<Vec<u64>> = Vec::new();
    'outer: for num in INPUT.iter() {
        for i in 0..vec_o_vecs.len() {
            let cur_sum: u64 = vec_o_vecs[i].iter().sum::<u64>() + num;
            if vec_o_vecs[i].len() == 2 && cur_sum == 2020 {
                println!("{}, {}", vec_o_vecs[i].iter().product::<u64>() * num, vec_o_vecs.len());
                break 'outer;
            } else if cur_sum < 2020 {
                let mut clone = vec_o_vecs[i].clone();
                clone.push(*num);
                vec_o_vecs.push(clone);
            }
        }
        vec_o_vecs.push(vec![*num]);
    }
}
