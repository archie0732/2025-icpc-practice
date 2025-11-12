# Kadane

>[!note]
>Kadane 演算法，dp 的延伸，複雜度: O(n)

## 使用時機

當要求找到一個陣列裡**連續**的總和最大值或最小值

### 使用方法

- 演示找最大值

```py
cur = 0
mx = 0
for x in arr:
  cur = max(cur, cur + x)
  mx = max(mx, cur)
return mx
```

