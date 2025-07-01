# 649. Dota2 Senate

>[!note]
>- `Medium`
>- `queue`

## 題目

給兩個陣營 `R`, `D` 

每個人都有可以將下一個人ban掉的權利

- 如果投票後只跟個陣營的票則贏

回傳誰贏

## 解題

略

```cpp
class Solution {
public:
    string predictPartyVictory(string senate) {
        queue<int> r, d;
        int n = senate.length();
        for (int i = 0; i < n; i++){
            if (senate[i] == 'R'){
                r.push(i);
            }
            else {
                d.push(i);
            }
        }
        while (!r.empty() && !d.empty()){
            if (r.front() < d.front()){
                r.push(n++);
            }
            else {
                d.push(n++);
            }
            r.pop(), d.pop();
        }
        return (r.empty()) ? ("Dire") : ("Radiant");
    }
};
```

- pyhton

```py
from collections import deque

class Solution:
    def predictPartyVictory(self, senate: str) -> str:
        r = deque()
        d = deque()
        n = len(senate)

        for i in range(n):
            if senate[i] == 'R':
                r.append(i)
            else:
                d.append(i)

        while r and d:
            r_idx = r.popleft()
            d_idx = d.popleft()

            if r_idx < d_idx:
                r.append(r_idx + n)
            else:
                d.append(d_idx + n)

        return "Radiant" if r else "Dire"
```

- c#

```cs
public class Solution {
    public string PredictPartyVictory(string senate) {
        Queue<char> s = new Queue<char>();
        int rCount = 0;
        int dCount = 0;
        
        for(int i =0; i<senate.Length; i++){
            s.Enqueue(senate[i]);
            if(senate[i] == 'R'){
                rCount++;
            }
            else{
                dCount++;
            }
        }
        if(dCount == 0){
            return "Radiant";
        }
        if(rCount == 0){
            return "Dire";
        }
        
        int skipr = 0;
        int skipd = 0;
        
        while(s.Count > 0){
            if(dCount == 0){
                return "Radiant";
            }
            if(rCount == 0){
                return "Dire";
            }

            if(s.Count == 1){
                if(s.Peek() == 'R'){
                    return "Radiant";
                }
                else{
                    return "Dire";
                }
            }

            char fc = s.Dequeue();

            if(skipd > 0 && fc == 'D'){
                skipd--;
                dCount--;
                continue;
            }
            if(skipr > 0 && fc == 'R'){
                skipr--;
                rCount--;
                continue;
            }
            if(fc == 'R'){
                skipd++;
            }
            else {
                skipr++;
            }

            s.Enqueue(fc);
        }

        if(s.Peek() == 'R'){
            return "Radiant";
        }
        else{
            return "Dire";
        }
    }
}
```
