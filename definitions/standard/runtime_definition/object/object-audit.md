# 26.05.2024 - First Iteration

## Removed:
 - concat
 - entries
 - any
 - forEach
 - containsValue
 - values
 - flter

# 27.05.2024

## Removed:
 - at
 - replace

## Renamed:
 - put -> set

# 05.06.2025

## Removed
- remove

# 06.02.2026

## Added
- get


## Todo
current get impl looks like:
<K. T> get(object: Object<K>, key: Text): T

But this should be changed to:
<K, T> get(object: Object<K>, key: T keyof K): T ofkey Object<K>

When the tucana definitions are adjusted
