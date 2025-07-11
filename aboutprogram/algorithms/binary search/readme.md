# binary search


- 有以下兩種`l <= r` 與 `l < r`

## `l < r`

- 適合用在 upper_bound / lower_bound 風格
- 最後 l == r 是答案所在位置（或剛好超過）
- `r = mid`	`mid` 可能是解 ⇒ 保留右半

| 時機                   | 說明                         |
| -------------------- | -------------------------- |
| 二分搜尋一個「答案區間」         | 如最大化最小值、最小化最大值等決策性問題（決策單調） |
| 搜尋某個臨界點              | 例如「第一個滿足條件的位置」             |
| 搜尋的是某個值域範圍而非陣列 index | e.g. 木材長度、速度、資源量           |


## `l <= r`

- 適合找具體數值或條件成立與否的某一點
- 最後通常是用 return l 或 return r 判斷解是否存在
- 要 `r = mid-1` 是因為 mid 已經確認不滿足條件，可安全排除掉它

| 時機                 | 說明                                       |
| ------------------ | ---------------------------------------- |
| 查找確切數值是否存在         | 如 `nums[mid] == target` 時回傳 mid          |
| 查找符合條件的最左/最右 index | 例如「最後一個 ≤ target 的位置」或「第一個 ≥ target 的位置」 |
| 查找某區間內最大/最小的合法值    | 通常搭配 `ans = mid` 更新答案                    |
| 二分搜尋在原始陣列          | 因為 `mid` 是有效 index，會進入 `nums[mid]` 訪問    |


---

## 使用時機

| 方面                 | `while (l <= r)`       | `while (l < r)` |
| ------------------ | ---------------------- | --------------- |
| 區間定義               | `[l, r]`（封閉）           | `[l, r)`（半開）    |
| 是否進入 l == r        | ✅ 會進入                  | ❌ 不會進入          |
| 適合找 index 嗎？       | ✅ 很適合                  | ⚠ 要小心是否超出 index |
| 適合找值域範圍的答案嗎？       | ⚠ 不太適合                 | ✅ 最適合           |
| 需要訪問 nums\[mid] 嗎？ | ✅ 可以安全訪問               | ⚠ 要注意 r 不會越界    |
| 最後怎麼寫 return       | 多用 `return ans` 或 `-1` | 多用 `return l`   |

- 列舉

| 目標                  | 建議使用         |
| ------------------- | ------------ |
| 要找某值在不在陣列           | `l <= r`     |
| 要找滿足條件的最左/最右 index  | `l <= r`     |
| 要找滿足條件的最小/最大值（值域答案） | `l < r`      |
| 想設計範圍收斂成單一值         | `l < r` 比較乾淨 |


### 整理

|條件型式	|while (l < r)	|while (l <= r)|
|--------|-------------|---------|
|左邊界更新|	l = mid + 1|	l = mid + 1|
|右邊界更新	|r = mid	|r = mid - 1|
|是否保留 mid|	是（可能是解）|	否（確定不是解）|
|何時適用|	要找第一個符合條件的位置	|要精準找某個數字|


![](anime_girls_anime_artwork_Fate_Series_Tohsaka_Rin_Archer_Fate_Stay_Night-131936.jpg)