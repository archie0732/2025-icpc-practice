# Monotonic Stack

>[!note]
>- 單調棧
>遞增（遞減）的 stack

## 使用時機

- 需要往前或是往後尋找，下一個比自己大或比自己小的元素就可以使用


## 常用

> PLE(Previous Less Element)

```cpp
vector<int> nums;
vector<int> ple(nums.size());
stack<int> st; // 存放index
for(int i = 0; i < nums.size(); ++i) {
    while(!st.empty() && nums[st.top()] > nums[i]) st.pop();
    ple[i] = st.empty() ? -1 : st.top();
    st.push(i); // 將目前index推入stack
}
```

> NLE (Next Less Element)

```cpp
vector<int> nums;
int sz = nums.size();
vector<int> nle(sz);
stack<int> st;
for(int i = nums.size() - 1; i >= 0; --i) {
    while(!st.empty() && nums[st.top()] > nums[i]) st.pop();
    nle[i] = st.empty() ? sz : st.top();
    st.push(i);
}
```

> 找山峰或者山谷

包含山谷隔斷與山峰隔斷

- 把最低位（高位）做隔斷等

## 例題

- [leetcode 474](https://leetcode.com/problems/ones-and-zeroes/?envType=daily-question&envId=2025-10-16)
