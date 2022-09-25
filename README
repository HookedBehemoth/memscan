# memscan

[scanmem](https://github.com/scanmem/scanmem) inspired memory search library.

Work in Progress
## Not implemented yet
- remote process search
- string search
- arbitrary byte array search

# Performance
Currently this ships with three code paths.
- AVX2 (Intel Haswell, AMD Zen or later)
- SSE4.2 (Intel Nehalem, AMD Bulldozer)
- Fallback
A scalar path is planed for integers.

Numbers recorded on a Ryzen 9 3900x

## L1 Cache
|     |  AVX2  | SSE4.2 | Fallback |
|-----|--------|--------|----------|
| u8  | 88GB/s | 44GB/s |  1.9GB/s |
| u16 | 88GB/s | 44GB/s |  5.8GB/s |
| u32 | 88GB/s | 44GB/s |  7.8GB/s |
| u64 | 88GB/s | 44GB/s | 21.8GB/s |
| f32 | 88GB/s | 44GB/s |  7.6GB/s |
| f64 | 88GB/s | 44GB/s |   19GB/s |

## L2 Cache
|     |  AVX2  | SSE4.2 | Fallback |
|-----|--------|--------|----------|
| u8  | 74GB/s | 43GB/s |  2.3GB/s |
| u16 | 74GB/s | 43GB/s |  5.8GB/s |
| u32 | 74GB/s | 43GB/s | 11.5GB/s |
| u64 | 74GB/s | 43GB/s |   24GB/s |
| f32 | 74GB/s | 43GB/s |  9.4GB/s |
| f64 | 74GB/s | 43GB/s |   19GB/s |

There is a lot of variation in L3 and main memory speed so it's not included here.

# License
Copyright (C) 2022  HookedBehemoth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
