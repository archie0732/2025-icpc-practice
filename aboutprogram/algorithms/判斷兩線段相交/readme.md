# 判斷兩線段是否相交

>[!note]
>非常的重要，2025 TOPC 有出過


## 解說

在一般數學裡可以用`y = ax + b` 來判斷，不過在程式裡這樣很麻煩

所以這裡是用 `向量外積(叉積)`

>[!tip]
> 向量內積分為以下
>- < 0 左側
>- \> 0 右側
>- = 相交

給兩線段 ab cd

先判斷 c 於 ab 在左邊還是右邊 在判斷 d 在左還是右。即可判斷是否相交

- 注意還要判斷是否**三點共線**

>[!important]
> 因為這裡為判斷線段是否共線，所以須考慮範圍!!


- 向量公式

```py
a = (x1, y1) , b = (x2, y2)

v = x1y2 - y1x2 ## 內內外外 ， = 0 為不同方向
```

>[!important]
> 雖然不同方向的直線一定會相交，但是這裡考慮的是**線段**，所以只能用一點一線的方法!!


```py
# ab 為線段
# p 為要判斷的點
# ab(向量) * ap(向量) 
(b.x-a.x, b.y-a.y) * (p.x-a.x, p.y-a.y)

## 叉積公式 => 內內外外

(b.y - a.y) * (p.x - a.x) - (b.x - .x) * (p.y - a.y)

# > 0 右邊
# < 0 左邊
# = 0 在線上(但不一定在**線段**上)
```
## 考慮三點共線情況

剛剛有說過 `ab * ap = 0` 為三點共線，但是還要看 `p` 是否在 `ab` 的範圍裡(這樣就是在線段上)

- 判斷

```py
min(a.x, b.x) <= p.x <= max(a.x, b.x)
min(a.y, b.y) <= p.y <= max(a.y, b.y)
``` 

### EX CODE

```cpp
#include<bits/stdc++.h>

using namespace std;


struct Point{
  int x, y;
};


bool isIntersect(Point a, Point b, Point c, Point d){
  long long d1 = cross(p1, p2, q1);
    long long d2 = cross(p1, p2, q2);
    long long d3 = cross(q1, q2, p1);
    long long d4 = cross(q1, q2, p2);

    if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) &&
        ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) {
        return true; // 一般情況：兩線段相交
    }

    // 特殊情況：共線，判斷是否在線段上
    if (d1 == 0 && onSegment(p1, p2, q1)) return true;
    if (d2 == 0 && onSegment(p1, p2, q2)) return true;
    if (d3 == 0 && onSegment(q1, q2, p1)) return true;
    if (d4 == 0 && onSegment(q1, q2, p2)) return true;

    return false;
}

int cross(Point a, Point b, Point c){
  return (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
}

bool isOnSegment(Point a, Point b, Point c){
  return min(a.x, b.x) <= c.x && c.x <= max(a.x, b.x) && min(a.y, b.y) <= c.y && c.y <= max(a.y, b.y);
}


int main(){

  Point a = {0, 0};
  Point b = {1, 1};
  Point c = {2, 2};
  Point d = {3, 3};
  
  cout << isIntersect(a, b, c, d) << endl;


}
```
