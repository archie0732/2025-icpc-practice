# Tram

>[!note]
>- `800`
>- `math`

## 題目

一個列車，有`n`站

然後給`a` `b`:

-  a: 本站下車的人
-  b: 本站上車的人

求車廂需要的最小空間

## slove

too easy

### CPP CODE

```cpp

#include <bits/stdc++.h>

using namespace std;

int main(){

  int cur = 0, mx = 0;

  int n;
  cin>>n;

  while(n--){
    int up, down;
    cin >> down >> up;

    cur = cur + up - down;
    //cout << "cur: " << cur << endl;
    mx = max(mx, cur);
  }
  cout << mx << endl;
}
```