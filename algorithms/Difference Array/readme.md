# Difference Array 插分陣列

>[!note]
>常用於區間更新

## 使用時機

當給多個連續區間需要更新時

- 例如

```py
arr = [1,2,3,4,5] # index 1~3 +10 index 2~4 -5
```

如果用傳統方法更新在長陣列、多次操作下會炸

## 使用方法

>[!tip]
>可以注意到，單一操作下 arr[i] 到 arr[j] 之間要調整的值都一樣，也可以看成頭尾操作(只要調整兩個)

建立一個插分陣列 `diff`

>[!import]
>當陣列值不一樣時，就會需要重新構建插分陣列

- 構建初始`diff`

```py
diff[0] = arr[0]
diff[i] = arr[i] - arr[i-1]
```

>需注意，如果陣列初始值都一樣，則不用重構插分陣列，直接 `diff = [0] * n`

- 操作更新值

要更新值時(ex: i~j 加`value`): `diff[i] += value`, `diff[j+1] -= value`

反之如果操作是減`value`時也是: `diff[i] -= vlaue`, `diff[j+1] -= value`

>[!note]
> 這樣子就可以省去由 arr[i]~arr[j]一個個去調整的時間

- 如何還原?

>其實我們可以用`Prefix Sum`來做還原，應該說兩者是互相的

```py
arr[0] = diff[0]
arr[i] = arr[i-1] + diff[i]
```

2025 @ arch1e & copyright
